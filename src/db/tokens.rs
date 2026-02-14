use chrono::Utc;
use sqlx::{PgPool, postgres::PgQueryResult};

pub struct Tokens;

impl Tokens {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let table = sqlx::query(
            "CREATE TABLE IF NOT EXISTS tokens (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id),
                created_at TIMESTAMPTZ,
                last_update TIMESTAMPTZ,
                revoked BOOLEAN NOT NULL DEFAULT FALSE,
                revoked_at TIMESTAMPTZ
            )",
        )
        .execute(pool)
        .await?;
        Ok(table)
    }

    pub async fn insert(pool: &PgPool, user_id: i32) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "INSERT INTO tokens (
                user_id, created_at, last_update
            )
            VALUES (
                $1, $2, $3
            )",
        )
        .bind(&user_id)
        .bind(Utc::now())
        .bind(Utc::now())
        .execute(pool)
        .await?;
        Ok(query)
    }
}
