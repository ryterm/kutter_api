use actix_web::{
    HttpRequest,
    cookie::{Cookie, SameSite, time::Duration},
};

pub struct CookieGenerator;

impl CookieGenerator {
    pub fn access_cookie(token: String) -> Cookie<'static> {
        Cookie::build("access", token)
            .path("/api")
            .secure(false)
            .same_site(SameSite::Lax)
            .http_only(true)
            .max_age(Duration::days(3))
            .finish()
    }

    pub fn session_cookie(token: String) -> Cookie<'static> {
        Cookie::build("session", token)
            .path("/api")
            .secure(false)
            .same_site(SameSite::Lax)
            .http_only(true)
            .permanent()
            .finish()
    }

    pub fn get_cookie(req: &HttpRequest, name: &str) -> Option<String> {
        req.cookie(&name).map(|c| c.value().to_string())
    }
}
