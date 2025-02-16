use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct MachineLog {
    pub id: i32,
    pub machine_id: String,
    pub timestamp: DateTime<Utc>,
    pub raw_data: serde_json::Value,
    pub created_at: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Metric {
    pub id: i32,
    pub log_id: i32,
    pub metric_name: String,
    pub metric_value: f64
}

impl MachineLog {
    // Create (insert)
    pub async fn create(
        pool: &PgPool,
        machine_id: String,
        raw_data: serde_json::Value
    ) -> Result<Self, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            INSERT INTO machine_logs (machine_id, timestamp, raw_data)
            VALUES ($1, $2, $3) 
            RETURNING id, machine_id, timestamp as "timestamp!: DateTime<Utc>", 
            raw_data, created_at as "created_at!: DateTime<Utc>"
            "#,
            machine_id, 
            Utc::now(),
            raw_data
        )
        .fetch_one(pool)
        .await
    }

    // Read 
    pub async fn find_by_id(pool: &PgPool, id: i32) -> Result<Option<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, machine_id, timestamp as "timestamp!: DateTime<Utc>",
                   raw_data, created_at as "created_at!: DateTime<Utc>"
            FROM machine_logs WHERE id = $1
            "#,
            id
        )
        .fetch_optional(pool)
        .await
    }

    // Query methods specific to your domain 
    pub async fn find_by_machine_id(
        pool: &PgPool,
        machine_id: &str
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id, machine_id, timestamp as "timestamp!: DateTime<Utc>",
                   raw_data, created_at as "created_at!: DateTime<Utc>"
            FROM machine_logs WHERE machine_id = $1
            "#,
            machine_id
        )
        .fetch_all(pool)
        .await
    }

    // Get all metrics for this log
    pub async fn metrics(&self, pool: &PgPool) -> Result<Vec<Metric>, sqlx::Error> {
        Metric::find_by_log_id(pool, self.id).await
    }
}

impl Metric {
    pub async fn find_by_log_id(pool: &PgPool, log_id: i32) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id as "id!", log_id as "log_id!", 
                   metric_name as "metric_name!", metric_value as "metric_value!"
            FROM metrics WHERE log_id = $1
            "#,
            log_id
        )
        .fetch_all(pool)
        .await
    }

    pub async fn find_above_threshold(
        pool: &PgPool,
        metric_name: &str,
        threshold: f64
    ) -> Result<Vec<Self>, sqlx::Error> {
        sqlx::query_as!(
            Self,
            r#"
            SELECT id as "id!", log_id as "log_id!", 
                   metric_name as "metric_name!", metric_value as "metric_value!"
            FROM metrics 
            WHERE metric_name = $1 AND metric_value > $2
            "#,
            metric_name,
            threshold
        )
        .fetch_all(pool)
        .await
    }
}