use crate::repositories::DeviceRepository;
use crate::repositories::postgres::models::DeviceRecord;
use async_trait::async_trait;
use sqlx::PgPool;
use sqlx::query_as;
use telemetry_core::Device;

pub struct PostgresDeviceRepository {
    pool: PgPool,
}

impl PostgresDeviceRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl DeviceRepository for PostgresDeviceRepository {
    async fn save(&self, device: &Device) -> anyhow::Result<()> {
        sqlx::query(
            r#"
            INSERT INTO devices (
                id,
                code,
                name,
                device_type,
                status,
                created_at,
                updated_at,
            )
            VALUES ($1, $2, $3, $4, $5, $6, $7)
            "#,
        )
        .bind(device.id)
        .bind(&device.code)
        .bind(&device.name)
        .bind(device.device_type.to_string())
        .bind(device.status.to_string())
        .bind(device.created_at)
        .bind(device.updated_at)
        .execute(&self.pool)
        .await?;
        Ok(())
    }

    async fn find_by_code(&self, code: &str) -> anyhow::Result<Option<Device>> {
        let record = sqlx::query_as::<_, DeviceRecord>(
            r#"
            SELECT id, code, name, device_type, status, created_at, updated_at FROM devices WHERE code = $1
            "#,
        ).bind(code).fetch_optional(&self.pool).await?;
        Ok(record.map(Into::into))
    }
}
