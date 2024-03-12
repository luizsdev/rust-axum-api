use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
pub async fn pool() -> Pool<Postgres> {
    dotenv().ok();
    let durl = std::env::var("DATABASE_URL").expect("database url must be set.");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&durl)
        .await
        .expect("Unable to connect to database");

    return pool;
}
