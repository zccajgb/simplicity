use crate::domain::repeat::{RepeatDTO, RepeatModel};
use crate::repos::project_repo::{get_inbox_id_for_user, get_project_by_id_for_user};
use crate::repos::tag_repo::get_tag_by_id_for_user;
use crate::repos::task_repo::{
    add_task_for_user, get_all_tasks_for_user, get_inbox_tasks_for_user, get_later_tasks_for_user,
    get_snoozed_tasks_for_user, get_task_by_id_for_user, get_tasks_by_project_for_user,
    get_tasks_by_tag_for_user, get_today_tasks_for_user, get_tomorrow_tasks_for_user,
    update_task_for_user, TaskModel,
};
use crate::routes::tasks;
use crate::services::api_error::ApiError;
use crate::services::api_error::{ApiJsonResult, ResultExt};
use anyhow::{anyhow, Context, Result};
use bson::oid::ObjectId;
use bson::DateTime;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

use super::users::User;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskDTO {
    pub _id: Option<String>,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub completed: Option<String>,
    pub ttl: Option<String>,
    pub project_id: Option<String>,
    pub tags: Vec<String>,
    pub date: Option<String>,
    pub snooze: Option<String>,
    pub repeat: Option<RepeatDTO>,
    pub order: Option<i64>,
}

impl TaskDTO {
    fn from_task_model(task: TaskModel) -> Self {
        Self {
            _id: task._id.map(|s| s.to_string()),
            name: Some(task.name),
            user_id: Some(task.user_id),
            completed: task.completed.and_then(|d| d.try_to_rfc3339_string().ok()),
            ttl: Some(task.ttl),
            project_id: task.project_id.map(|id| id.to_string()),
            tags: task.tags.iter().map(|id| id.to_string()).collect(),
            date: task.date.and_then(|d| d.try_to_rfc3339_string().ok()),
            snooze: task.snooze.and_then(|d| d.try_to_rfc3339_string().ok()),
            repeat: RepeatDTO::from_model(task.repeat),
            order: Some(task.order),
        }
    }

    fn to_task_model(self: TaskDTO) -> Result<TaskModel> {
        Ok(TaskModel {
            _id: self._id.and_then(|s| ObjectId::parse_str(s).ok()),
            name: self.name.context("name is required")?,
            user_id: self.user_id.context("user_id is required")?,
            completed: self.completed.map(|d| {
                DateTime::parse_rfc3339_str(d)
                    .context("date parse failed")
                    .expect("date parse failed")
            }),
            ttl: self.ttl.unwrap_or(String::from("later")),
            project_id: self.project_id.map(|id| {
                ObjectId::parse_str(&id)
                    .context("string to ObjectId failed")
                    .expect("date parse failed")
            }),
            tags: self.tags.iter().try_fold(Vec::new(), |mut acc, item| {
                let id = ObjectId::parse_str(item).context("string to ObjectId failed")?;
                acc.push(id);
                Ok::<Vec<ObjectId>, anyhow::Error>(acc)
            })?,

            date: self.date.map(|d| {
                DateTime::parse_rfc3339_str(d)
                    .context("date parse failed")
                    .expect("date parse failed")
            }),
            snooze: self.snooze.map(|d| {
                DateTime::parse_rfc3339_str(d)
                    .context("date parse failed")
                    .expect("date parse failed")
            }),
            repeat: self
                .repeat
                .map(|r| r.to_model())
                .unwrap_or(RepeatModel::None),
            last_updated: Some(DateTime::now()),
            order: self.order.unwrap_or(DateTime::now().timestamp_millis()),
        })
    }
    fn vec_from_task_model(tasks: Vec<TaskModel>) -> Vec<TaskDTO> {
        tasks.into_iter().map(TaskDTO::from_task_model).collect()
    }
}

pub fn map_and_return_tasks(tasks: Result<Vec<TaskModel>>) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = tasks.map_api_err()?;
    let task_dto = TaskDTO::vec_from_task_model(tasks);
    Ok(Json(task_dto))
}

