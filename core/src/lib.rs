use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Memo {
    pub id: Uuid,
    pub title: Option<String>,
    pub content: String,
    pub public: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct NewMemo {
    pub title: Option<String>,
    pub content: String,
    pub public: Option<bool>,
}
