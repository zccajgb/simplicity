use rand::Rng;
use rocket::http::CookieJar;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use crate::repos::couchdb_repo;
use crate::repos::project_repo;
use crate::repos::users_repo;
use crate::services::api_error::ApiJsonResult;
use crate::services::api_error::{ApiResult, ResultExt};
use crate::services::auth::create_jwt;
use crate::services::auth::{generate_session_cookie, get_user_from_auth_code};
pub fn get_routes() -> Vec<rocket::Route> {
    routes![public, login_with_auth_code, logout, ping, get_jwt_for_user]
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
    pub inbox_id: Option<String>,
}

impl User {
    pub fn from_user_model(model: users_repo::UserModel) -> Self {
        Self {
            user_id: model.user_id,
            image_url: model.image_url,
            token_expiry: Some(model.token_expiry),
            inbox_id: model.inbox_id.map(|id| id.to_string()),
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

    let db_user = users_repo::find_user_by_user_id(&token_user.user_id)
        .await
        .map_api_err()?;
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
        let user = User::from_user_model(user);
        return Ok(Json(user));
    }

    let db_user = users_repo::add_user(token_user).await.map_api_err()?;
    info!("added user to repo");
    let user = User::from_user_model(db_user);
    couchdb_repo::create_db_for_user(user.clone())
        .await
        .map_api_err()?;
    info!("created db for user");
    let inbox_id = couchdb_repo::create_inbox_for_user(user.clone())
        .await
        .map_api_err()?;
    info!("created inbox for user");
    users_repo::set_inbox_id_for_user(&user.user_id, &inbox_id)
        .await
        .map_api_err()?;
    info!("set inbox id for user");
    Ok(Json(user))
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

#[get("/ping")]
pub async fn ping(_user: User) -> &'static str {
    "pong"
}

#[get("/jwt")]
pub async fn get_jwt_for_user(user: User) -> ApiResult<String> {
    let jwt = create_jwt(&user).map_api_err()?;
    Ok(jwt)
}
