use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MetricRequest {
    pub key: String,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Serialize)]
pub struct TelemetryRequest {
    pub device_code: String,
    pub metrics: Vec<MetricRequest>,
}
