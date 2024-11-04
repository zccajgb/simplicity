use crate::repos::project_repo;
use crate::repos::project_repo::{get_all_projects_for_user, get_project_by_id_for_user, Project};
use crate::services::api_error::{ApiError, ApiJsonResult, ResultExt};
use crate::services::auth::User;
use bson::oid::ObjectId;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        get_all_projects,
        get_all_projects_without_inbox,
        get_project_by_id,
        add_project,
    ]
}

#[get("/projects")]
pub async fn get_all_projects(user: User) -> ApiJsonResult<Vec<Project>> {
    let projects = get_all_projects_for_user(user).await;
    projects.map(Json).map_api_err()
}

#[get("/projects/withoutInbox")]
pub async fn get_all_projects_without_inbox(user: User) -> ApiJsonResult<Vec<Project>> {
    let projects = project_repo::get_all_projects_without_inbox(user).await;
    projects.map(Json).map_api_err()
}

#[get("/projects/<id>")]
pub async fn get_project_by_id(user: User, id: String) -> ApiJsonResult<Project> {
    let id = ObjectId::parse_str(&id).map_api_err()?;
    let project = get_project_by_id_for_user(&user, id).await;
    project.map(Json).map_api_err()
}

#[post("/projects", data = "<project>")]
pub async fn add_project(user: User, project: Json<Project>) -> ApiJsonResult<ObjectId> {
    let project = project.into_inner();
    if project.name == "Inbox" {
        return Err(ApiError::new(
            String::from("Cannot create a project with the name 'Inbox'"),
            400,
        ));
    }
    if project.name.is_empty() {
        return Err(ApiError::new(
            String::from("Cannot create a project with an empty name"),
            400,
        ));
    }
    if project.user_id != user.id {
        return Err(ApiError::new(
            String::from("Cannot create a project for another user"),
            400,
        ));
    }
    let project = project_repo::add_project(user, project).await;

    project.map(Json).map_api_err()
}
