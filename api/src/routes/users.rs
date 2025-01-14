use rand::Rng;
use rocket::data::FromData;
use rocket::http::Cookie;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::repos::project_repo;
use crate::repos::users_repo;
use crate::services::api_error::ApiJsonResult;
use crate::services::api_error::{ApiResult, ResultExt};
use crate::services::auth::{generate_session_cookie, get_user_from_auth_code};
pub fn get_routes() -> Vec<rocket::Route> {
    routes![public, login_with_auth_code, logout]
}

#[derive(Serialize, Deserialize)]
pub struct AuthCode {
    code: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    pub user_id: String,
    pub image_url: Option<String>,
    pub token_expiry: Option<i64>,
}

impl User {
    pub fn from_user_model(model: users_repo::UserModel) -> Self {
        Self {
            user_id: model.user_id.clone(),
            image_url: model.image_url.clone(),
            token_expiry: Some(model.token_expiry),
        }
    }
}

#[post("/login/authCode", data = "<auth_code>")]
pub async fn login_with_auth_code(
    auth_code: Json<AuthCode>,
    jar: &CookieJar<'_>,
) -> ApiJsonResult<User> {
    let auth_code = auth_code.into_inner().code;
    let token_user = get_user_from_auth_code(&auth_code).await.map_api_err()?;
    let session_token = rand::thread_rng().gen::<u128>().to_string();

    let cookie = generate_session_cookie(session_token.clone()).map_api_err()?;
    info!("creating cookie");

    let db_user = users_repo::find_user_by_user_id(&token_user.user_id).await;

    if db_user.is_some() {
        let user = users_repo::update_tokens_for_user(
            &token_user.user_id,
            token_user.access_token,
            token_user.refresh_token,
            token_user.token_expiry,
            session_token,
        )
        .await
        .map_api_err()?;
        jar.add(cookie);
        return Ok(Json(User::from_user_model(user)));
    }

    let db_user = users_repo::add_user(token_user).await.map_api_err()?;
    let _user_inbox = project_repo::create_inbox_for_user(User::from_user_model(db_user.clone()))
        .await
        .map_api_err()?;

    Ok(Json(User::from_user_model(db_user)))
}

#[get("/logout")]
pub async fn logout(user: User, jar: &CookieJar<'_>) -> ApiResult<()> {
    let session_token = jar
        .get("session_token")
        .map(|c| c.value().to_string())
        .ok_or(anyhow::anyhow!("No session token found"))
        .map_api_err()?;
    let res = users_repo::logout(&user.user_id, &session_token).await;
    res.map_api_err()?;
    Ok(())
}

#[get("/public")]
pub async fn public() -> &'static str {
    "Hello, world!"
}
