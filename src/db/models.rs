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
    pub async fn create_with_metrics(
        pool: &PgPool,
        machine_id: String,
        raw_data: serde_json::Value,
        metrics: Vec<(String, f64)>
    ) -> Result<(Self, Vec<Metric>), sqlx::Error> {
        let mut tx = pool.begin().await?;

        let log = sqlx::query_as!(
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
        .fetch_one(&mut *tx)
        .await?;

        let (ids, names, values): (Vec<i32>, Vec<String>, Vec<f64>) = metrics 
        .into_iter()
        .map(|(name, value)| (log.id, name, value))
        .fold(
            (Vec::new(), Vec::new(), Vec::new()),
            |(mut ids, mut names, mut vals), (id, name, val)| {
                ids.push(id);
                names.push(name);
                vals.push(val);
                (ids, names, vals)
            }
        );

        let stored_metrics = if !ids.is_empty() {
            sqlx::query_as!(
                Metric,
                r#"
                INSERT INTO metrics (log_id, metric_name, metric_value)
                SELECT * FROM UNNEST($1::int[], $2::text[], $3::float8[])
                RETURNING id as "id!", log_id as "log_id!",
                metric_name as "metric_name!", metric_value as "metric_value!"
                "#,
                &ids,
                &names,
                &values
            )
            .fetch_all(&mut *tx)
            .await? 
        } else {
            vec![]
        };

        tx.commit().await?;

        Ok((log, stored_metrics))
    }

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