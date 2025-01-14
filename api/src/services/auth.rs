use crate::repos::users_repo::{self, UserModel};
use anyhow::Result;
use google_jwt_signin::Client;
use log::error;
use oauth2::{
    basic::BasicClient, reqwest::async_http_client, AuthUrl, AuthorizationCode, ClientId,
    ClientSecret, RedirectUrl, RefreshToken, TokenResponse, TokenUrl,
};
use rand::Rng;
use rocket::{
    http::{Cookie, SameSite},
    time::{Duration, OffsetDateTime},
};

pub fn validate_token(token: &str) -> Result<()> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = Client::new(&client_id);
    let token_info = client.verify_id_token(token);
    let Ok(_) = token_info else {
        return Err(anyhow::anyhow!("Token not valid"));
    };

    Ok(())
}

pub fn validate_token_and_get_user(token: &str, refresh_token: &Option<&str>) -> Result<UserModel> {
    let client_id = std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set");
    let client = Client::new(&client_id);
    let token_info = client.verify_id_token(token).map_err(|e| {
        error!("Error verifying token: {:?}", e);
        anyhow::anyhow!("Token not valid")
    })?;
    let user_id = token_info.claims.subject;
    let user_image = token_info.payload.picture;
    let token_expiry = token_info.claims.expires_at;

    let user = UserModel {
        _id: None,
        user_id: user_id.to_string(),
        access_token: token.to_string(),
        refresh_token: refresh_token.map(|s| s.to_string()),
        image_url: user_image,
        token_expiry: token_expiry as i64,
        session_token: vec![],
    };
    Ok(user)
}

pub fn generate_session_cookie(token: String) -> Result<Cookie<'static>> {
    let expiration = OffsetDateTime::now_utc() + Duration::weeks(4);
    let cookie = Cookie::build(("session_token", token))
        .path("/")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .expires(expiration);

    Ok(cookie.into())
}

pub async fn get_user_from_auth_code(auth_code: &str) -> Result<UserModel> {
    let client = create_auth_client();
    let token = client
        .exchange_code(AuthorizationCode::new(auth_code.to_string()))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            error!("Error exchanging code: {:?}", e);
            anyhow::anyhow!("Error exchanging code")
        })?;
    let access_token = token.access_token().secret().as_str();
    let refresh_token = token.refresh_token().map(|t| t.secret().as_str());
    let expires_in = token.expires_in().map(|x| x.as_secs()).unwrap_or(0);
    let token_expiry = expires_in as i64 + chrono::Utc::now().timestamp();
    info!("got tokens and expiry");
    let user: UserModel = get_user_from_token(access_token, &refresh_token, token_expiry).await?;
    info!("got user from token");

    Ok(user)
}

pub async fn get_user_from_token(
    access_token: &str,
    refresh_token: &Option<&str>,
    token_expiry: i64,
) -> Result<UserModel> {
    let user_info_url = "https://www.googleapis.com/oauth2/v3/userinfo";
    let client = reqwest::Client::new();
    let response = client
        .get(user_info_url)
        .bearer_auth(access_token)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(anyhow::anyhow!("Failed to get user info"));
    }

    let user_info: serde_json::Value = response.json().await?;
    let image_url = user_info["picture"].as_str().map(|i| i.to_string());
    let user_id = user_info["sub"]
        .as_str()
        .ok_or(anyhow::anyhow!("No sub field in user info"))?
        .to_string();
    let user = UserModel {
        _id: None,
        user_id: user_id.to_string(),
        access_token: access_token.to_string(),
        refresh_token: refresh_token.map(|s| s.to_string()),
        image_url,
        token_expiry,
        session_token: vec![],
    };
    Ok(user)
}

pub struct ApiKey(pub String);

impl ApiKey {
    pub fn validate(&self) -> Result<()> {
        let api_key = std::env::var("API_KEY")?;
        if self.0 == api_key {
            Ok(())
        } else {
            Err(anyhow::anyhow!("Invalid API key"))
        }
    }
}

fn create_auth_client() -> BasicClient {
    let google_client_id =
        ClientId::new(std::env::var("GOOGLE_CLIENT_ID").expect("GOOGLE_CLIENT_ID must be set"));
    let google_client_secret = ClientSecret::new(
        std::env::var("GOOGLE_CLIENT_SECRET").expect("GOOGLE_CLIENT_SECRET must be set"),
    );
    let auth_url =
        AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap();
    let token_url = TokenUrl::new("https://oauth2.googleapis.com/token".to_string()).unwrap();
    let redirect_url = std::env::var("REDIRECT_URL").expect("REDIRECT_URL must be set");
    BasicClient::new(
        google_client_id,
        Some(google_client_secret),
        auth_url,
        Some(token_url),
    )
    .set_redirect_uri(RedirectUrl::new(redirect_url).unwrap())
}

pub async fn get_user_from_session_token(session_token: &str) -> Result<UserModel> {
    let user = users_repo::find_user_by_session_token(session_token).await;
    let mut user = user.ok_or(anyhow::anyhow!("User not found"))?;
    if (user.token_expiry - chrono::Utc::now().timestamp()) < 0 {
        warn!("Token expired, refreshing");
        user = refresh_token(user, session_token.to_string())
            .await
            .inspect_err(|e| error!("Error refreshing token: {:?}", e))?;
    }
    Ok(user)
}

async fn refresh_token(user: UserModel, session_token: String) -> Result<UserModel> {
    let client = create_auth_client();
    let refresh_token = user
        .refresh_token
        .ok_or(anyhow::anyhow!("No refresh token"))?;
    let token = client
        .exchange_refresh_token(&RefreshToken::new(refresh_token))
        .request_async(async_http_client)
        .await
        .map_err(|e| {
            error!("Error refreshing token: {:?}", e);
            anyhow::anyhow!("Error refreshing token")
        })?;

    let access_token = token.access_token().secret().as_str();
    let refresh_token = token.refresh_token().map(|t| t.secret().as_str());
    let _token_user = validate_token_and_get_user(access_token, &refresh_token)?;
    let user = users_repo::update_tokens_for_user(
        &user.user_id,
        access_token.to_string(),
        refresh_token.map(|s| s.to_string()),
        user.token_expiry,
        session_token,
    )
    .await?;
    Ok(user)
}
