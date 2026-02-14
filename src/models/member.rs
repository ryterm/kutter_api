use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct Member {
    pub id: i32,
    pub user_id: i32,
    pub community_id: i32,
    pub owner: bool,
    pub admin: bool,
}

#[derive(FromRow, Serialize, Deserialize, Debug)]
pub struct MemberCommunities {
    pub community_id: Option<i32>,
}