pub fn map_and_return_task(tasks: Result<TaskModel>) -> ApiJsonResult<TaskDTO> {
    tasks.map(TaskDTO::from_task_model).map(Json).map_api_err()
}

pub fn get_routes() -> Vec<rocket::Route> {
    routes![
        tasks::get_all_tasks,
        tasks::get_today_tasks,
        tasks::get_tomorrow_tasks,
        tasks::get_later_tasks,
        tasks::get_inbox_tasks,
        tasks::get_task_by_id,
        tasks::get_tasks_by_project,
        tasks::get_tasks_by_tag,
        tasks::get_snoozed_tasks,
        tasks::add_task,
        tasks::update_task,
        tasks::complete_task,
        tasks::get_all_tasks_with_completed,
        tasks::get_today_tasks_with_completed,
        tasks::get_tomorrow_tasks_with_completed,
        tasks::get_later_tasks_with_completed,
        tasks::get_inbox_tasks_with_completed,
        tasks::get_tasks_by_project_with_completed,
        tasks::get_tasks_by_tag_with_completed,
        tasks::get_snoozed_tasks_with_completed
    ]
}

#[get("/tasks")]
pub async fn get_all_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_all_tasks_for_user(user, false).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/today")]
pub async fn get_today_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_today_tasks_for_user(user, false).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/tomorrow")]
pub async fn get_tomorrow_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_tomorrow_tasks_for_user(user, false).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/later")]
pub async fn get_later_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_later_tasks_for_user(user, false).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/snoozed")]
pub async fn get_snoozed_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_snoozed_tasks_for_user(user, false).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/inbox")]
pub async fn get_inbox_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let inbox_id = get_inbox_id_for_user(user.clone()).await;
    match inbox_id {
        Ok(inbox_id) => {
            let tasks = get_inbox_tasks_for_user(user, inbox_id, false).await;
            map_and_return_tasks(tasks)
        }
        Err(err) => Err(ApiError::new(err.to_string(), 400)),
    }
}

#[get("/tasks/<id>")]
pub async fn get_task_by_id(user: User, id: String) -> ApiJsonResult<TaskDTO> {
    let task = get_task_by_id_for_user(user, id).await;
    map_and_return_task(task)
}

