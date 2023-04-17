use chrono::{DateTime, Local};
use serde::Deserialize;

#[derive(Clone, PartialEq, Deserialize, Debug)]
pub struct Task {
    pub id: String,
    pub title: String,
    pub completed: bool,
    pub created_at: DateTime<Local>,
}

#[derive(Deserialize)]
pub struct AffectedRows {
    pub rows_affected: u64,
}
