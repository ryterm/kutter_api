use crate::{
    db::{auth::Auth, messages::Messages},
    models::actions::{IncomingMessage, OutgoingMessage},
    utils::{tokens::AccessClaim, validate_user},
};
use actix_web::{Error, HttpRequest, HttpResponse, error::InternalError, rt, web};
use actix_ws::AggregatedMessage;
use futures_util::StreamExt as _;
use tokio::sync::broadcast::Sender;

mod actions;

pub async fn messages_handler(
    pool: web::Data<sqlx::PgPool>,
    sender: web::Data<Sender<OutgoingMessage>>,
    req: HttpRequest,
    stream: web::Payload,
) -> Result<HttpResponse, Error> {
    let user = match validate_user(&req, &pool).await {
        Ok(user) => user,
        Err(e) => {
            println!("{:?}", e);
            return Ok(e);
        }
    };
    let (res, mut session, stream) = actix_ws::handle(&req, stream)?;
    let mut stream = stream.aggregate_continuations();

    let tx = sender.clone();
    let mut rx = tx.subscribe();
    rt::spawn(async move {
        while let Ok(msg) = rx.recv().await {
            match serde_json::to_string(&msg) {
                Ok(json) => {
                    if session.text(json).await.is_err() {
                        break;
                    }
                }
                Err(e) => {
                    println!("{:?}", e);
                }
            }
        }
    });
    rt::spawn(async move {
        while let Some(msg) = stream.next().await {
            match msg {
                Ok(AggregatedMessage::Text(txt)) => {
                    if let Ok(ws_msg) = serde_json::from_str::<IncomingMessage>(&txt) {
                        let _ = tx.send(actions::action_handle(&ws_msg, &user, &pool).await);
                    }
                }
                _ => {
                    // ugly as fuck
                }
            }
        }
    });
    Ok(res)
}
