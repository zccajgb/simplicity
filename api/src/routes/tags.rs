use crate::domain::user::User;
use crate::repos::tag_repo::{
    add_tag_for_user, get_all_tags_for_user, get_tag_by_id_for_user, Tag,
};
use crate::services::api_error::{ApiJsonResult, ResultExt};
use bson::oid::ObjectId;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_all_tags, get_tag_by_id, add_tag,]
}

#[get("/tags")]
pub async fn get_all_tags(user: User) -> ApiJsonResult<Vec<Tag>> {
    let tags = get_all_tags_for_user(user).await;
    tags.map(Json).map_api_err()
}

#[get("/tags/<id>")]
pub async fn get_tag_by_id(user: User, id: String) -> ApiJsonResult<Tag> {
    let id = ObjectId::parse_str(&id).map_api_err()?;
    let result = get_tag_by_id_for_user(&user, id).await;
    result.map(Json).map_api_err()
}

#[post("/tags", data = "<tag>")]
pub async fn add_tag(user: User, tag: Json<Tag>) -> ApiJsonResult<ObjectId> {
    let id: Result<_, _> = add_tag_for_user(user, tag.into_inner()).await;
    id.map(Json).map_api_err()
}
