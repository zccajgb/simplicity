use core::error;

use crate::services::mongo::get_client;
use crate::{domain::repeat::RepeatModel, routes::users::User};
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
    pub order: i64,
}

impl TaskModel {
    fn to_update_doc(self) -> Document {
        let mut task_doc = to_document(&self).expect("Failed to convert to doc");
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

async fn get_tasks_without_snoozed(
    filter: bson::Document,
    completed: bool,
) -> Result<Vec<TaskModel>> {
    let mut filter = filter;
    let tomorrow = Utc::now()
        .date_naive()
        .succ_opt()
        .expect("could not get tomorrow");
    let tomorrow = tomorrow
        .and_hms_opt(0, 0, 0)
        .expect("could not get tomorrow");
    let tomorrow_millis = tomorrow.and_utc().timestamp_millis();

    let snooze = vec![
        doc! { "snooze": null },
        doc! { "snooze": { "$exists": false } },
        doc! { "snooze": { "$lte": bson::DateTime::from_millis(tomorrow_millis) } },
    ];

    filter.insert("$or", to_bson(&snooze).expect("Failed to convert to bson"));
    get_tasks_inner(filter, completed).await
}

pub fn add_completed_filter(mut filter: bson::Document) -> bson::Document {
    let today = Utc::now()
        .date_naive()
        .and_hms_opt(0, 0, 0)
        .expect("could not get today")
        .and_utc();
    let completed = vec![
        doc! { "completed": null },
        doc! { "completed": { "$exists": false } },
        doc! { "completed": { "$gte": bson::DateTime::from_chrono(today) } },
    ];
    let doc = vec![doc! { "$or": completed }];
    filter.insert("$and", to_bson(&doc).expect("Failed to convert to bson"));
    filter
}

pub async fn get_tasks_inner(filter: bson::Document, completed: bool) -> Result<Vec<TaskModel>> {
    let mut filter = filter;
    if !completed {
        filter = add_completed_filter(filter);
    }
    let collection = get_tasks_collection().await?;
    let mut cursor = collection.find(filter).await?;

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        tasks.push(result);
    }
    Ok(tasks)
}

pub async fn get_all_tasks_for_user(user: User, completed: bool) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id };
    get_tasks_inner(filter, completed).await
}

pub async fn get_today_tasks_for_user(user: User, completed: bool) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id, "ttl": { "$eq": "today" } };
    get_tasks_without_snoozed(filter, completed).await
}

pub async fn get_tomorrow_tasks_for_user(user: User, completed: bool) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id, "ttl": { "$eq": "tomorrow" } };
    get_tasks_without_snoozed(filter, completed).await
}

pub async fn get_later_tasks_for_user(user: User, completed: bool) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id, "ttl": { "$eq": "later" } };
    get_tasks_without_snoozed(filter, completed).await
}

pub async fn get_snoozed_tasks_for_user(user: User, completed: bool) -> Result<Vec<TaskModel>> {
    let tomorrow = Utc::now()
        .date_naive()
        .succ_opt()
        .expect("could not get tomorrow");
    let tomorrow = tomorrow
        .and_hms_opt(0, 0, 0)
        .expect("could not get tomorrow");
    let tomorrow_millis = tomorrow.and_utc().timestamp_millis();

    let filter = doc! { "user_id": user.user_id, "snooze": { "$ne": null, "$gte": bson::DateTime::from_millis(tomorrow_millis) } };
    get_tasks_inner(filter, completed).await
}

pub async fn get_inbox_tasks_for_user(
    user: User,
    inbox_id: ObjectId,
    completed: bool,
) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id, "project_id": { "$eq": inbox_id }};
    get_tasks_without_snoozed(filter, completed).await
}

pub async fn get_task_by_id_for_user(user: User, id: String) -> Result<TaskModel> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::parse_str(&id)?, "user_id": user.user_id };
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
    completed: bool,
) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id, "project_id": project};
    get_tasks_without_snoozed(filter, completed).await
}

pub async fn get_tasks_by_tag_for_user(
    user: User,
    tag: ObjectId,
    completed: bool,
) -> Result<Vec<TaskModel>> {
    let filter = doc! { "user_id": user.user_id, "tags": { "$contains": tag }};
    get_tasks_without_snoozed(filter, completed).await
}

pub async fn add_task_for_user(user: User, task: TaskModel) -> Result<TaskModel> {
    if task.user_id != user.user_id {
        return Err(anyhow::anyhow!("Task user_id does not match user id"));
    }
    let collection = get_tasks_collection().await?;
    let res = collection.insert_one(task).await?;
    let id = res
        .inserted_id
        .as_object_id()
        .ok_or_else(|| anyhow::anyhow!("No id found"))?;
    get_task_by_id(id).await
}

pub async fn update_task_for_user(user: User, id: ObjectId, task: TaskModel) -> Result<TaskModel> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": id, "user_id": &user.user_id };
    let res = collection.replace_one(filter, task).await?;
    if res.modified_count == 0 {
        warn!("Task not updated, likely nothing has changed");
    }
    get_task_by_id(id).await
}

pub async fn delete_task_for_user(user: User, id: String) -> Result<()> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::parse_str(id)?, "user_id": &user.user_id };
    let res = collection.delete_one(filter).await?;
    if res.deleted_count == 0 {
        warn!("Task not updated, likely nothing has changed");
    }
    Ok(())
}
