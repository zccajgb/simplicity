use crate::domain::user::User;
use crate::repos::tag_repo::{
    add_tag_for_user, get_all_tags_for_user, get_tag_by_id_for_user, Tag,
};
use crate::services::api_error::{ApiJsonResult, ResultExt};
use anyhow::{anyhow, Context, Result};
use bson::oid::ObjectId;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct TagDTO {
    _id: Option<String>,
    name: Option<String>,
    user_id: Option<String>,
}

impl TagDTO {
    fn from_tag(tag: Tag) -> Self {
        Self {
            _id: Some(tag._id.to_string()),
            name: Some(tag.name),
            user_id: Some(tag.user_id),
        }
    }

    fn to_tag(self: TagDTO) -> Result<Tag> {
        Ok(Tag {
            _id: ObjectId::parse_str(&self._id.ok_or(anyhow!("no id"))?)?,
            name: self.name.context("name is required")?,
            user_id: self.user_id.context("user_id is required")?,
        })
    }

    fn vec_from_tag_model(tags: Vec<Tag>) -> Vec<TagDTO> {
        tags.into_iter().map(TagDTO::from_tag).collect()
    }
}

fn map_and_return_tag(tag: Result<Tag>) -> ApiJsonResult<TagDTO> {
    tag.map(TagDTO::from_tag).map(Json).map_api_err()
}

fn map_and_return_tags(tags: Result<Vec<Tag>>) -> ApiJsonResult<Vec<TagDTO>> {
    let tags = tags.map_api_err()?;
    let tags_dto = TagDTO::vec_from_tag_model(tags);
    Ok(Json(tags_dto))
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![get_all_tags, get_tag_by_id, add_tag,]
}

#[get("/tags")]
pub async fn get_all_tags(user: User) -> ApiJsonResult<Vec<TagDTO>> {
    let tags = get_all_tags_for_user(user).await;
    map_and_return_tags(tags)
}

#[get("/tags/<id>")]
pub async fn get_tag_by_id(user: User, id: String) -> ApiJsonResult<TagDTO> {
    let id = ObjectId::parse_str(&id).map_api_err()?;
    let result = get_tag_by_id_for_user(&user, id).await;
    map_and_return_tag(result)
}

#[post("/tags", data = "<tag>")]
pub async fn add_tag(user: User, tag: Json<TagDTO>) -> ApiJsonResult<ObjectId> {
    let tag = tag.into_inner();
    let tag_model = tag.to_tag().map_api_err()?;
    let id: Result<_, _> = add_tag_for_user(user, tag_model).await;
    id.map(Json).map_api_err()
}
