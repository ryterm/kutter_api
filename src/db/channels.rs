use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;

pub struct Channels;

impl Channels {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let table = sqlx::query(
            "CREATE TABLE IF NOT EXISTS channels (
                id SERIAL PRIMARY KEY,
                community INTEGER NOT NULL REFERENCES communities(id),
                name VARCHAR(100) NOT NULL,
                topic TEXT,
                hidden BOOLEAN NOT NULL DEFAULT FALSE
            )",
        )
        .execute(pool)
        .await?;
        Ok(table)
    }

    pub async fn create(
        pool: &PgPool,
        community_id: &i32,
        name: &String,
        topic: &Option<String>,
        hidden: &bool,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "INSERT INTO channels (
                community, name, topic, hidden
            ) VALUES (
                $1, $2, $3, $4
            )",
        )
        .bind(&community_id)
        .bind(&name)
        .bind(&topic)
        .bind(&hidden)
        .execute(pool)
        .await?;
        Ok(query)
    }
}
