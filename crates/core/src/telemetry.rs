#[derive(Debug, Clone)]
pub struct Telemetry {
    pub device_id: String,
    pub temperature: f32,
    pub humidity: f32,
    pub voltage: f32,
    pub current: f32,
    pub rpm: u32,
    pub timestamp: u64,
}

impl Telemetry {
    pub fn new(
        device_id: String,
        temperature: f32,
        humidity: f32,
        voltage: f32,
        current: f32,
        rpm: u32,
        timestamp: u64,
    ) -> Self {
        Self {
            device_id,
            temperature,
            humidity,
            voltage,
            current,
            rpm,
            timestamp,
        }
    }
}
