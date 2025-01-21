use super::task_repo::{get_tasks_collection, get_tasks_inner, update_task_for_user, TaskModel};
use anyhow::Result;
use chrono::Utc;
use mongodb::bson::{doc, DateTime};

pub async fn update_tasks_for_tomorrow_by_date() -> Result<()> {
    let tomorrow = Utc::now().date_naive().succ_opt().unwrap();
    let start_of_tomorrow = tomorrow.and_hms_opt(0, 0, 0).unwrap();
    let start_of_tomorrow_millis = start_of_tomorrow.and_utc().timestamp_millis();
    let end_of_tomorrow = tomorrow.and_hms_opt(23, 59, 59).unwrap();
    let end_of_tomorrow_millis = end_of_tomorrow.and_utc().timestamp_millis();
    update_tasks_for_ttl_by_date(
        start_of_tomorrow_millis,
        end_of_tomorrow_millis,
        "tomorrow".to_string(),
    )
    .await
}

pub async fn update_tasks_for_today_by_date() -> Result<()> {
    let today = Utc::now().date_naive();
    let start_of_today = today.and_hms_opt(0, 0, 0).unwrap();
    let start_of_today_millis = start_of_today.and_utc().timestamp_millis();
    let end_of_today = today.and_hms_opt(23, 59, 59).unwrap();
    let end_of_today_millis = end_of_today.and_utc().timestamp_millis();
    update_tasks_for_ttl_by_date(
        start_of_today_millis,
        end_of_today_millis,
        "today".to_string(),
    )
    .await
}

pub async fn update_tasks_for_today_from_tomorrow() -> Result<()> {
    let filter = doc! { "ttl": "tomorrow" };
    let collection = get_tasks_collection().await?;
    let update_doc = doc! { "$set": {"ttl": "today", "last_updated": DateTime::now() } };
    let update_result = collection.update_many(filter, update_doc).await?;

    if update_result.modified_count == 0 {
        warn!("No tasks found for today by ttl");
    }

    info!(
        "updated {} tasks for today because tomorrow",
        update_result.modified_count
    );

    Ok(())
}

pub async fn update_tasks_for_ttl_by_date(
    start_date: i64,
    end_date: i64,
    ttl: String,
) -> Result<()> {
    let collection = get_tasks_collection().await?;
    let filter = doc! {
      "date": {
        "$gte": DateTime::from_millis(start_date),
        "$lte": DateTime::from_millis(end_date),
      }
    };

    let update_doc = doc! { "$set": {"ttl": ttl.clone(), "last_updated": DateTime::now() } };
    let update_result = collection.update_many(filter, update_doc).await?;
    if update_result.modified_count == 0 {
        warn!("No tasks found for {} by date", ttl);
    }
    info!("updated {} tasks for {}", update_result.modified_count, ttl);

    Ok(())
}

pub fn change_today_task_to_tomorrow() {}

pub fn change_tomorrow_task_to_later() {}

pub fn change_today_task_to_later() {}
