use actix_web::web;

use crate::{app::websocket::actions::{channels::channel_create, communities::community_join, messages::channel_send}, models::actions::{IncomingMessage, OutgoingMessage}, utils::tokens::AccessClaim};

pub mod channels;
pub mod messages;
pub mod communities;

// idk i want a separate function to handle all messages for message handle, yea ik its
// kind weirdo but fuck status quo
pub async fn action_handle(
    ws_msg: &IncomingMessage,
    user: &AccessClaim,
    pool: &web::Data<sqlx::PgPool>,
) -> OutgoingMessage {
    // maybe put this match on the ./actions/mod.rs idk 
    match &ws_msg {
        IncomingMessage::ChannelSend(data) => channel_send(&pool, &user, &data).await,
        IncomingMessage::ChannelCreate(data) => channel_create(&pool, &user, &data).await,
        IncomingMessage::CommunityJoin(data) => community_join(&pool, &user, &data).await
    }
}
