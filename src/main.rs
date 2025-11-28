use actix_web::{App, HttpServer};

mod app;
mod db;
mod models;

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    db::init_pool().await;

    HttpServer::new(|| App::new())
        .bind(("localhost", PORT))?
        .run()
        .await
}
