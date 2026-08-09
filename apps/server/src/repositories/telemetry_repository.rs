use async_trait::async_trait;
use telemetry_core::Telemetry;

#[async_trait]
pub trait TelemetryRepository: Send + Sync {
    async fn save(&self, telemetry: &Telemetry) -> anyhow::Result<()>;
}
