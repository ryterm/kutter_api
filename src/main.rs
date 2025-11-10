mod utils;

use actix_web::{App, HttpResponse, HttpServer, web};

const PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
	utils::db_connection::db_connection().await;

    HttpServer::new(|| {
        App::new()
    })
    .bind(("localhost", PORT))?
    .run()
    .await
}