#[get("/tasks/project/<project>")]
pub async fn get_tasks_by_project(user: User, project: String) -> ApiJsonResult<Vec<TaskDTO>> {
    let project_id = ObjectId::parse_str(&project).map_api_err()?;
    let tasks = get_tasks_by_project_for_user(user, project_id, false).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/tags/<tag>")]
pub async fn get_tasks_by_tag(user: User, tag: String) -> ApiJsonResult<Vec<TaskDTO>> {
    let tag_id = ObjectId::parse_str(&tag).map_api_err()?;
    let tasks = get_tasks_by_tag_for_user(user, tag_id, false).await;

    map_and_return_tasks(tasks)
}

#[get("/tasks/all")]
pub async fn get_all_tasks_with_completed(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_all_tasks_for_user(user, true).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/today/all")]
pub async fn get_today_tasks_with_completed(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_today_tasks_for_user(user, true).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/tomorrow/all")]
pub async fn get_tomorrow_tasks_with_completed(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_tomorrow_tasks_for_user(user, true).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/later/all")]
pub async fn get_later_tasks_with_completed(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_later_tasks_for_user(user, true).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/snoozed/all")]
pub async fn get_snoozed_tasks_with_completed(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_snoozed_tasks_for_user(user, true).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/inbox/all")]
pub async fn get_inbox_tasks_with_completed(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let inbox_id = get_inbox_id_for_user(user.clone()).await;
    match inbox_id {
        Ok(inbox_id) => {
            let tasks = get_inbox_tasks_for_user(user, inbox_id, true).await;
            map_and_return_tasks(tasks)
        }
        Err(err) => Err(ApiError::new(err.to_string(), 400)),
    }
}

#[get("/tasks/project/<project>/all")]
pub async fn get_tasks_by_project_with_completed(
    user: User,
    project: String,
) -> ApiJsonResult<Vec<TaskDTO>> {
    let project_id = ObjectId::parse_str(&project).map_api_err()?;
    let tasks = get_tasks_by_project_for_user(user, project_id, true).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/tags/<tag>/all")]
pub async fn get_tasks_by_tag_with_completed(
    user: User,
    tag: String,
) -> ApiJsonResult<Vec<TaskDTO>> {
    let tag_id = ObjectId::parse_str(&tag).map_api_err()?;
    let tasks = get_tasks_by_tag_for_user(user, tag_id, true).await;

    map_and_return_tasks(tasks)
}

#[post("/tasks", data = "<task>")]
pub async fn add_task(user: User, task: Json<TaskDTO>) -> ApiJsonResult<TaskDTO> {
    info!("Adding task: {:?}", task);
    let task = task.into_inner();

    let task_result = task_guard(user.clone(), task.clone()).await;
    let task = task_result.map_api_err()?;
    let task_model = task.to_task_model().map_api_err()?;

    let task_model = add_task_for_user(user, task_model).await;
    let task = task_model.map(TaskDTO::from_task_model).map(Json);
    task.map_api_err()
}

#[put("/tasks/<id>", data = "<task>")]
pub async fn update_task(user: User, id: String, task: Json<TaskDTO>) -> ApiJsonResult<TaskDTO> {
    let task = task.into_inner();
    let Some(taskid) = task.clone()._id else {
        return Err(ApiError::new("Task ID must be set".into(), 400));
    };
    if id != taskid {
        return Err(ApiError::new(
            "Cannot update a task with a different ID".into(),
            400,
        ));
    }

    if let Err(guard) = task_guard(user.clone(), task.clone()).await {
        return Err(ApiError::new(guard.to_string(), 400));
    }

    let task_model = task.to_task_model().map_api_err()?;
    let object_id = ObjectId::parse_str(&id).map_api_err()?;
    let task_model = update_task_for_user(user, object_id, task_model).await;
    let task = task_model.map(TaskDTO::from_task_model).map(Json);
    task.map_api_err()
}

#[put("/tasks/<id>/complete")]
pub async fn complete_task(user: User, id: String) -> ApiJsonResult<TaskDTO> {
    let object_id = ObjectId::parse_str(&id).map_api_err()?;
    let task = get_task_by_id_for_user(user.clone(), id.clone()).await;
    let mut task = task.map_api_err()?;
    if !task.repeat.is_none() {
        let _new_task = task.repeat.create_repeat(&task);
    }
    task.completed = Some(DateTime::now());
    let task = update_task_for_user(user, object_id, task).await;
    let task = task.map(TaskDTO::from_task_model).map(Json);
    task.map_api_err()
}

async fn task_guard(user: User, task: TaskDTO) -> Result<TaskDTO> {
    let mut task = task;
    if task.name.is_none() || task.clone().name.expect("task guard").is_empty() {
        return Err(anyhow!("Cannot create a task with an empty name"));
    }
    if let Some(user_id) = task.clone().user_id {
        if user_id.is_empty() {
            task.user_id = Some(user.user_id.clone());
        }
        if user_id != user.user_id {
            return Err(anyhow!("Cannot create a task for another user"));
        }
    } else {
        task.user_id = Some(user.user_id.clone());
    }
    if task.project_id.is_none() {
        task.project_id = get_inbox_id_for_user(user.clone())
            .await
            .ok()
            .map(|id| id.to_string());
    };

    let project_id = ObjectId::parse_str(task.clone().project_id.expect("project_id is required"))
        .context("string to ObjectId failed")?;
    let project = get_project_by_id_for_user(&user, project_id).await;

    if project.is_err() {
        return Err(anyhow!(
            "Cannot create a task for a project that does not exist"
        ));
    }

    if !task.tags.is_empty() {
        for tag_id in task.tags.clone() {
            let tag_id = ObjectId::parse_str(&tag_id).context("string to ObjectId failed")?;
            let tag = get_tag_by_id_for_user(&user, tag_id).await;
            if tag.is_err() {
                return Err(anyhow!(
                    "Cannot create a task with a tag that does not exist"
                ));
            }
        }
    }

    Ok(task)
}
