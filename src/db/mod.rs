mod auth;

use sqlx::postgres::PgPoolOptions;
use sqlx::{Pool, Postgres};

pub async fn init_pool() -> Pool<Postgres> {
    dotenv::dotenv().ok();
    PgPoolOptions::new()
        .max_connections(200)
        .min_connections(30)
        .connect(
            std::env::var("DATABASE_URL")
                .as_ref()
                .expect("DATABASE_URL must be set"),
        )
        .await
        .expect("Error connecting to database")
}
