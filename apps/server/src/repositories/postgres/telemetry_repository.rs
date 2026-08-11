use crate::repositories::TelemetryRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use telemetry_core::{Sample, Telemetry};
use uuid::Uuid;

pub struct PostgresTelemetryRepository {
    pool: PgPool,
}

impl PostgresTelemetryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}
#[derive(sqlx::FromRow)]
struct SampleRecord {
    key: String,
    value: f64,
    unit: String,
    recorded_at: chrono::DateTime<chrono::Utc>,
}

#[async_trait]
impl TelemetryRepository for PostgresTelemetryRepository {
    async fn save(&self, telemetry: &Telemetry) -> anyhow::Result<()> {
        let mut transaction = self.pool.begin().await?;
        for metric in &telemetry.metrics {
            sqlx::query(
                r#"
                    INSERT INTO telemetry(
                    id,
                    device_id,
                    key,
                    value,
                    unit,
                    recorded_at
                    )
                    VALUES (
                    $1,
                    $2,
                    $3,
                    $4,
                    $5,
                    $6
                    )
                "#,
            )
            .bind(Uuid::new_v4())
            .bind(telemetry.device_id)
            .bind(&metric.key)
            .bind(metric.value)
            .bind(&metric.unit)
            .bind(telemetry.recorded_at)
            .execute(&mut *transaction)
            .await?;
        }
        transaction.commit().await?;
        Ok(())
    }

    async fn find_by_device(&self, device_id: Uuid, limit: i64) -> anyhow::Result<Vec<Sample>> {
        let rows = sqlx::query_as::<_, SampleRecord>(
            r#"
            SELECT key, value, unit, recorded_at
            FROM telemetry
            WHERE device_id = $1
            ORDER BY recorded_at DESC
            LIMIT $2
            "#,
        )
        .bind(device_id)
        .bind(limit)
        .fetch_all(&self.pool)
        .await?;

        Ok(rows
            .into_iter()
            .map(|row| Sample {
                key: row.key,
                value: row.value,
                unit: row.unit,
                recorded_at: row.recorded_at,
            })
            .collect())
    }
}
