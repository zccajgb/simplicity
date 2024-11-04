use crate::repos::tag_repo::{
    add_tag_for_user, get_all_tags_for_user, get_tag_by_id_for_user, Tag,
};
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_all_projects,
        get_all_projects_without_inbox,
        get_project_by_id,
        add_project,
    ]
}

#[get("/tags")]
pub async fn get_all_tags(user: User) -> ApiJsonError<Vec<Tag>> {
    let tags = get_all_tags_for_user(user).await;
    tags.map(|tags| Json(tags)).map_api_err()
}

#[get("/tags/<id>")]
pub async fn get_tag_by_id(user: User, id: String) -> ApiJsonError<Tag> {
    let tag: Result<_, _> = get_tag_by_id_for_user(user, id).await;
    tags.map(|tags| Json(tags)).map_api_err()
}

#[post("/tags", data = "<tag>")]
pub async fn add_tag(user: User, tag: Json<Tag>) -> ApiJsonError<Tag> {
    let tag: Result<_, _> = add_tag_for_user(user, tag.into_inner()).await;
    tags.map(|tags| Json(tags)).map_api_err()
}
