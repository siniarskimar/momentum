use chrono::serde::{ts_seconds, ts_seconds_option};
use chrono::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Task {
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,

    #[serde(with = "ts_seconds_option")]
    pub deadline: Option<DateTime<chrono::Utc>>,
}
