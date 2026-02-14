use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;

use crate::models::community::Community;

pub struct Communities;

impl Communities {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let table = sqlx::query(
            "CREATE TABLE IF NOT EXISTS communities (
                id SERIAL PRIMARY KEY,
                name VARCHAR(50) NOT NULL,
                about TEXT,
                nsfw BOOLEAN NOT NULL
            )",
        )
        .execute(pool)
        .await?;
        Ok(table)
    }

    pub async fn create(
        pool: &PgPool,
        name: &String,
        about: &Option<String>,
        nsfw: &bool,
    ) -> sqlx::Result<Community> {
        let community = sqlx::query_as::<_, Community>(
            "INSERT INTO communities (
                name, about, nsfw
            )
            VALUES (
                $1, $2, $3
            ) RETURNING *",
        )
        .bind(&name)
        .bind(&about)
        .bind(&nsfw)
        .fetch_one(pool)
        .await?;
        Ok(community)
    }

    pub async fn verify_channel(
        pool: &PgPool,
        community_id: &i32,
        channel_id: &i32,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "SELECT * FROM channels WHERE (
                community_id = $1 AND  id = $2
            )",
        )
        .bind(&community_id)
        .bind(&channel_id)
        .execute(pool)
        .await?;
        Ok(query)
    }
}
