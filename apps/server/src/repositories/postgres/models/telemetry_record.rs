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

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::SubsecRound;

    #[test]
    fn from_telemetry_record_to_metric() {
        let now = Utc::now().trunc_subsecs(3);
        let record = TelemetryRecord {
            id: Uuid::new_v4(),
            key: "temperature".to_string(),
            value: 25.5,
            unit: "celsius".to_string(),
            recorded_at: now,
            device_id: Uuid::new_v4(),
        };
        let metric: Metric = record.into();
        assert_eq!(metric.key, "temperature");
        assert_eq!(metric.value, 25.5);
        assert_eq!(metric.unit, "celsius");
    }

    #[test]
    fn from_telemetry_record_to_sample() {
        let now = Utc::now().trunc_subsecs(3);
        let record = TelemetryRecord {
            id: Uuid::new_v4(),
            key: "humidity".to_string(),
            value: 60.0,
            unit: "percent".to_string(),
            recorded_at: now,
            device_id: Uuid::new_v4(),
        };
        let sample: Sample = record.into();
        assert_eq!(sample.key, "humidity");
        assert_eq!(sample.value, 60.0);
        assert_eq!(sample.unit, "percent");
        assert_eq!(sample.recorded_at.trunc_subsecs(3), now);
    }
}
