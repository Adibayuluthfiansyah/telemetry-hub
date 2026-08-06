use chrono::{DateTime, Utc};
use uuid::Uuid;

use crate::enums::AlertSeverity;
use crate::metric::Metric;

#[derive(Debug, Clone)]
pub struct Telemetry {
    pub id: Uuid,
    pub device_id: Uuid,
    pub temperature: f32,
    pub humidity: f32,
    pub metric: Vec<Metric>,
    pub created_at: DateTime<Utc>,
    pub severity: AlertSeverity,
}

impl Telemetry {
    pub fn new(
        id: Uuid,
        device_id: Uuid,
        temperature: f32,
        humidity: f32,
        created_at: DateTime<Utc>,
        severity: AlertSeverity,
        metric: Vec<Metric>,
    ) -> Self {
        Self {
            id,
            device_id,
            temperature,
            humidity,
            created_at,
            severity,
            metric,
        }
    }
}
