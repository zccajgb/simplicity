use crate::domain::user::User;
use crate::repos::project_repo::{get_inbox_id_for_user, get_project_by_id_for_user};
use crate::repos::tag_repo::get_tag_by_id_for_user;
use crate::repos::task_repo::{
    add_task_for_user, get_all_tasks_for_user, get_inbox_tasks_for_user, get_task_by_id_for_user,
    get_tasks_by_project_for_user, get_tasks_by_tag_for_user, get_today_tasks_for_user,
    get_tomorrow_tasks_for_user, update_task_for_user, TaskModel,
};
use crate::routes::tasks;
use crate::services::api_error::ApiError;
use crate::services::api_error::{ApiJsonResult, ResultExt};
use anyhow::{anyhow, Context, Result};
use bson::oid::ObjectId;
use bson::DateTime;
use rocket::serde::json::Json;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct TaskDTO {
    pub _id: Option<String>,
    pub user_id: Option<String>,
    pub name: Option<String>,
    pub completed: Option<bool>,
    pub ttl: Option<String>,
    pub project_id: Option<String>,
    pub tags: Vec<String>,
    pub date: Option<i64>,
    pub snooze: Option<i64>,
}

impl TaskDTO {
    fn from_task_model(task: TaskModel) -> Self {
        Self {
            _id: Some(task._id.to_string()),
            name: Some(task.name),
            user_id: Some(task.user_id),
            completed: Some(task.completed),
            ttl: Some(task.ttl),
            project_id: task.project_id.map(|id| id.to_string()),
            tags: task.tags.iter().map(|id| id.to_string()).collect(),
            date: task.date.map(|d| d.timestamp_millis()),
            snooze: task.snooze.map(|d| d.timestamp_millis()),
        }
    }

    fn to_task_model(self: TaskDTO) -> Result<TaskModel> {
        Ok(TaskModel {
            _id: ObjectId::parse_str(self._id.ok_or(anyhow!("no id"))?)?,
            name: self.name.context("name is required")?,
            user_id: self.user_id.context("user_id is required")?,
            completed: self.completed.ok_or(false).unwrap(),
            ttl: self.ttl.ok_or("ttl is required").unwrap(),
            project_id: self.project_id.map(|id| {
                ObjectId::parse_str(&id)
                    .context("string to ObjectId failed")
                    .unwrap()
            }),
            tags: self.tags.iter().try_fold(Vec::new(), |mut acc, item| {
                let id = ObjectId::parse_str(item).context("string to ObjectId failed")?;
                acc.push(id);
                Ok::<Vec<ObjectId>, anyhow::Error>(acc)
            })?,

            date: self.date.map(DateTime::from_millis),
            snooze: self.snooze.map(DateTime::from_millis),
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
        tasks::get_inbox_tasks,
        tasks::get_task_by_id,
        tasks::get_tasks_by_project,
        tasks::get_tasks_by_tag,
        tasks::add_task,
        tasks::update_task,
    ]
}

#[get("/tasks")]
pub async fn get_all_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_all_tasks_for_user(user).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/today")]
pub async fn get_today_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_today_tasks_for_user(user).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/tomorrow")]
pub async fn get_tomorrow_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let tasks = get_tomorrow_tasks_for_user(user).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/inbox")]
pub async fn get_inbox_tasks(user: User) -> ApiJsonResult<Vec<TaskDTO>> {
    let inbox_id = get_inbox_id_for_user(user.clone()).await;
    match inbox_id {
        Ok(inbox_id) => {
            let tasks = get_inbox_tasks_for_user(user, inbox_id).await;
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
    let tasks = get_tasks_by_project_for_user(user, project_id).await;
    map_and_return_tasks(tasks)
}

#[get("/tasks/tags/<tag>")]
pub async fn get_tasks_by_tag(user: User, tag: String) -> ApiJsonResult<Vec<TaskDTO>> {
    let tag_id = ObjectId::parse_str(&tag).map_api_err()?;
    let tasks = get_tasks_by_tag_for_user(user, tag_id).await;

    map_and_return_tasks(tasks)
}

#[post("/tasks", data = "<task>")]
pub async fn add_task(user: User, task: Json<TaskDTO>) -> ApiJsonResult<ObjectId> {
    error!("Adding task: {:?}", task);
    let task = task.into_inner();

    let task_result = task_guard(user.clone(), task.clone()).await;
    let task = task_result.map_api_err()?;
    let task_model = task.to_task_model().map_api_err()?;

    let id = add_task_for_user(user, task_model).await;
    id.map(Json).map_api_err()
}

#[put("/tasks/<id>", data = "<task>")]
pub async fn update_task(user: User, id: String, task: Json<TaskDTO>) -> ApiJsonResult<ObjectId> {
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
    let new_id = update_task_for_user(user, object_id, task_model).await;

    new_id.map(Json).map_api_err()
}

async fn task_guard(user: User, task: TaskDTO) -> Result<TaskDTO> {
    let mut task = task;
    if task.name.is_none() || task.clone().name.unwrap().is_empty() {
        return Err(anyhow!("Cannot create a task with an empty name"));
    }
    if let Some(user_id) = task.clone().user_id {
        if user_id.is_empty() {
            task.user_id = Some(user.id.clone());
        }
        if user_id != user.id {
            return Err(anyhow!("Cannot create a task for another user"));
        }
    } else {
        task.user_id = Some(user.id.clone());
    }

    if task.project_id.is_none() {
        task.project_id = get_inbox_id_for_user(user.clone())
            .await
            .ok()
            .map(|id| id.to_string());
    };

    let project_id = ObjectId::parse_str(task.clone().project_id.unwrap())
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
