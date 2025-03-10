use chrono::serde::ts_seconds;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

use crate::model::object::ObjectID;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: ObjectID,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    #[serde(with = "ts_seconds")]
    pub date: DateTime<chrono::Utc>,

    #[serde(with = "ts_seconds")]
    pub end_date: DateTime<chrono::Utc>,
}

impl Event {
    pub fn new(title: &str, date: DateTime<chrono::Utc>) -> Self {
        return Self {
            id: ObjectID::new(),
            title: String::from(title),
            description: String::new(),
            tags: vec![],
            date,
            end_date: date
                .checked_add_signed(chrono::TimeDelta::hours(1))
                .unwrap(),
        };
    }
}
