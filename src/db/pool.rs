use std::time::Duration;
use std::env;
use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use super::error::PoolError;

pub async fn create_pool() -> Result<PgPool, PoolError> {
    let database_url = env::var("DATABASE_URL")
        .map_err(PoolError::ConfigError)?;

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
        .map_err(PoolError::DatabaseError)
}


