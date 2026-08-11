use serde::Serialize;
#[derive(Debug, Serialize)]
pub struct CreateDeviceRequest {
    pub code: String,
    pub name: String,
    pub device_type: String,
}
