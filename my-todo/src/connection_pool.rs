use anyhow::Context;
use sqlx::MySqlPool;
use std::env;

pub async fn create_connection_pool() -> anyhow::Result<MySqlPool> {
    let database_url =
        env::var("DATABASE_URL").context("undefined DATABASE_URL enviornment variable")?;
    let pool = MySqlPool::connect(&database_url)
        .await
        .context("Failed to connect DB")?;
    Ok(pool)
}
