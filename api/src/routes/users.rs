use crate::domain::user::User;
use crate::services::api_error::{ApiResult, ResultExt};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![check_user_or_create_new, public]
}

#[get("/login")]
pub async fn check_user_or_create_new(user: User) -> ApiResult<()> {
    error!("hello");
    if (User::does_user_exist(&user).await).is_ok() {
        error!("user exists");
        return Ok(());
    }
    User::create_user(user).await.map(|_| ()).map_api_err()
}

#[get("/public")]
pub async fn public() -> &'static str {
    "Hello, world!"
}