use crate::services::auth::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::oid::ObjectId;
use futures::stream::{Filter, TryStreamExt};
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
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
    let mut cursor = collection.find(filter).await?;

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        tasks.push(result);
    }
    return Ok(tasks);
}

pub async fn get_all_tasks_for_user(user: User) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id };

    return get_tasks_inner(filter).await;
}

pub async fn get_today_tasks_for_user(user: User) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "today" } };

    return get_tasks_inner(filter).await;
}

pub async fn get_tomorrow_tasks_for_user(user: User) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "tomorrow" } };

    return get_tasks_inner(filter).await;
}

pub async fn get_inbox_tasks_for_user(user: User, inbox_id: ObjectId) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id, "project_id": { "$eq": inbox_id } };

    return get_tasks_inner(filter).await;
}

pub async fn get_task_by_id_for_user(user: User, id: String) -> Result<Task> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::with_string(&id)?, "user_id": user.id };
    let task = collection.find_one(filter).await?;
    return match task {
        Some(task) => Ok(task),
        None => Ok(Task::default()),
    };
}

pub async fn get_tasks_by_project_for_user(user: User, project: ObjectId) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id, "project_id": project };

    return get_tasks_inner(filter).await;
}

pub async fn get_tasks_by_tag_for_user(user: User, tag: ObjectId) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id, "tags": { "$contains": tag } };

    return get_tasks_inner(filter).await;
}

pub async fn add_task_for_user(user: User, task: Task) -> Result<Task> {
    let collection = get_tasks_collection().await?;
    let task = collection.insert_one(task).await?;
    return match task {
        Ok(task) => Ok(task),
        Err(e) => Ok(Task::default()),
    };
}

pub async fn update_task_for_user(user: User, id: ObjectId, task: Task) -> Result<Task> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": id, "user_id": user.id };
    let task = collection.update_one(filter, task).await?;
    return match task {
        Some(task) => Ok(task),
        None => Ok(Task::default()),
    };
}
