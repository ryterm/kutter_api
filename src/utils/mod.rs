pub mod cookies;
pub mod invites;
pub mod tokens;

use actix_web::{HttpRequest, HttpResponse};
use sqlx::PgPool;

use crate::db::{auth::Auth};

pub async fn validate_user(
    req: &HttpRequest,
    pool: &PgPool,
) -> Result<tokens::AccessClaim, HttpResponse> {
    if let Some(access_cookie) = cookies::CookieGenerator::get_cookie(&req, "access") {
        if let Ok(access) = tokens::AccessClaim::verify(access_cookie) {
            return Ok(access);
        }
    }
    if let Some(session_cookie) = cookies::CookieGenerator::get_cookie(&req, "session") {
        if let Ok(session) = tokens::SessionClaim::verify(session_cookie) {
            let user = match Auth::get_by_id(pool, session.sub).await {
                Ok(Some(user)) => user,
                Ok(None) => return Err(HttpResponse::NotFound().body("User not found")),
                Err(_) => return Err(HttpResponse::InternalServerError().body("Error getting user")),
            };
            let token = tokens::AccessClaim::generate(user.id, user.username);
            let cookie = cookies::CookieGenerator::access_cookie(token);
            return Err(HttpResponse::Created().cookie(cookie).body("New cookie"));
        }
    }
    Err(HttpResponse::BadRequest().body("Invalid or non-existent tokens"))
}
