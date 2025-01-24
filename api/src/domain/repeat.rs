use std::str::FromStr;

use bson::DateTime;
use chrono::{Datelike, Duration, Months, NaiveDate, TimeZone, Utc, Weekday};
use serde::{Deserialize, Serialize};

use crate::repos::task_repo::TaskModel;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RepeatModel {
    None,
    Daily,
    Weekly,
    Monthly,
    Yearly,
    EveryN { n: u32, day: String, freq: String },
    EveryNth { nth: u32, day: String, freq: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepeatDTO {
    key: Option<String>,
    freq: Option<String>,
    day: Option<String>,
    nth: Option<String>,
    n: Option<u32>,
}

impl RepeatDTO {
    pub fn from_model(model: RepeatModel) -> Option<RepeatDTO> {
        match model {
            RepeatModel::None => None,
            RepeatModel::Daily => Some(RepeatDTO {
                key: Some("daily".to_string()),
                freq: None,
                day: None,
                nth: None,
                n: None,
            }),
            RepeatModel::Weekly => Some(RepeatDTO {
                key: Some("weekly".to_string()),
                freq: None,
                day: None,
                nth: None,
                n: None,
            }),
            RepeatModel::Monthly => Some(RepeatDTO {
                key: Some("monthly".to_string()),
                freq: None,
                day: None,
                nth: None,
                n: None,
            }),
            RepeatModel::Yearly => Some(RepeatDTO {
                key: Some("yearly".to_string()),
                freq: None,
                day: None,
                nth: None,
                n: None,
            }),
            RepeatModel::EveryN { n, day, freq } => Some(RepeatDTO {
                key: Some("everyN".to_string()),
                freq: Some(freq),
                day: Some(day),
                nth: None,
                n: Some(n),
            }),
            RepeatModel::EveryNth { nth, day, freq } => Some(RepeatDTO {
                key: Some("everyNth".to_string()),
                freq: Some(freq),
                day: Some(day),
                nth: Some(RepeatDTO::nth_from(nth)),
                n: None,
            }),
        }
    }
}

impl RepeatDTO {
    fn nth_from(nth: u32) -> String {
        match nth {
            1 => "1st".to_string(),
            2 => "2nd".to_string(),
            3 => "3rd".to_string(),
            4 => "4th".to_string(),
            5 => "last".to_string(),
            _ => "0".to_string(),
        }
    }
    fn nth_into(nth: String) -> u32 {
        match nth.as_str() {
            "1st" => 1,
            "2nd" => 2,
            "3rd" => 3,
            "4th" => 4,
            "last" => 5,
            _ => 0,
        }
    }
}

impl RepeatDTO {
    pub fn to_model(&self) -> RepeatModel {
        match self.key.clone().unwrap_or("none".to_string()).as_str() {
            "none" => RepeatModel::None,
            "daily" => RepeatModel::Daily,
            "weekly" => RepeatModel::Weekly,
            "monthly" => RepeatModel::Monthly,
            "yearly" => RepeatModel::Yearly,
            "everyN" => RepeatModel::EveryN {
                n: self.n.unwrap_or(0),
                day: self.day.clone().unwrap_or("0".to_string()),
                freq: self.freq.clone().unwrap_or_default(),
            },
            "everyNth" => RepeatModel::EveryNth {
                nth: RepeatDTO::nth_into(self.nth.clone().unwrap_or("0".to_string())),
                day: self.day.clone().unwrap_or("0".to_string()),
                freq: self.freq.clone().unwrap_or_default(),
            },
            _ => RepeatModel::None,
        }
    }
}

impl RepeatModel {
    pub fn is_none(&self) -> bool {
        matches!(self, RepeatModel::None)
    }
}

impl RepeatModel {
    pub fn create_repeat(&self, task: &TaskModel) -> Option<TaskModel> {
        let mut new_task = task.clone();
        new_task._id = None;
        new_task.completed = None;
        match self {
            RepeatModel::Daily => {
                new_task.date = add_days(new_task.date, 1);
                new_task.snooze = add_days(new_task.snooze, 1);
                new_task.ttl = if new_task.ttl == "today" {
                    "tomorrow".to_string()
                } else {
                    "later".to_string()
                };
            }
            RepeatModel::Weekly => {
                new_task.date = add_days(new_task.date, 7);
                new_task.snooze = add_days(new_task.snooze, 7);
                new_task.ttl = "later".to_string();
            }
            RepeatModel::Monthly => {
                new_task.date = add_months(new_task.date, 1);
                new_task.snooze = add_months(new_task.snooze, 1);
                new_task.ttl = "later".to_string();
            }
            RepeatModel::Yearly => {
                new_task.date = add_years(new_task.date, 1);
                new_task.snooze = add_years(new_task.snooze, 1);
                new_task.ttl = "later".to_string();
            }
            RepeatModel::EveryNth { nth, day, freq } => {
                if freq.as_str() == "days" {
                    new_task.date =
                        create_date_from_nth_day(new_task.date, day.clone(), *nth as i64);
                    new_task.snooze =
                        create_date_from_nth_day(new_task.date, day.clone(), *nth as i64);
                }
            }
            RepeatModel::EveryN { n, day: _day, freq } => match freq.as_str() {
                "days" => {
                    new_task.date = add_days(new_task.date, *n);
                    new_task.snooze = add_days(new_task.snooze, *n);
                }
                "weeks" => {
                    new_task.date = add_days(new_task.date, *n);
                    new_task.snooze = add_days(new_task.snooze, *n);
                }
                "months" => {
                    new_task.date = add_months(new_task.date, *n);
                    new_task.snooze = add_months(new_task.snooze, *n);
                }
                "years" => {
                    new_task.date = add_years(new_task.date, *n);
                    new_task.snooze = add_years(new_task.snooze, *n);
                }
                _ => {
                    return None;
                }
            },
            _ => {
                return None;
            }
        };
        Some(new_task)
    }
}

fn create_date_from_nth_day(date: Option<DateTime>, day: String, nth: i64) -> Option<DateTime> {
    let date = date?;
    let target_day = Weekday::from_str(day.as_str()).ok()?;
    let target_day_n_from_monday = target_day.number_from_monday() as i64;

    let chrono_datetime = date.to_chrono();
    let next_month = chrono_datetime.checked_add_months(Months::new(1))?;
    let first_of_month = NaiveDate::from_ymd_opt(next_month.year(), next_month.month(), 1);
    let first_of_month_n_from_monday = first_of_month?.weekday().number_from_monday() as i64;

    let mut first_target_day = 1 + first_of_month_n_from_monday - target_day_n_from_monday;
    if target_day_n_from_monday < first_of_month_n_from_monday {
        first_target_day += 7;
    }

    let mut target_day = first_target_day + (nth - 1) * 7;
    if nth == 5 {
        let last_day_of_month = first_of_month?
            .signed_duration_since(next_month.date_naive())
            .num_days();
        if target_day > last_day_of_month {
            target_day -= 7;
        }
    }

    let target_date =
        NaiveDate::from_ymd_opt(next_month.year(), next_month.month(), target_day as u32);
    let target_datetime = target_date?.and_time(chrono_datetime.time());
    let utc_datetime = Utc.from_utc_datetime(&target_datetime);
    Some(DateTime::from_chrono(utc_datetime))
}

fn add_days(date: Option<DateTime>, days: u32) -> Option<DateTime> {
    let date = date?;

    let chrono_datetime = date.to_chrono();

    let new_datetime = chrono_datetime + Duration::days(days as i64);

    let new_date = DateTime::from_chrono(new_datetime);
    Some(new_date)
}

fn add_months(date: Option<DateTime>, months: u32) -> Option<DateTime> {
    let date = date?;

    let chrono_datetime = date.to_chrono();

    let new_datetime = chrono_datetime.checked_add_months(Months::new(months))?;

    let new_date = DateTime::from_chrono(new_datetime);
    Some(new_date)
}

fn add_years(date: Option<DateTime>, years: u32) -> Option<DateTime> {
    let date = date?;

    let chrono_datetime = date.to_chrono();

    let new_datetime = chrono_datetime.with_year(chrono_datetime.year() + years as i32)?;

    let new_date = DateTime::from_chrono(new_datetime);
    Some(new_date)
}
