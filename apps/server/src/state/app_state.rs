use crate::repositories::PostgresDeviceRepository;
use crate::services::device_service::DeviceService;
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub device_service: Arc<DeviceService<PostgresDeviceRepository>>,
}
