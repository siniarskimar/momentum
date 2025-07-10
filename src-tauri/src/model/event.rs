use chrono::serde::ts_seconds;
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Event {
    pub id: u64,
    pub title: String,
}

impl Event {
    pub fn new(title: &str) -> Self {
        return Self {
            id: 0,
            title: String::from(title),
        };
    }
}
