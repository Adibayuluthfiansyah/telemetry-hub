use crate::{
    dto::TelemetryRequest,
    repositories::{DeviceRepository, TelemetryRepository},
};
use chrono::Utc;
use telemetry_core::{Sample, Telemetry, metric::Metric};
use uuid::Uuid;

pub struct TelemetryService<D, T>
where
    D: DeviceRepository,
    T: TelemetryRepository,
{
    device_repository: D,
    telemetry_repository: T,
}
impl<D, T> TelemetryService<D, T>
where
    D: DeviceRepository,
    T: TelemetryRepository,
{
    pub fn new(device_repository: D, telemetry_repository: T) -> Self {
        Self {
            device_repository,
            telemetry_repository,
        }
    }
    pub async fn create_telemetry(&self, request: TelemetryRequest) -> anyhow::Result<Telemetry> {
        if request.metrics.is_empty() {
            anyhow::bail!("Metrics cannot be empty");
        }
        let device = self
            .device_repository
            .find_by_code(&request.device_code)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Device with code {} not found", request.device_code))?;

        let metrics = request
            .metrics
            .into_iter()
            .map(|metric| Metric {
                key: metric.key,
                value: metric.value,
                unit: metric.unit,
            })
            .collect();

        let now = Utc::now();

        let telemetry = Telemetry {
            id: Uuid::new_v4(),
            device_id: device.id,
            metrics,
            recorded_at: now,
        };

        self.telemetry_repository.save(&telemetry).await?;

        Ok(telemetry)
    }
    pub async fn get_telemetry(&self, device_id: Uuid, limit: i64) -> anyhow::Result<Vec<Sample>> {
        let limit = limit.clamp(1, 1000);
        self.device_repository
            .find_by_id(device_id)
            .await?
            .ok_or_else(|| anyhow::anyhow!("Device with id {} not found", device_id))?;
        self.telemetry_repository
            .find_by_device(device_id, limit)
            .await
    }
}
