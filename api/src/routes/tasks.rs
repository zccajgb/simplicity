use crate::domain::user::User;
use crate::repos::project_repo::{get_inbox_id_for_user, get_project_by_id_for_user};
use crate::repos::tag_repo::get_tag_by_id_for_user;
use crate::repos::task_repo::{
    add_task_for_user, get_all_tasks_for_user, get_inbox_tasks_for_user, get_task_by_id_for_user,
    get_tasks_by_project_for_user, get_tasks_by_tag_for_user, get_today_tasks_for_user,
    get_tomorrow_tasks_for_user, update_task_for_user, Task,
};
use crate::routes::tasks;
use crate::services::api_error::ApiError;
use crate::services::api_error::{ApiJsonResult, ResultExt};
use anyhow::{anyhow, Result};
use bson::oid::ObjectId;
use rocket::serde::json::Json;

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        tasks::get_all_tasks,
        tasks::get_today_tasks,
        tasks::get_tomorrow_tasks,
        tasks::get_inbox_tasks,
        tasks::get_task_by_id,
        tasks::get_tasks_by_project,
        tasks::get_tasks_by_tag,
        tasks::add_task,
        tasks::update_task,
    ]
}

#[get("/tasks")]
pub async fn get_all_tasks(user: User) -> ApiJsonResult<Vec<Task>> {
    let tasks = get_all_tasks_for_user(user).await;
    tasks.map(Json).map_api_err()
}

#[get("/tasks/today")]
pub async fn get_today_tasks(user: User) -> ApiJsonResult<Vec<Task>> {
    let tasks = get_today_tasks_for_user(user).await;
    tasks.map(Json).map_api_err()
}

#[get("/tasks/tomorrow")]
pub async fn get_tomorrow_tasks(user: User) -> ApiJsonResult<Vec<Task>> {
    let tasks = get_tomorrow_tasks_for_user(user).await;
    tasks.map(Json).map_api_err()
}

#[get("/tasks/inbox")]
pub async fn get_inbox_tasks(user: User) -> ApiJsonResult<Vec<Task>> {
    let inbox_id = get_inbox_id_for_user(user.clone()).await;

    match inbox_id {
        Ok(inbox_id) => {
            let tasks = get_inbox_tasks_for_user(user, inbox_id).await;
            tasks.map(Json).map_api_err()
        }
        Err(err) => Err(ApiError::new(err.to_string(), 400)),
    }
}

#[get("/tasks/<id>")]
pub async fn get_task_by_id(user: User, id: String) -> ApiJsonResult<Task> {
    let task = get_task_by_id_for_user(user, id).await;
    task.map(Json).map_api_err()
}

#[get("/tasks/project/<project>")]
pub async fn get_tasks_by_project(user: User, project: String) -> ApiJsonResult<Vec<Task>> {
    let project_id = ObjectId::parse_str(&project).map_api_err()?;
    let tasks = get_tasks_by_project_for_user(user, project_id).await;
    tasks.map(Json).map_api_err()
}

#[get("/tasks/tags/<tag>")]
pub async fn get_tasks_by_tag(user: User, tag: String) -> ApiJsonResult<Vec<Task>> {
    let tag_id = ObjectId::parse_str(&tag).map_api_err()?;
    let tasks = get_tasks_by_tag_for_user(user, tag_id).await;
    tasks.map(Json).map_api_err()
}

#[post("/tasks", data = "<task>")]
pub async fn add_task(user: User, task: Json<Task>) -> ApiJsonResult<ObjectId> {
    let task = task.into_inner();

    let guard = task_guard(user.clone(), task.clone()).await;
    if let Err(guard) = guard {
        return Err(ApiError::new(guard.to_string(), 400));
    }

    let task = add_task_for_user(user, task).await;
    task.map(Json).map_api_err()
}

#[put("/tasks/<id>", data = "<task>")]
pub async fn update_task(user: User, id: String, task: Json<Task>) -> ApiJsonResult<ObjectId> {
    let task = task.into_inner();
    let id = ObjectId::parse_str(&id).map_api_err()?;
    if id != task._id {
        return Err(ApiError::new(
            "Cannot update a task with a different ID".into(),
            400,
        ));
    }

    if let Err(guard) = task_guard(user.clone(), task.clone()).await {
        return Err(ApiError::new(guard.to_string(), 400));
    }

    let task = update_task_for_user(user, id, task).await;

    task.map(Json).map_api_err()
}

async fn task_guard(user: User, task: Task) -> Result<()> {
    if task.name.is_empty() {
        return Err(anyhow!("Cannot create a task with an empty name"));
    }
    if task.user_id != user.id {
        anyhow::bail!("Cannot create a task for another user");
    }

    let project = get_project_by_id_for_user(&user, task.project_id).await;

    if project.is_err() {
        return Err(anyhow!(
            "Cannot create a task for a project that does not exist"
        ));
    }

    if !task.tags.is_empty() {
        for tag_id in task.tags {
            let tag = get_tag_by_id_for_user(&user, tag_id).await;
            if tag.is_err() {
                return Err(anyhow!(
                    "Cannot create a task with a tag that does not exist"
                ));
            }
        }
    }

    Ok(())
}
