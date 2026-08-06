use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MetricRequest {
    pub key: String,
    pub value: f64,
    pub unit: String,
}

#[derive(Debug, Deserialize)]
pub struct TelemetryRequest {
    pub device_code: String,
    pub metrics: Vec<MetricRequest>,
}
