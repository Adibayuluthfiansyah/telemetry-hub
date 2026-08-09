use async_trait::async_trait;
use server::repositories::TelemetryRepository;
use std::sync::{Arc, Mutex};
use telemetry_core::Telemetry;

pub struct MockTelemetryRepository {
    should_fail: bool,
    telemetries: Arc<Mutex<Vec<Telemetry>>>,
}

impl MockTelemetryRepository {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            telemetries: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn failing() -> Self {
        Self {
            should_fail: true,
            telemetries: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait]
impl TelemetryRepository for MockTelemetryRepository {
    async fn save(&self, telemetry: &Telemetry) -> anyhow::Result<()> {
        if self.should_fail {
            anyhow::bail!("Failed to save telemetry");
        }

        self.telemetries.lock().unwrap().push(telemetry.clone());

        Ok(())
    }
}
