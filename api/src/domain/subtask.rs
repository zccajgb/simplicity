use anyhow::{Context, Result};
use bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubTaskModel {
    pub name: String,
    pub completed: Option<DateTime>,
    pub last_updated: Option<DateTime>,
    pub order: i64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SubTaskDTO {
    pub name: Option<String>,
    pub completed: Option<String>,
    pub order: Option<i64>,
}

impl SubTaskDTO {
    pub fn from_subtask_model(subtask: SubTaskModel) -> SubTaskDTO {
        SubTaskDTO {
            name: Some(subtask.name),
            completed: subtask
                .completed
                .and_then(|d| d.try_to_rfc3339_string().ok()),
            order: Some(subtask.order),
        }
    }

    pub fn to_task_model(self: SubTaskDTO) -> Result<SubTaskModel> {
        Ok(SubTaskModel {
            name: self.name.context("name is required")?,
            completed: self.completed.map(|d| {
                DateTime::parse_rfc3339_str(d)
                    .context("date parse failed")
                    .expect("date parse failed")
            }),
            last_updated: Some(DateTime::now()),
            order: self.order.unwrap_or(DateTime::now().timestamp_millis()),
        })
    }
}
