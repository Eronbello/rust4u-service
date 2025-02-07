use anyhow::Result;
use sqlx::{Pool, Postgres};

pub async fn create_db_pool(database_url: &str) -> Result<Pool<Postgres>> {
    let pool = Pool::<Postgres>::connect(database_url).await?;
    Ok(pool)
}

pub mod issue_repository_sql;
pub mod project_repository_sql;
pub mod user_repository_sql;
