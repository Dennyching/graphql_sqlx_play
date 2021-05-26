mod graphql;
mod result;
mod todo;
mod web;

use dotenv::dotenv;
use sqlx::{PgPool, Row};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    let pool = PgPool::connect("postgres://postgres:123456@localhost:5432/postgres")
        .await
        .expect("Failed to create pool.");

    sqlx::query!("CREATE TABLE IF NOT EXISTS todo (id TEXT PRIMARY KEY NOT NULL, body TEXT NOT NULL, complete BOOLEAN NOT NULL) ")
            .execute(&pool)
            .await?;
    
    web::start(pool).await;
    Ok(())
}
