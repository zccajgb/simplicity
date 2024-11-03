use crate::services::auth::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::oid::ObjectId;
use futures::stream::{Filter, TryStreamExt};
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Tag {
    pub _id: ObjectId,
    pub name: String,
}

pub async fn get_tasks_collection() -> Result<Collection<Tag>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<Tag>("tags");
    Ok(collection)
}

async fn get_tags_inner(filter: bson::Document) -> Result<Vec<Tag>> {
    let mut cursor = collection.find(filter).await?;

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        tasks.push(result);
    }
    return Ok(tasks);
}

pub async fn get_all_tags_for_user(user: User) -> Result<Vec<Tag>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id };

    return get_tags_inner(filter).await;
}

pub async fn get_tag_by_id_for_user(user: User, id: String) -> Result<Tag> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::with_string(&id)?, "user_id": user.id };
    let tag = collection.find_one(filter).await?;
    return match tag {
        Some(tag) => Ok(tag),
        None => Ok(Tag::default()),
    };
}

pub async fn add_tag_for_user(user: User, tag: Tag) -> Result<Tag> {
    let collection = get_tasks_collection().await?;
    let tag = collection.insert_one(tag).await?;
    return Ok(tag);
}
