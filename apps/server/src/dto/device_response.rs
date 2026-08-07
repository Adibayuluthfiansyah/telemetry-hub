use serde::Serialize;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct DeviceResponse {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub status: String,
    pub device_type: String,
}
