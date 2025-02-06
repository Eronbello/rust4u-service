use sqlx::{Pool, Postgres};
use anyhow::Result;

pub async fn create_db_pool(database_url: &str) -> Result<Pool<Postgres>> {
    let pool = Pool::<Postgres>::connect(database_url).await?;
    Ok(pool)
}

pub mod user_repository_sql;
