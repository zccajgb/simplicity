use chrono::Utc;
use rocket::serde::json::Json;

use crate::{
    repos::{
        last_update_repo::{get_last_update, set_last_update},
        ttl_repo::{
            update_tasks_for_today_by_date, update_tasks_for_today_from_tomorrow,
            update_tasks_for_tomorrow_by_date,
        },
    },
    routes::scheduled,
    services::{
        api_error::{ApiJsonResult, ResultExt},
        auth::ApiKey,
    },
};

pub fn get_routes() -> Vec<rocket::Route> {
    routes![scheduled::dailyupdate]
}

#[get("/dailyupdate")]
pub async fn dailyupdate(_key: ApiKey) -> ApiJsonResult<()> {
    error!("dailyupdate");
    let last_update = get_last_update().await.map_api_err()?;
    let last_update_date = last_update.last_update;
    let is_updated_today = last_update_date.map_or_else(
        || false,
        |d| d.to_chrono().date_naive() == Utc::now().date_naive(),
    );
    if is_updated_today {
        warn!("already updated today");
        return Ok(Json(()));
    }
    update_tasks_for_today_by_date().await.map_api_err()?;
    error!("updated today");
    update_tasks_for_today_from_tomorrow().await.map_api_err()?;
    error!("updated today by tomorrow");
    update_tasks_for_tomorrow_by_date().await.map_api_err()?;
    error!("updated tomorrow");
    set_last_update().await.map_api_err()?;
    error!("updated lat update");
    Ok(Json(()))
}
