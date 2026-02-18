use actix_web::{HttpRequest, web};

use crate::{
    db::members::Members,
    models::{
        actions::OutgoingMessage,
        community::{self, CommunityJoin},
    },
    utils::tokens::AccessClaim,
};

pub async fn community_join(
    pool: &web::Data<sqlx::PgPool>,
    user: &AccessClaim,
    data: &CommunityJoin,
) -> OutgoingMessage {
    match Members::join(&pool, &user.sub, &data.community_id, &false, &false).await {
        Ok(_) => OutgoingMessage::CommunityJoin(data.clone()),
        Err(e) => {
            println!("{:?}", e);
            OutgoingMessage::Error(format!("{}", e))
        }
    }
}
