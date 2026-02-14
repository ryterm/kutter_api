use chrono::Utc;
use sqlx::{PgPool, postgres::PgQueryResult};

use crate::models::message::ChannelMessage;

pub struct Messages;

impl Messages {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "CREATE TABLE IF NOT EXISTS messages (
                id SERIAL PRIMARY KEY,
                channel_id INTEGER NOT NULL REFERENCES channels(id),
                user_id INTEGER NOT NULL REFERENCES users(id),
                message TEXT NOT NULL,
                replied_message INTEGER REFERENCES messages(id),
                timestamp TIMESTAMPTZ,
                edited BOOLEAN NOT NULL DEFAULT false
            )",
        )
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn channel_send(
        pool: &PgPool,
        channel_id: &i32,
        user_id: &i32,
        message: &String,
        replied_message: &Option<i32>,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "INSERT INTO messages (
                channel_id, user_id, message, replied_message, timestamp
            ) VALUES (
                $1, $2, $3, $4, $5
            )",
        )
        .bind(channel_id)
        .bind(user_id)
        .bind(message)
        .bind(replied_message)
        .bind(Utc::now())
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn get_by_id(
        pool: &PgPool,
        message_id: &Option<i32>,
    ) -> sqlx::Result<Option<ChannelMessage>> {
        let message = sqlx::query_as::<_, ChannelMessage>("SELECT * FROM messages WHERE id = $1")
            .bind(&message_id)
            .fetch_optional(pool)
            .await?;
        Ok(message)
    }
}
