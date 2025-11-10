use sqlx::postgres::PgPoolOptions;

pub async fn db_connection() -> sqlx::Pool<sqlx::Postgres> {
	let database_url = dotenv::var("DATABASE_URL").expect("DATABASE_URL must be set");
	PgPoolOptions::new()
		.max_connections(200)
		.min_connections(20)
		.connect(&database_url)
		.await
		.expect("Failed to connect to database")
}
