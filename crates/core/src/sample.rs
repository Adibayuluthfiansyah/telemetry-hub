use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Sample {
    pub key: String,
    pub value: f64,
    pub unit: String,
    pub recorded_at: DateTime<Utc>,
}
