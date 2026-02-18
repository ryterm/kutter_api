use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Debug, Clone)]
pub struct Community {
    pub id: i32,
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateCommunity {
    pub name: String,
    pub about: Option<String>,
    pub nsfw: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct CommunityJoin {
    pub user_id: i32,
    pub community_id: i32,
    pub admin: bool,
}

// idk if i'll rfeally use this
#[derive(Serialize, Deserialize, Debug)]
pub struct CreateInvite {
    pub community: i32,
    // TODO: add expiration time
}
