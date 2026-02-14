use chrono::{DateTime, Utc};
use sqlx::{PgPool, postgres::PgQueryResult};

use crate::models::user::User;

pub struct Auth;

impl Auth {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let table = sqlx::query(
            "CREATE TABLE IF NOT EXISTS users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(30) NOT NULL UNIQUE,
                email TEXT NOT NULL UNIQUE,
                password VARCHAR(255) NOT NULL,
                pub_key BYTEA NOT NULL UNIQUE,
                verified BOOLEAN NOT NULL DEFAULT FALSE,
                biography VARCHAR(200),
                created_at TIMESTAMPTZ
            )",
        )
        .execute(pool)
        .await?;
        Ok(table)
    }

    pub async fn register(
        pool: &PgPool,
        username: String,
        email: String,
        password: String,
        pub_key: [u8; 32],
        created_at: DateTime<Utc>,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "INSERT INTO users (
                username, email, password, pub_key, created_at
            )
            VALUES (
                $1, $2, $3, $4, $5
            )",
        )
        .bind(&username)
        .bind(&email)
        .bind(&password)
        .bind(&pub_key)
        .bind(&created_at)
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn get_by_id(pool: &PgPool, id: i32) -> sqlx::Result<Option<User>> {
        let hashed = sqlx::query_as::<_, User>("SELECT * FROM users WHERE id = $1")
            .bind(&id)
            .fetch_optional(pool)
            .await?;
        Ok(hashed)
    }

    pub async fn get_by_email(pool: &PgPool, email: String) -> sqlx::Result<Option<User>> {
        let hashed = sqlx::query_as::<_, User>("SELECT * FROM users WHERE email = $1")
            .bind(&email)
            .fetch_optional(pool)
            .await?;
        Ok(hashed)
    }
}
