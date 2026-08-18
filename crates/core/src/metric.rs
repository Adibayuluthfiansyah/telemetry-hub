use serde::Serialize;
#[derive(Debug, Clone, Serialize)]
pub struct Metric {
    pub key: String,
    pub value: f64,
    pub unit: String,
}
