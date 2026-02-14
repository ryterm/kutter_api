use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct ChannelMessage {
    pub id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
    pub timestamp: DateTime<Utc>,
    pub edited: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelSend {
    pub community_id: i32,
    pub channel_id: i32,
    pub user_id: i32,
    pub message: String,
    pub replied_message: Option<i32>,
}
