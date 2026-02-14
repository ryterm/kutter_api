use sqlx::PgPool;
use sqlx::postgres::PgQueryResult;

use crate::models::member::Member;

pub struct Members;

impl Members {
    pub async fn create_table(pool: &PgPool) -> sqlx::Result<PgQueryResult> {
        let table = sqlx::query(
            "CREATE TABLE IF NOT EXISTS members (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id),
                community_id INTEGER NOT NULL REFERENCES communities(id),
                owner BOOLEAN NOT NULL DEFAULT false,
                admin BOOLEAN NOT NULL DEFAULT false
            )",
        )
        .execute(pool)
        .await?;
        Ok(table)
    }

    pub async fn join(
        pool: &PgPool,
        user_id: &i32,
        community_id: &i32,
        owner: &bool,
        admin: &bool,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "INSERT INTO members (
                user_id, community_id, owner, admin
            )
            VALUES (
                $1, $2, $3, $4
            )",
        )
        .bind(&user_id)
        .bind(&community_id)
        .bind(&owner)
        .bind(&admin)
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn make_admin(
        pool: &PgPool,
        user_id: &i32,
        community_id: &i32,
    ) -> sqlx::Result<PgQueryResult> {
        let query = sqlx::query(
            "UPDATE members
                SET admin = true
            WHERE (
                user_id =
                AND
                community_id = $2
            )",
        )
        .bind(&user_id)
        .bind(&community_id)
        .execute(pool)
        .await?;
        Ok(query)
    }

    pub async fn get(pool: &PgPool, user_id: &i32) -> sqlx::Result<Vec<Member>> {
        let query = sqlx::query_as::<_, Member>("SELECT * FROM members WHERE user_id = $1")
            .bind(&user_id)
            .fetch_all(pool)
            .await?;
        Ok(query)
    }
}
