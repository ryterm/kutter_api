use actix_web::web;

use crate::{
    db::{members::Members, messages::Messages},
    models::{actions::OutgoingMessage, message::ChannelSend},
    utils::tokens::AccessClaim,
};

pub async fn channel_send(
    pool: &web::Data<sqlx::PgPool>,
    user: &AccessClaim,
    data: &ChannelSend,
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
            return OutgoingMessage::Error(format!("{}", e));
        }
    };
    match community.contains(&data.community_id) {
        true => {}
        false => {
            return OutgoingMessage::Error(format!("Not a member of this community"));
        }
    };

    match Some(data.replied_message) {
        Some(_) => {
            match Messages::get_by_id(&pool, &data.replied_message).await {
                Ok(Some(msg)) => {
                    if msg.channel_id != data.channel_id {
                        return OutgoingMessage::Error(format!(
                            "Cannot reply a message from another channel"
                        ));
                    }
                }
                Ok(None) => {}
                Err(e) => return OutgoingMessage::Error(format!("{:?}", e)),
            };
        }
        None => {}
    };

    match Messages::channel_send(
        &pool,
        &data.channel_id,
        &user.sub,
        &data.message,
        &data.replied_message,
    )
    .await
    {
        Ok(_) => OutgoingMessage::ChannelSend(data.clone()),
        Err(e) => {
            println!("{:?}", e);
            OutgoingMessage::Error(format!("{}", e))
        }
    }
}
