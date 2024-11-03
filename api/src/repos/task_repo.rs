use crate::services::auth::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    id: String,
    user_id: String,
    name: String,
    completed: bool,
}

pub async fn get_tasks_collection() -> Result<Collection<Task>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<Task>("tasks");
    Ok(collection)
}

pub async fn get_all_tasks_for_user(user: User) -> Result<Vec<Task>> {
    let collection = get_tasks_collection().await?;
    println!("got collection");
    let filter = doc! { "user_id": user.id };
    let mut cursor = collection.find(filter).await?;
    println!("got cursor");

    let mut tasks = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        println!("got task");

        tasks.push(result);
    }

    return Ok(tasks);
}
