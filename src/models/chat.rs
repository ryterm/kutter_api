use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Message {
    pub id: u32,
    pub replied_user: Option<u32>,
    pub reṕlied_message: Option<u32>,
    pub message: String,
    pub username: u32,
    pub edited: bool,
    pub timestamp: DateTime<Utc>,
}
