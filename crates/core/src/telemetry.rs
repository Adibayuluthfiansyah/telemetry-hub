use crate::metric::Metric;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub id: Uuid,
    pub device_id: Uuid,
    pub metrics: Vec<Metric>,
    pub recorded_at: DateTime<Utc>,
}

impl Telemetry {
    pub fn new(
        id: Uuid,
        device_id: Uuid,
        recorded_at: DateTime<Utc>,
        metrics: Vec<Metric>,
    ) -> Self {
        Self {
            id,
            device_id,
            recorded_at,
            metrics,
        }
    }
}
