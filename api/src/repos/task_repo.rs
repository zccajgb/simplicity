use crate::domain::subtask::SubTaskModel;
use crate::services::mongo::get_client;
use crate::{domain::repeat::RepeatModel, routes::users::User};
use anyhow::Result;
use bson::{oid::ObjectId, to_bson, to_document, DateTime, Document};
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
    pub subtasks: Vec<SubTaskModel>,
    pub comments: String,
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

pub async fn move_all_tasks_for_project_to_inbox(user: User, project_id: &ObjectId) -> Result<()> {
    let inbox_id = user
        .inbox_id
        .ok_or_else(|| anyhow::anyhow!("No inbox found"))?;
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.user_id, "project_id": project_id };
    let update = doc! { "$set": { "project_id": ObjectId::parse_str(inbox_id)? } };
    collection.update_many(filter, update).await?;
    Ok(())
}
