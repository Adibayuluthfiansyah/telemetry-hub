use async_trait::async_trait;
use telemetry_core::Telemetry;
use uuid::Uuid;

#[async_trait]
pub trait TelemetryRepository: Send + Sync {
    async fn save(&self, telemetry: &Telemetry) -> anyhow::Result<()>;
    async fn find_latest(&self, device_id: Uuid) -> anyhow::Result<Option<Telemetry>>;
}
