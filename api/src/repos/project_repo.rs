use crate::services::auth::User;
use crate::services::mongo::get_client;
use anyhow::Result;
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    _id: ObjectId,
    user_id: String,
    name: String,
    completed: bool,
}

pub async fn get_tasks_collection() -> Result<Collection<Project>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<Project>("projects");
    Ok(collection)
}

pub async fn get_all_projects_for_user(user: User) -> Result<Vec<Project>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id };
    let mut cursor = collection.find(filter).await?;

    let mut projects = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        projects.push(result);
    }
    return Ok(projects);
}

pub async fn get_all_projects_without_inbox(user: User) -> Result<Vec<Project>> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "user_id": user.id, "name": { "$ne": "Inbox" } };
    let mut cursor = collection.find(filter).await?;

    let mut projects = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        projects.push(result);
    }
    return Ok(projects);
}

pub async fn get_project_by_id_for_user(user: User, id: String) -> Result<Project> {
    let collection = get_tasks_collection().await?;
    let filter = doc! { "_id": ObjectId::with_string(&id)?, "user_id": user.id };
    let project = collection.find_one(filter).await?;
    return match project {
        Some(project) => Ok(project),
        None => Ok(Project::default()),
    };
}

pub async fn add_project(user: User, project: Project) -> Result<Project> {
    let collection = get_tasks_collection().await?;
    let project = collection.insert_one(project).await?;
    return match project {
        Ok(project) => Ok(project),
        Err(e) => Ok(Project::default()),
    };
}
