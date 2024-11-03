use crate::repos::task_repo::{get_all_tasks_for_user, Task};
use crate::routes::tasks;
use crate::services::auth::{validate_token_and_get_user, User};
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
pub async fn get_all_tasks(user: User) -> Json<Vec<Task>> {
    let tasks = get_all_tasks_for_user(user).await;
    return match tasks {
        Ok(tasks) => Json(tasks),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/tasks/today")]
pub async fn get_today_tasks(user: User) -> Json<Vec<Task>> {
    let tasks = get_today_tasks_for_user(user).await;
    return match tasks {
        Ok(tasks) => Json(tasks),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/tasks/tomorrow")]
pub async fn get_tomorrow_tasks(user: User) -> Json<Vec<Task>> {
    let tasks = get_tomorrow_tasks_for_user(user).await;
    return match tasks {
        Ok(tasks) => Json(tasks),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/tasks/inbox")]
pub async fn get_inbox_tasks(user: User) -> Json<Vec<Task>> {
    let tasks = get_inbox_tasks_for_user(user).await;
    return match tasks {
        Ok(tasks) => Json(tasks),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/tasks/<id>")]
pub async fn get_task_by_id(user: User, id: String) -> Json<Task> {
    let task = get_task_by_id_for_user(user, id).await;
    return match task {
        Ok(task) => Json(task),
        Err(e) => Json(Task::default()),
    };
}

#[get("/tasks/project/<project>")]
pub async fn get_tasks_by_project(user: User, project: String) -> Json<Vec<Task>> {
    let tasks = get_tasks_by_project_for_user(user, project).await;
    return match tasks {
        Ok(tasks) => Json(tasks),
        Err(e) => Json(Vec::new()),
    };
}

#[get("/tasks/tags/<tag>")]
pub async fn get_tasks_by_tag(user: User, tag: String) -> Json<Vec<Task>> {
    let tasks = get_tasks_by_tag_for_user(user, tag).await;
    return match tasks {
        Ok(tasks) => Json(tasks),
        Err(e) => Json(Vec::new()),
    };
}

#[post("/tasks", data = "<task>")]
pub async fn add_task(user: User, task: Json<Task>) -> Json<Task> {
    let task = task.into_inner();

    let guard = task_guard(user, task).await;
    if (guard.is_err()) {
        return Json(Task::default());
    }

    let task = add_task_for_user(user, task).await;

    return match task {
        Ok(task) => Json(task),
        Err(e) => Json(Task::default()),
    };
}

#[put("/tasks/<id>", data = "<task>")]
pub async fn update_task(user: User, id: String, task: Json<Task>) -> Json<Task> {
    let task = update_task_for_user(user, id, task.into_inner()).await;

    let guard = task_guard(user, task).await;
    if (guard.is_err()) {
        return Json(Task::default());
    }

    return match task {
        Ok(task) => Json(task),
        Err(e) => Json(Task::default()),
    };
}

async fn task_guard(user: User, task: Task) -> Result<()> {
    if task.name == "" {
        return Err("Cannot create a task with an empty name".into());
    }
    if task.user_id != user.id {
        return Err("Cannot create a task for another user".into());
    }

    let project = get_project_by_id_for_user(user, task.project_id).await;

    if (project.is_err()) {
        return Err("Cannot create a task for a project that does not exist".into());
    }

    if (tags.len() > 0) {
        for tag in tags {
            let tag = get_tag_by_id_for_user(user, tag).await;
            if (tag.is_err()) {
                return Err("Cannot create a task with a tag that does not exist".into());
            }
        }
    }

    return Ok(());
}
