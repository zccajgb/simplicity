use core::error;

use crate::domain::repeat::RepeatModel;
use crate::domain::user::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::{document, oid::ObjectId, to_bson, to_document, DateTime, Document};
use chrono::Utc;
use futures::stream::TryStreamExt;
use mongodb::bson;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
    pub completed: Option<DateTime>,
    pub ttl: String,
    pub project_id: Option<ObjectId>,
    pub tags: Vec<ObjectId>,
    pub date: Option<DateTime>,
    pub snooze: Option<DateTime>,
    pub repeat: RepeatModel,
    pub last_updated: Option<DateTime>,
}

impl TaskModel {
    fn to_update_doc(self) -> Document {
        let mut task_doc = to_document(&self).unwrap();
        task_doc.remove("_id"); // Remove the _id field
        doc! { "$set": task_doc }
    }
}

pub async fn get_tasks_collection() -> Result<Collection<TaskModel>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<TaskModel>("tasks");
    Ok(collection)
}

async fn get_tasks_without_snoozed(filter: bson::Document) -> Result<Vec<TaskModel>> {
    let mut filter = filter;
    let tomorrow = Utc::now().date_naive().succ_opt().unwrap();
    let tomorrow = tomorrow.and_hms_opt(0, 0, 0).unwrap();
    let tomorrow_millis = tomorrow.and_utc().timestamp_millis();

    let snooze = vec![
        doc! { "snooze": null },
        doc! { "snooze": { "$exists": false } },
        doc! { "snooze": { "$lte": bson::DateTime::from_millis(tomorrow_millis) } },
    ];
    filter.insert("$or", to_bson(&snooze).unwrap());

    error!("{:?}", filter);
    get_tasks_inner(filter).await
}

pub async fn get_tasks_inner(filter: bson::Document) -> Result<Vec<TaskModel>> {
    let collection = get_tasks_collection().await?;
    let mut cursor = collection.find(filter).await?;

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        tasks.push(result);
    }
    Ok(tasks)
}

pub async fn get_all_tasks_for_user(user: User) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.id };

    get_tasks_inner(filter).await
}

pub async fn get_today_tasks_for_user(user: User) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "today" } };

    get_tasks_without_snoozed(filter).await
}

pub async fn get_tomorrow_tasks_for_user(user: User) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "tomorrow" } };

    get_tasks_without_snoozed(filter).await
}

pub async fn get_later_tasks_for_user(user: User) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.id, "ttl": { "$eq": "later" } };

    get_tasks_without_snoozed(filter).await
}

pub async fn get_snoozed_tasks_for_user(user: User) -> Result<Vec<TaskModel>> {
    let tomorrow = Utc::now().date_naive().succ_opt().unwrap();
    let tomorrow = tomorrow.and_hms_opt(0, 0, 0).unwrap();
    let tomorrow_millis = tomorrow.and_utc().timestamp_millis();

    let filter = doc! { "user_id": user.id, "snooze": { "$ne": null, "$gte": bson::DateTime::from_millis(tomorrow_millis) } };
    get_tasks_inner(filter).await
}

pub async fn get_inbox_tasks_for_user(user: User, inbox_id: ObjectId) -> Result<Vec<TaskModel>> {
    let filter =
        doc! { "user_id": user.id, "project_id": { "$eq": inbox_id },  "snoozed": { "$eq": null } };

    get_tasks_without_snoozed(filter).await
}

pub async fn get_task_by_id_for_user(user: User, id: String) -> Result<TaskModel> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::parse_str(&id)?, "user_id": user.id };
    let task = collection.find_one(filter).await?;
    match task {
        Some(task) => Ok(task),
        None => anyhow::bail!("Task not found"),
    }
}

async fn get_task_by_id(id: ObjectId) -> Result<TaskModel> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": id};
    let task = collection.find_one(filter).await?;
    match task {
        Some(task) => Ok(task),
        None => anyhow::bail!("Task not found"),
    }
}

pub async fn get_tasks_by_project_for_user(
    user: User,
    project: ObjectId,
) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.id, "project_id": project,  "snoozed": { "$eq": null } };

    get_tasks_without_snoozed(filter).await
}

pub async fn get_tasks_by_tag_for_user(user: User, tag: ObjectId) -> Result<Vec<TaskModel>> {
    let filter =
        doc! { "user_id": user.id, "tags": { "$contains": tag }, "snoozed": { "$eq": null } };

    get_tasks_without_snoozed(filter).await
}

pub async fn add_task_for_user(user: User, task: TaskModel) -> Result<TaskModel> {
    if task.user_id != user.id {
        return Err(anyhow::anyhow!("Task user_id does not match user id"));
    }
    error!("Adding task: {:?}", task);
    let collection = get_tasks_collection().await?;
    let res = collection.insert_one(task).await?;
    error!("{:?}", res);
    let id = res
        .inserted_id
        .as_object_id()
        .ok_or_else(|| anyhow::anyhow!("No id found"))?;
    get_task_by_id(id).await
}

pub async fn update_task_for_user(user: User, id: ObjectId, task: TaskModel) -> Result<TaskModel> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": id, "user_id": &user.id };
    let res = collection.replace_one(filter, task).await?;
    if res.modified_count == 0 {
        warn!("Task not updated, likely nothing has changed");
    }
    get_task_by_id(id).await
}
