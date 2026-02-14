use actix_web::{
    HttpRequest, HttpResponse,
    body::{self, None},
    web,
};
use bcrypt::{DEFAULT_COST, hash};

use crate::{
    db::{communities::Communities, invites::Invites, members::Members},
    models::community::{CreateCommunity, CreateInvite},
    utils::{validate_user},
};

pub async fn create_community(
    pool: web::Data<sqlx::PgPool>,
    req: HttpRequest,
    body: web::Json<CreateCommunity>,
) -> HttpResponse {
    let user = match validate_user(&req, &pool).await {
        Ok(user) => user,
        Err(e) => return e,
    };
    match Communities::create(&pool, &body.name, &body.about, &body.nsfw).await {
        Ok(community) => match Members::join(&pool, &user.sub, &community.id, &true, &true).await {
            Ok(_) => HttpResponse::Ok().body("Community created successfully"),
            Err(e) => return HttpResponse::InternalServerError().body(format!("{}", e)),
        },
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

