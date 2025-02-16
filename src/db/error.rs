use std::env;

#[derive(Debug)]
pub enum PoolError {
    DatabaseError(sqlx::Error),
    ConfigError(env::VarError),
}
