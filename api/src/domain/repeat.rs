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
    key: String,
    freq: String,
    day: String,
    nth: String,
    n: u32,
}

impl RepeatDTO {
    pub fn from_model(model: RepeatModel) -> Option<RepeatDTO> {
        match model {
            RepeatModel::None => None,
            RepeatModel::Daily => Some(RepeatDTO {
                key: "daily".to_string(),
                freq: "".to_string(),
                day: "".to_string(),
                nth: "0".to_string(),
                n: 0,
            }),
            RepeatModel::Weekly => Some(RepeatDTO {
                key: "weekly".to_string(),
                freq: "".to_string(),
                day: "".to_string(),
                nth: "0".to_string(),
                n: 0,
            }),
            RepeatModel::Monthly => Some(RepeatDTO {
                key: "monthly".to_string(),
                freq: "".to_string(),
                day: "".to_string(),
                nth: "0".to_string(),
                n: 0,
            }),
            RepeatModel::Yearly => Some(RepeatDTO {
                key: "yearly".to_string(),
                freq: "".to_string(),
                day: "".to_string(),
                nth: "0".to_string(),
                n: 0,
            }),
            RepeatModel::EveryN { n, day, freq } => Some(RepeatDTO {
                key: "everyN".to_string(),
                freq,
                day,
                nth: "0".to_string(),
                n,
            }),
            RepeatModel::EveryNth { nth, day, freq } => Some(RepeatDTO {
                key: "everyNth".to_string(),
                freq,
                day,
                nth: RepeatDTO::nth_from(nth),
                n: 0,
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
        match self.key.as_str() {
            "none" => RepeatModel::None,
            "daily" => RepeatModel::Daily,
            "weekly" => RepeatModel::Weekly,
            "monthly" => RepeatModel::Monthly,
            "yearly" => RepeatModel::Yearly,
            "everyN" => RepeatModel::EveryN {
                n: self.n,
                day: self.day.clone(),
                freq: self.freq.clone(),
            },
            "everyNth" => RepeatModel::EveryNth {
                nth: RepeatDTO::nth_into(self.nth.clone()),
                day: self.day.clone(),
                freq: self.freq.clone(),
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
            RepeatModel::EveryNth { nth, day, freq } => match freq.as_str() {
                "days" => {
                    new_task.date =
                        create_date_from_nth_day(new_task.date, day.clone(), *nth as i64);
                    new_task.snooze =
                        create_date_from_nth_day(new_task.date, day.clone(), *nth as i64);
                }
                // "weeks" => {
                //     new_task.date = create_date_from_nth_week(new_task.date, *freq, *nth.into());
                //     new_task.snooze = create_date_from_nth_week(new_task.date, *freq, *nth.into());
                // }
                // "months" => {
                //     new_task.date = create_date_from_nth_month(new_task.date, *freq, *nth.into());
                //     new_task.snooze = create_date_from_nth_month(new_task.date, *freq, *nth.into());
                // }
                _ => {}
            },
            RepeatModel::EveryN { n, day, freq } => match freq.as_str() {
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
        return Some(new_task);
    }
}

fn create_date_from_nth_day(date: Option<DateTime>, day: String, nth: i64) -> Option<DateTime> {
    let Some(date) = date else {
        return None;
    };
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
    let Some(date) = date else {
        return None;
    };

    let chrono_datetime = date.to_chrono();

    let new_datetime = chrono_datetime + Duration::days(days as i64);

    let new_date = DateTime::from_chrono(new_datetime);
    return Some(new_date);
}

fn add_months(date: Option<DateTime>, months: u32) -> Option<DateTime> {
    let Some(date) = date else {
        return None;
    };

    let chrono_datetime = date.to_chrono();

    let new_datetime = chrono_datetime.checked_add_months(Months::new(months))?;

    let new_date = DateTime::from_chrono(new_datetime);
    return Some(new_date);
}

fn add_years(date: Option<DateTime>, years: u32) -> Option<DateTime> {
    let Some(date) = date else {
        return None;
    };

    let chrono_datetime = date.to_chrono();

    let new_datetime = chrono_datetime.with_year(chrono_datetime.year() + years as i32)?;

    let new_date = DateTime::from_chrono(new_datetime);
    return Some(new_date);
}
