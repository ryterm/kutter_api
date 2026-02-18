use actix_web::{HttpRequest, HttpResponse, web};
use bcrypt::{DEFAULT_COST, hash, verify};
use chrono::Utc;
use sqlx::PgPool;

use crate::{
    db::{auth::Auth, tokens::Tokens},
    models::user::{UserLogin, UserRegister},
    utils::{
        cookies::CookieGenerator,
        tokens::{AccessClaim, SessionClaim},
        validate_user,
    },
};

pub async fn register(pool: web::Data<PgPool>, body: web::Json<UserRegister>) -> HttpResponse {
    let hashed_password = match hash(body.password.clone(), DEFAULT_COST) {
        Ok(hash) => hash,
        Err(e) => {
            eprintln!("Error hashing password: {}", e);
            return HttpResponse::InternalServerError().body("Error hashing password");
        }
    };

    let created_at = Utc::now();
    match Auth::register(
        &pool,
        body.username.clone(),
        body.email.clone(),
        hashed_password,
        created_at,
    )
    .await
    {
        Ok(_) => HttpResponse::Ok().body("registered successfully"),
        Err(e) => HttpResponse::InternalServerError().body(format!("{}", e)),
    }
}

pub async fn login(pool: web::Data<PgPool>, body: web::Json<UserLogin>) -> HttpResponse {
    let user = match Auth::get_by_email(&pool, body.email.clone()).await {
        Ok(Some(user)) => user,
        Ok(None) => return HttpResponse::BadRequest().body("User not found"),
        Err(_) => return HttpResponse::InternalServerError().body("Error getting user"),
    };

    let is_psswrd_valid = match verify(body.password.clone(), &user.password) {
        Ok(is_valid) => is_valid,
        Err(_) => return HttpResponse::InternalServerError().body("Error verifying password"),
    };

    match is_psswrd_valid {
        true => {
            let session_token = SessionClaim::generate(user.id);
            match Tokens::insert(&pool, user.id).await {
                Ok(_) => {}
                Err(e) => {
                    return HttpResponse::InternalServerError()
                        .body(format!("Error inserting token to database: {}", e));
                }
            }
            HttpResponse::Ok()
                .cookie(CookieGenerator::session_cookie(session_token))
                .body("Logged in successfully")
        }
        false => HttpResponse::BadRequest().body("invalid psswrd"),
    }
}

// NOTE: debug only
pub async fn token_test(pool: web::Data<PgPool>, req: HttpRequest) -> HttpResponse {
    match validate_user(&req, &pool).await {
        Ok(claim) => HttpResponse::Ok().body(format!("{:?}", claim)),
        Err(err) => err,
    }
}
