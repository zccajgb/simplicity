use crate::services::auth::User;
use crate::services::mongo::get_client;
use anyhow::{anyhow, Result};
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Project {
    pub _id: ObjectId,
    pub user_id: String,
    pub name: String,
    pub completed: bool,
}

pub async fn get_projects_collection() -> Result<Collection<Project>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<Project>("projects");
    Ok(collection)
}

pub async fn get_all_projects_for_user(user: User) -> Result<Vec<Project>> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id };
    let mut cursor = collection.find(filter).await?;

    let mut projects = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        projects.push(result);
    }
    Ok(projects)
}

pub async fn get_all_projects_without_inbox(user: User) -> Result<Vec<Project>> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id, "name": { "$ne": "Inbox" } };
    let mut cursor = collection.find(filter).await?;

    let mut projects = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        projects.push(result);
    }
    Ok(projects)
}

pub async fn get_project_by_id_for_user(user: &User, id: ObjectId) -> Result<Project> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "_id":id, "user_id": user.id.clone() };
    let project = collection.find_one(filter).await?;
    match project {
        Some(project) => Ok(project),
        None => anyhow::bail!("Project not found"),
    }
}

pub async fn add_project(user: User, project: Project) -> Result<ObjectId> {
    if user.id != project.user_id {
        anyhow::bail!("Project user_id does not match user id");
    }
    let collection = get_projects_collection().await?;
    let project = collection.insert_one(project).await?;
    Ok(project.inserted_id.as_object_id().unwrap())
}

pub async fn get_inbox_id_for_user(user: User) -> Result<ObjectId> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id.clone(), "name": "inbox" };
    let project = collection.find_one(filter).await?;
    match project {
        Some(project) => Ok(project._id),
        None => {
            let project = Project {
                _id: ObjectId::new(),
                user_id: user.id.clone(),
                name: "inbox".into(),
                completed: false,
            };
            add_project(user, project)
                .await
                .map_err(|e| anyhow!("Failed to create inbox: {}", e))
        }
    }
}

pub async fn does_inbox_exist_for_user(user: User) -> Result<bool> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id.clone(), "name": "inbox" };
    let project = collection.find_one(filter).await?;
    match project {
        Some(_) => Ok(true),
        None(_) => Ok(false),
    }
}

pub async fn create_inbox_for_user(user: User) -> Result<ObjectId> {
    let project = Project {
        _id: ObjectId::new(),
        user_id: user.id.clone(),
        name: "inbox".into(),
        completed: false,
    };
    add_project(user, project).await
}