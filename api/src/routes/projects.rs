use crate::domain::user::User;
use crate::repos::project_repo;
use crate::repos::project_repo::{get_all_projects_for_user, get_project_by_id_for_user, Project};
use crate::services::api_error::{ApiError, ApiJsonResult, ResultExt};
use anyhow::{anyhow, Context, Result};
use bson::oid::ObjectId;
use rocket::serde::json::Json;
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct ProjectDTO {
    _id: Option<String>,
    name: Option<String>,
    user_id: Option<String>,
    completed: Option<bool>,
}

impl ProjectDTO {
    fn from_project(project: Project) -> Self {
        Self {
            _id: Some(project._id.to_string()),
            name: Some(project.name),
            user_id: Some(project.user_id),
            completed: Some(project.completed),
        }
    }

    fn to_project(self: ProjectDTO) -> Result<Project> {
        Ok(Project {
            _id: ObjectId::parse_str(self._id.ok_or(anyhow!("no id"))?).unwrap(),
            name: self.name.context("name is required")?,
            user_id: self.user_id.context("user_id is required")?,
            completed: self.completed.ok_or(false).unwrap(),
        })
    }

    fn vec_from_project_model(projects: Vec<Project>) -> Vec<ProjectDTO> {
        projects.into_iter().map(ProjectDTO::from_project).collect()
    }
}

fn map_and_return_project(project: Result<Project>) -> ApiJsonResult<ProjectDTO> {
    project
        .map(ProjectDTO::from_project)
        .map(Json)
        .map_api_err()
}

fn map_and_return_projects(projects: Result<Vec<Project>>) -> ApiJsonResult<Vec<ProjectDTO>> {
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
pub async fn add_project(user: User, project: Json<ProjectDTO>) -> ApiJsonResult<ObjectId> {
    let project = project.into_inner();
    let project_name = project.name.clone().ok_or(ApiError::new(
        String::from("Cannot create a project with an empty name"),
        400,
    ))?;
    if project_name == "Inbox" {
        return Err(ApiError::new(
            String::from("Cannot create a project with the name 'Inbox'"),
            400,
        ));
    }

    if let Some(user_id) = project.clone().user_id {
        if user_id != user.id {
            return Err(ApiError::new(
                String::from("Cannot create a project for another user"),
                400,
            ));
        }
    } else {
        return Err(ApiError::new(
            String::from("Cannot create a project without a user_id"),
            400,
        ));
    }

    let project_model = project.to_project().map_api_err()?;
    let project = project_repo::add_project(user, project_model).await;

    project.map(Json).map_api_err()
}
