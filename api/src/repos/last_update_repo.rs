use crate::services::mongo::get_client;

use super::task_repo::{get_tasks_collection, get_tasks_inner, update_task_for_user, TaskModel};
use anyhow::Result;
use bson::oid::ObjectId;
use chrono::Utc;
use mongodb::bson::{doc, DateTime};
use mongodb::{bson, Collection};
use rocket::time::Date;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct LastUpdateModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<bson::oid::ObjectId>,
    pub last_update: Option<DateTime>,
}

pub async fn get_last_update_collection() -> Result<Collection<LastUpdateModel>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<LastUpdateModel>("daily_update");
    Ok(collection)
}

pub async fn get_last_update() -> Result<LastUpdateModel> {
    let collection = get_last_update_collection().await?;
    let cursor = collection.find_one(doc! {}).await?;

    cursor.ok_or_else(|| anyhow::anyhow!("could not get last update"))
}

pub async fn set_last_update() -> Result<()> {
    let now = Utc::now().timestamp_millis();
    let collection = get_last_update_collection().await?;
    let update_doc = doc! { "$set": { "last_update": DateTime::from_millis(now) } };
    let update_result = collection.update_one(doc! {}, update_doc).await?;
    if update_result.modified_count == 0 {
        error!("could not update last update time");
    }
    Ok(())
}
