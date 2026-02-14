pub mod auth;
pub mod channels;
pub mod communities;
pub mod invites;
pub mod members;
pub mod messages;
pub mod tokens;

use sqlx::postgres::PgPoolOptions;
use sqlx::{PgPool, Pool, Postgres};

pub async fn init_pool() -> Pool<Postgres> {
    dotenv::dotenv().ok();
    PgPoolOptions::new()
        .max_connections(200)
        .min_connections(40)
        .connect(
            std::env::var("DATABASE_URL")
                .as_ref()
                .expect("DATABASE_URL must be set"),
        )
        .await
        .expect("Error connecting to database")
    // idk if is delay or smth i didnt see but is not panicking when i dont have postgres
}

pub async fn init_tables(pool: &PgPool) {
    auth::Auth::create_table(pool)
        .await
        .expect("Failed to create users table");
    tokens::Tokens::create_table(pool)
        .await
        .expect("Failed to create tokens table");
    communities::Communities::create_table(pool)
        .await
        .expect("Failed to create communities table");
    invites::Invites::create_table(pool)
        .await
        .expect("Failed to create invites table");
    channels::Channels::create_table(pool)
        .await
        .expect("Failed to create channels table");
    members::Members::create_table(pool)
        .await
        .expect("Failed to create members table");
    messages::Messages::create_table(pool)
        .await
        .expect("Failed to create messages table");
}
