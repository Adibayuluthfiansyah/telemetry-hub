use crate::repositories::{PostgresDeviceRepository, PostgresTelemetryRepository};
use crate::services::TelemetryService;
use crate::services::device_service::DeviceService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub device_service: Arc<DeviceService<PostgresDeviceRepository>>,
    pub telemetry_service:
        Arc<TelemetryService<PostgresDeviceRepository, PostgresTelemetryRepository>>,
}
