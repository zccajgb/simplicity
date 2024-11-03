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
pub async fn get_all_projects(user: User) -> Json<Vec<Project>> {
    let projects = get_all_projects_for_user(user).await;
    return match projects {
        Ok(projects) => Json(projects),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/projects/withoutInbox")]
pub async fn get_all_projects_without_inbox(user: User) -> Json<Vec<Project>> {
    let projects = get_all_projects_without_inbox_for_user(user).await;
    return match projects {
        Ok(projects) => Json(projects),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/projects/<id>")]
pub async fn get_project_by_id(user: User, id: String) -> Json<Project> {
    let project = get_project_by_id_for_user(user, id).await;
    return match project {
        Ok(project) => Json(project),
        Err(e) => Json(Project::default()),
    };
}

#[post("/projects", data = "<project>")]
pub async fn add_project(user: User, project: Json<Project>) -> Json<Project> {
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
    return match project {
        Ok(project) => Json(project),
        Err(e) => Json(Project::default()),
    };
}
