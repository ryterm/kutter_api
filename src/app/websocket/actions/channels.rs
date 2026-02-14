use actix_web::web;

use crate::{
    db::{channels::Channels, members::Members, messages::Messages},
    models::{
        actions::{IncomingMessage, OutgoingMessage}, channel::ChannelCreate, message::ChannelSend
    },
    utils::tokens::AccessClaim,
};

pub async fn channel_create(
    pool: &web::Data<sqlx::PgPool>,
    user: &AccessClaim,
    data: &ChannelCreate
) -> OutgoingMessage {
    let community = match Members::get(&pool, &user.sub).await {
        Ok(member) => {
            let mut communities: Vec<i32> = Vec::new();
            for info in member {
                communities.push(info.community_id);
            }
            communities
        }
        Err(e) => {
            return OutgoingMessage::Error(format!("{:?}", e))
        }
    };
    match community.contains(&data.community_id) {
        true => {
            match Channels::create(
                &pool,
                &data.community_id,
                &data.name,
                &data.topic,
                &data.hidden,
            )
            .await
            {
                Ok(_) => OutgoingMessage::ChannelCreate(data.clone()),
                Err(e) => OutgoingMessage::Error(format!("{}", e))
            }
        }
        false => OutgoingMessage::Error(format!("Unauthorized"))
    }
}
