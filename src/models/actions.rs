use serde::{Deserialize, Serialize};

use crate::models::{channel::ChannelCreate, community::CommunityJoin, message::ChannelSend};

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "action", content = "data", rename_all = "snake_case")]
pub enum IncomingMessage {
    ChannelSend(ChannelSend),
    ChannelCreate(ChannelCreate),
    CommunityJoin(CommunityJoin),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "action", content = "data", rename_all = "snake_case")]
pub enum OutgoingMessage {
    ChannelSend(ChannelSend),
    ChannelCreate(ChannelCreate),
    CommunityJoin(CommunityJoin),
    Error(String), // idk if it's valid to put a error code like HTTP
}
