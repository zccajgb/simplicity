use crate::repos::project_repo::{self, get_inbox_id_for_user};
use crate::repos::project_repo::{
    get_all_projects_for_user, get_project_by_id_for_user, ProjectModel,
};
use crate::services::api_error::{ApiError, ApiJsonResult, ResultExt};
use anyhow::{anyhow, Context, Result};
use bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

use super::users::User;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct ProjectDTO {
    id: Option<String>,
    name: Option<String>,
    user_id: Option<String>,
    completed: Option<bool>,
}

impl ProjectDTO {
    fn from_project(project: ProjectModel) -> Self {
        Self {
            id: project._id.map(|s| s.to_string()),
            name: Some(project.name),
            user_id: Some(project.user_id),
            completed: Some(project.completed),
        }
    }

    fn to_project(self: ProjectDTO) -> Result<ProjectModel> {
        Ok(ProjectModel {
            _id: self.id.and_then(|s| ObjectId::parse_str(s).ok()),
            name: self.name.context("name is required")?,
            user_id: self.user_id.context("user_id is required")?,
            completed: self.completed.unwrap_or(false),
        })
    }

    fn vec_from_project_model(projects: Vec<ProjectModel>) -> Vec<ProjectDTO> {
        projects.into_iter().map(ProjectDTO::from_project).collect()
    }
}

fn map_and_return_project(project: Result<ProjectModel>) -> ApiJsonResult<ProjectDTO> {
    project
        .map(ProjectDTO::from_project)
        .map(Json)
        .map_api_err()
}

fn map_and_return_projects(projects: Result<Vec<ProjectModel>>) -> ApiJsonResult<Vec<ProjectDTO>> {
    let projects = projects.map_api_err()?;
    let projects_dto = ProjectDTO::vec_from_project_model(projects);
    Ok(Json(projects_dto))
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_all_projects,
        get_all_projects_without_inbox,
        get_project_by_id,
        add_project,
        update_project,
        delete_project
    ]
}

#[get("/projects")]
pub async fn get_all_projects(user: User) -> ApiJsonResult<Vec<ProjectDTO>> {
    let projects = get_all_projects_for_user(user).await;
    map_and_return_projects(projects)
}

#[get("/projects/withoutInbox")]
pub async fn get_all_projects_without_inbox(user: User) -> ApiJsonResult<Vec<ProjectDTO>> {
    let projects = project_repo::get_all_projects_without_inbox(user).await;
    map_and_return_projects(projects)
}

#[get("/projects/<id>")]
pub async fn get_project_by_id(user: User, id: &str) -> ApiJsonResult<ProjectDTO> {
    let id = ObjectId::parse_str(id).map_api_err()?;
    let project = get_project_by_id_for_user(&user, id).await;
    map_and_return_project(project)
}

#[post("/projects", data = "<project>")]
pub async fn add_project(user: User, project: Json<ProjectDTO>) -> ApiJsonResult<ProjectDTO> {
    let mut project = project.into_inner();
    let project_name = project.name.clone().ok_or(ApiError::new(
        String::from("Cannot create a project with an empty name"),
        400,
    ))?;
    if project_name.to_ascii_lowercase() == "inbox" {
        return Err(ApiError::new(
            String::from("Cannot create a project with the name 'Inbox'"),
            400,
        ));
    }

    if let Some(user_id) = project.clone().user_id {
        if user_id != user.user_id {
            return Err(ApiError::new(
                String::from("Cannot create a project for another user"),
                400,
            ));
        }
    } else {
        project.user_id = Some(user.user_id.clone());
    }

    let project_model = project.to_project().map_api_err()?;
    let project = project_repo::add_project(&user, project_model).await;

    project
        .map(ProjectDTO::from_project)
        .map(Json)
        .map_api_err()
}

#[put("/projects", data = "<project>")]
pub async fn update_project(user: User, project: Json<ProjectDTO>) -> ApiJsonResult<ProjectDTO> {
    let mut project = project.into_inner();
    let project_name = project.name.clone().ok_or(ApiError::new(
        String::from("Cannot create a project with an empty name"),
        400,
    ))?;

    if project_name.to_ascii_lowercase() == "inbox" {
        return Err(ApiError::new(
            String::from("Cannot set a project name to be 'Inbox'"),
            400,
        ));
    }

    let inbox_id = match user.inbox_id.clone() {
        Some(id) => id,
        None => get_inbox_id_for_user(&user)
            .await
            .map(|x| x.to_string())
            .map_api_err()?,
    };

    if project.id.is_none() {
        return Err(ApiError::new(
            String::from("Cannot update a project without an id"),
            400,
        ));
    }

    if project.id.clone().expect("checked is some above") == inbox_id {
        return Err(ApiError::new(
            String::from("Cannot update the inbox project"),
            400,
        ));
    }

    if let Some(user_id) = project.clone().user_id {
        if user_id != user.user_id {
            return Err(ApiError::new(
                String::from("Cannot create a project for another user"),
                400,
            ));
        }
    } else {
        project.user_id = Some(user.user_id.clone());
    }

    let project_model = project.to_project().map_api_err()?;
    let project_model = project_repo::update_project_for_user(&user, project_model)
        .await
        .map_api_err()?;
    let project = ProjectDTO::from_project(project_model);
    Ok(Json(project))
}

#[delete("/projects/<id>")]
pub async fn delete_project(user: User, id: &str) -> ApiJsonResult<()> {
    let inbox_id = user
        .inbox_id
        .clone()
        .ok_or(anyhow!("no inbox id on user"))
        .map_api_err()?;

    if id == inbox_id {
        return Err(ApiError::new(
            String::from("Cannot update the inbox project"),
            400,
        ));
    }

    project_repo::delete_project_for_user(&user, id)
        .await
        .map_api_err()?;
    Ok(Json(()))
}
