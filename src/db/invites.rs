use sqlx::{PgPool, postgres::PgQueryResult};

pub struct Invites;

impl Invites {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "CREATE TABLE IF NOT EXISTS invites (
                id SERIAL PRIMARY KEY,
                created_by INTEGER NOT NULL REFERENCES users(id),
                community INTEGER NOT NULL REFERENCES communities(id),
                token TEXT NOT NULL,
                revoked BOOLEAN NOT NULL DEFAULT false,
                revoked_at TIMESTAMPTZ
            )",
        )
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn create(
        pool: &PgPool,
        created_by: &i32,
        community_id: &i32,
        token: &String,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "INSERT INTO invites (
                created_by, community, token
            )
            VALUES (
                $1, $2, $3
            )",
        )
        .bind(&created_by)
        .bind(&community_id)
        .bind(&token)
        .execute(pool)
        .await?;
        Ok(query)
    }
}
