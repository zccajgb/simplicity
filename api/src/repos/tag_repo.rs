use crate::domain::user::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
}

pub async fn get_tags_collection() -> Result<Collection<Tag>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<Tag>("tags");
    Ok(collection)
}

async fn get_tags_inner(filter: bson::Document) -> Result<Vec<Tag>> {
    let collection = get_tags_collection().await?;
    let mut cursor = collection.find(filter).await?;

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        tasks.push(result);
    }
    Ok(tasks)
}

pub async fn get_all_tags_for_user(user: User) -> Result<Vec<Tag>> {
    let filter = doc! { "user_id": user.id };

    get_tags_inner(filter).await
}

pub async fn get_tag_by_id_for_user(user: &User, id: ObjectId) -> Result<Tag> {
    let collection = get_tags_collection().await?;
    let filter = doc! { "_id": id, "user_id": user.id.clone() };
    let tag = collection.find_one(filter).await?;
    match tag {
        Some(tag) => Ok(tag),
        None => anyhow::bail!("Tag not found"),
    }
}

pub async fn add_tag_for_user(user: User, tag: Tag) -> Result<ObjectId> {
    if tag.user_id != user.id {
        anyhow::bail!("Tag user_id does not match user id");
    }
    let collection = get_tags_collection().await?;
    let res = collection.insert_one(tag).await?;
    Ok(res.inserted_id.as_object_id().unwrap())
}
