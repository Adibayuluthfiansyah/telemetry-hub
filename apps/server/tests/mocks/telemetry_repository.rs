use async_trait::async_trait;
use server::repositories::TelemetryRepository;
use std::sync::{Arc, Mutex};
use telemetry_core::{Sample, Telemetry};

#[derive(Clone)]
pub struct MockTelemetryRepository {
    should_fail: bool,
    telemetries: Arc<Mutex<Vec<Telemetry>>>,
    last_limit: Arc<Mutex<Option<i64>>>,
}

impl MockTelemetryRepository {
    pub fn new() -> Self {
        Self {
            should_fail: false,
            telemetries: Arc::new(Mutex::new(Vec::new())),
            last_limit: Arc::new(Mutex::new(None)),
        }
    }

    pub fn failing() -> Self {
        Self {
            should_fail: true,
            telemetries: Arc::new(Mutex::new(Vec::new())),
            last_limit: Arc::new(Mutex::new(None)),
        }
    }

    pub fn last_limit(&self) -> Option<i64> {
        *self.last_limit.lock().unwrap()
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
        if self.should_fail {
            anyhow::bail!("Failed to save telemetry");
        }
        // Spy: record clamped limit passed from service (prod uses clamp 1..1000, fake uses max(0))
        *self.last_limit.lock().unwrap() = Some(limit);
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
        // NOTE: fake uses max(0) while prod TelemetryService clamps to 1..1000 — divergence documented per #55
        samples.truncate(limit.max(0) as usize);
        Ok(samples)
    }
}
