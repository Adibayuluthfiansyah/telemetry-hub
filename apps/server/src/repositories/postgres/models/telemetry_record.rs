use chrono::{DateTime, Utc};
use sqlx::FromRow;
use telemetry_core::{Sample, metric::Metric};
use uuid::Uuid;

#[derive(Debug, FromRow)]
pub struct TelemetryRecord {
    pub id: Uuid,
    pub key: String,
    pub value: f64,
    pub unit: String,
    pub recorded_at: DateTime<Utc>,
    pub device_id: Uuid,
}

impl From<TelemetryRecord> for Metric {
    fn from(record: TelemetryRecord) -> Self {
        Self {
            key: record.key,
            value: record.value,
            unit: record.unit,
        }
    }
}

impl From<TelemetryRecord> for Sample {
    fn from(record: TelemetryRecord) -> Self {
        Self {
            key: record.key,
            value: record.value,
            unit: record.unit,
            recorded_at: record.recorded_at,
        }
    }
}
