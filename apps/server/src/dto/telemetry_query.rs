use serde::{Deserialize, Serialize};
use telemetry_core::Sample;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct TelemetryQuery {
    pub device_id: Uuid,
    pub limit: Option<i64>,
}

#[derive(Debug, Serialize)]
pub struct TelemetrySampleResponse {
    pub key: String,
    pub value: f64,
    pub unit: String,
    pub recorded_at: chrono::DateTime<chrono::Utc>,
}

impl From<Sample> for TelemetrySampleResponse {
    fn from(sample: Sample) -> Self {
        Self {
            key: sample.key,
            value: sample.value,
            unit: sample.unit,
            recorded_at: sample.recorded_at,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct TelemetryQueryResponse {
    pub device_id: Uuid,
    pub count: usize,
    pub samples: Vec<TelemetrySampleResponse>,
}
