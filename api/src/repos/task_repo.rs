use crate::domain::user::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Task {
    pub _id: ObjectId,
    pub user_id: String,
    pub name: String,
    pub completed: bool,
    pub ttl: String,
    pub project_id: ObjectId,
    pub tags: Vec<ObjectId>,
    pub date: String,
    pub snooze: String,
}

pub async fn get_tasks_collection() -> Result<Collection<Task>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<Task>("tasks");
    Ok(collection)
}

async fn get_tasks_inner(filter: bson::Document) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let mut cursor = collection.find(filter).await?;

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        tasks.push(result);
    }
    Ok(tasks)
}

pub async fn get_all_tasks_for_user(user: User) -> Result<Vec<Task>> {
    let filter = doc! { "user_id": user.id };

    get_tasks_inner(filter).await
}

pub async fn get_today_tasks_for_user(user: User) -> Result<Vec<Task>> {
    // error!("Getting today tasks for user: {:?}", user.id);
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "today" } };

    get_tasks_inner(filter).await
}

pub async fn get_tomorrow_tasks_for_user(user: User) -> Result<Vec<Task>> {
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "tomorrow" } };

    get_tasks_inner(filter).await
}

pub async fn get_inbox_tasks_for_user(user: User, inbox_id: ObjectId) -> Result<Vec<Task>> {
    let filter = doc! { "user_id": user.id, "project_id": { "$eq": inbox_id } };

    get_tasks_inner(filter).await
}

pub async fn get_task_by_id_for_user(user: User, id: String) -> Result<Task> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::parse_str(&id)?, "user_id": user.id };
    let task = collection.find_one(filter).await?;
    match task {
        Some(task) => Ok(task),
        None => anyhow::bail!("Task not found"),
    }
}

pub async fn get_tasks_by_project_for_user(user: User, project: ObjectId) -> Result<Vec<Task>> {
    let filter = doc! { "user_id": user.id, "project_id": project };

    get_tasks_inner(filter).await
}

pub async fn get_tasks_by_tag_for_user(user: User, tag: ObjectId) -> Result<Vec<Task>> {
    let filter = doc! { "user_id": user.id, "tags": { "$contains": tag } };

    get_tasks_inner(filter).await
}

pub async fn add_task_for_user(user: User, task: Task) -> Result<ObjectId> {
    if task.user_id != user.id {
        anyhow::bail!("Task user_id does not match user id");
    }
    let collection = get_tasks_collection().await?;
    let res = collection.insert_one(task).await?;
    res.inserted_id
        .as_object_id()
        .ok_or_else(|| anyhow::anyhow!("No id found"))
}

pub async fn update_task_for_user(user: User, id: ObjectId, task: Task) -> Result<ObjectId> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": id, "user_id": user.id };
    let update_task = bson::to_document(&task)?;
    let update = doc! { "$set": update_task };
    let res = collection.update_one(filter, update).await?;
    res.upserted_id
        .ok_or_else(|| anyhow::anyhow!("No id found"))?
        .as_object_id()
        .ok_or_else(|| anyhow::anyhow!("No id found"))
}
