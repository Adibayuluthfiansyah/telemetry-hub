use async_trait::async_trait;
use server::repositories::TelemetryRepository;
use std::sync::{Arc, Mutex};
use telemetry_core::{Sample, Telemetry};

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

    async fn find_by_device(
        &self,
        device_id: uuid::Uuid,
        limit: i64,
    ) -> anyhow::Result<Vec<telemetry_core::Sample>> {
        let telemetries = self.telemetries.lock().unwrap();
        let mut samples: Vec<Sample> = telemetries
            .iter()
            .filter(|telemetry| telemetry.device_id == device_id)
            .flat_map(|telemetry| {
                telemetry.metrics.iter().map(|metric| Sample {
                    key: metric.key.clone(),
                    value: metric.value,
                    unit: metric.unit.clone(),
                    recorded_at: telemetry.recorded_at,
                })
            })
            .collect();
        samples.sort_by_key(|sample| std::cmp::Reverse(sample.recorded_at));
        samples.truncate(limit.max(0) as usize);
        Ok(samples)
    }
}
