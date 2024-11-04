use crate::repos::project_repo::{
    add_project_for_user, get_all_projects_for_user, get_all_projects_without_inbox_for_user,
    get_project_by_id_for_user, Project,
};
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
pub async fn get_all_projects(user: User) -> ApiJsonError<Vec<Project>> {
    let projects = get_all_projects_for_user(user).await;
    projects.map(|projects| Json(projects)).map_api_err()
}

#[get("/projects/withoutInbox")]
pub async fn get_all_projects_without_inbox(user: User) -> ApiJsonError<Vec<Project>> {
    let projects = get_all_projects_without_inbox_for_user(user).await;
    projects.map(|projects| Json(projects)).map_api_err()
}

#[get("/projects/<id>")]
pub async fn get_project_by_id(user: User, id: String) -> ApiJsonError<Project> {
    let project = get_project_by_id_for_user(user, id).await;
    project.map(|project| Json(project)).map_api_err()
}

#[post("/projects", data = "<project>")]
pub async fn add_project(user: User, project: Json<Project>) -> ApiJsonError<Project> {
    let project = project.into_inner();
    if project.name == "Inbox" {
        Err("Cannot create a project with the name 'Inbox'".into())
    }
    if project.name == "" {
        Err("Cannot create a project with an empty name".into())
    }
    if project.user_id != user.id {
        Err("Cannot create a project for another user".into())
    }
    let project = add_project_for_user(user, project.into_inner()).await;

    project.map(|project| Json(project)).map_api_err()
}
