use crate::services::api_error::{ApiJsonResult, ResultExt};
use crate::{
    repos::tag_repo::{add_tag_for_user, get_all_tags_for_user, get_tag_by_id_for_user, Tag},
    services::auth::User,
};
use bson::oid::ObjectId;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_all_tags, get_tag_by_id, add_tag,]
}

#[get("/login")]
pub async fn check_user_or_create_new(user: User) -> ApiResult<()> {
    if let Ok(existsResult) = User::does_user_exist(&user) {
      return Ok();
    }
    User::create_user(&user).await.map(|_| ()).map_api_err()
   
}