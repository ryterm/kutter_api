use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Channel {
    pub id: i32,
    pub community: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelCreate {
    pub community_id: i32,
    pub name: String,
    pub topic: Option<String>,
    pub hidden: bool,
}
