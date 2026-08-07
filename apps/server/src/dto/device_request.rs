use serde::Deserialize;
use telemetry_core::DeviceType;

#[derive(Debug, Deserialize)]
pub struct CreateDeviceRequest {
    pub code: String,
    pub name: String,
    pub device_type: DeviceType,
}
