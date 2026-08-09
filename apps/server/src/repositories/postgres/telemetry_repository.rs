use crate::repositories::TelemetryRepository;
use async_trait::async_trait;
use sqlx::PgPool;
use telemetry_core::Telemetry;
use uuid::Uuid;
pub struct PostgresTelemetryRepository {
    pool: PgPool,
}

impl PostgresTelemetryRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
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
}
