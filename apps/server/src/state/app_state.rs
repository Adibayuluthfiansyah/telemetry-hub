use crate::events::EventPublisher;
use crate::repositories::{PostgresDeviceRepository, PostgresTelemetryRepository};
use crate::services::TelemetryService;
use crate::services::device_service::DeviceService;
use sqlx::PgPool;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub device_service: Arc<DeviceService<PostgresDeviceRepository>>,
    pub telemetry_service:
        Arc<TelemetryService<PostgresDeviceRepository, PostgresTelemetryRepository>>,
    pub event_publisher: Arc<dyn EventPublisher>,
}
