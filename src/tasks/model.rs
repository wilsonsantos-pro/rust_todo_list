use chrono::{DateTime, Utc};

#[derive(Debug)]
pub struct Task {
    pub id: i32,
    pub created: DateTime<Utc>,
    pub title: String,
    pub done: bool,
}
