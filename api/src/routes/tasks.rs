use crate::repos::task_repo::{get_all_tasks_for_user, Task};
use crate::routes::tasks;
use crate::services::auth::User;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        tasks::get_all_tasks,
        tasks::get_all_tasks_no_auth,
        tasks::test
    ]
}

#[get("/tasks")]
pub async fn get_all_tasks(user: User) -> Json<Vec<Task>> {
    let tasks = get_all_tasks_for_user(user).await;
    let Ok(tasks) = tasks else {
        return Json(Vec::new());
    };
    Json(tasks)
}

#[get("/tasksNoAuth")]
pub async fn get_all_tasks_no_auth() -> Json<Vec<Task>> {
    let user = User {
        id: "117228399787730243293".to_string(),
    };
    let tasks = get_all_tasks_for_user(user).await;
    let Ok(tasks) = tasks else {
        return Json(Vec::new());
    };
    Json(tasks)
}

#[get("/test")]
pub async fn test() -> &'static str {
    "Hello, world!"
}
