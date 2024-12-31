use crate::domain::user::User;
use crate::services::mongo::get_client;
use anyhow::{anyhow, Result};
use bson::oid::ObjectId;
use futures::stream::TryStreamExt;
use mongodb::bson::doc;
use mongodb::Collection;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectModel {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub _id: Option<ObjectId>,
    pub user_id: String,
    pub name: String,
    pub completed: bool,
}

pub async fn get_projects_collection() -> Result<Collection<ProjectModel>> {
    let client = get_client().await?;
    let db = client.database("simplicity");
    let collection = db.collection::<ProjectModel>("projects");
    Ok(collection)
}

pub async fn get_all_projects_for_user(user: User) -> Result<Vec<ProjectModel>> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id };
    let mut cursor = collection.find(filter).await?;

    let mut projects = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        projects.push(result);
    }
    Ok(projects)
}

pub async fn get_all_projects_without_inbox(user: User) -> Result<Vec<ProjectModel>> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id, "name": { "$ne": "Inbox" } };
    let mut cursor = collection.find(filter).await?;

    let mut projects = Vec::new();
    while let Some(result) = cursor.try_next().await? {
        projects.push(result);
    }
    Ok(projects)
}

pub async fn get_project_by_id_for_user(user: &User, id: ObjectId) -> Result<ProjectModel> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "_id":id, "user_id": user.id.clone() };
    let project = collection.find_one(filter).await?;
    match project {
        Some(project) => Ok(project),
        None => anyhow::bail!("Project not found"),
    }
}

pub async fn add_project(user: User, project: ProjectModel) -> Result<ObjectId> {
    if user.id != project.user_id {
        anyhow::bail!("Project user_id does not match user id");
    }
    error!("Adding project");
    let collection = get_projects_collection().await?;
    error!("Got collection");
    let project = collection.insert_one(project).await?;
    error!("Inserted project");
    Ok(project.inserted_id.as_object_id().ok_or(anyhow!("Failed to get inserted id"))?)
}

pub async fn get_inbox_id_for_user(user: User) -> Result<ObjectId> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id.clone(), "name": "inbox" };
    let project = collection.find_one(filter).await?;
    let project_id = project.and_then(|p| p._id);
    match project_id {
        Some(project_id) => Ok(project_id),
        None => {
            let project = ProjectModel {
                _id: Some(ObjectId::new()),
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

pub async fn does_inbox_exist_for_user(user: &User) -> Result<bool> {
    let collection = get_projects_collection().await?;
    let filter = doc! { "user_id": user.id.clone(), "name": "inbox" };
    let project = collection.find_one(filter).await?;
    match project {
        Some(_) => Ok(true),
        None => Ok(false),
    }
}

pub async fn create_inbox_for_user(user: User) -> Result<ObjectId> {
    let project = ProjectModel {
        _id: Some(ObjectId::new()),
        user_id: user.id.clone(),
        name: "inbox".into(),
        completed: false,
    };
    add_project(user, project).await
}
