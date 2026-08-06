use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct TelemetryResponse {
    pub success: bool,
    pub message: String,
}
