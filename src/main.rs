use actix_web::{App, HttpServer, web};
use tokio::sync::broadcast;

use crate::models::actions::OutgoingMessage;

mod app;
mod db;
mod models;
mod utils;

const PORT: u16 = 8001;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let pool = db::init_pool().await;
    let (tx, _) = broadcast::channel::<OutgoingMessage>(1000);

    db::init_tables(&pool).await;

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .app_data(web::Data::new(tx.clone()))
            .service(web::scope("/api").configure(app::routes))
    })
    .bind(("localhost", PORT))?
    .run()
    .await
}
