use actix_web::web;

pub mod auth;
pub mod channels;
pub mod communities;
pub mod websocket;

pub fn routes(app: &mut web::ServiceConfig) {
    app.service(
        web::scope("/auth")
            .service(web::resource("register").route(web::post().to(auth::register)))
            .service(web::resource("login").route(web::post().to(auth::login)))
            .service(web::resource("token_test").route(web::get().to(auth::token_test))),
    )
    .service(
        web::scope("/community")
            .service(web::resource("create").route(web::post().to(communities::create_community)))
            .service(
                web::resource("channel/message").route(web::get().to(websocket::messages_handler)),
            ),
    )
    .service(web::resource("/ws").route(web::get().to(websocket::messages_handler)));
}
