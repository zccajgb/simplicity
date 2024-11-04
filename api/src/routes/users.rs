use crate::domain::user::User;
use crate::services::api_error::{ApiResult, ResultExt};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![check_user_or_create_new]
}

#[get("/login")]
pub async fn check_user_or_create_new(user: User) -> ApiResult<()> {
    if (User::does_user_exist(&user).await).is_ok() {
        return Ok(());
    }
    User::create_user(user).await.map(|_| ()).map_api_err()
}
