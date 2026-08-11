use crate::dto::MetricRequest;

pub struct MetricGenerator {
    step: f64,
    battery: f64,
}

impl MetricGenerator {
    pub fn new() -> Self {
        Self {
            step: 0.0,
            battery: 100.0,
        }
    }
    pub fn generate_temperature(&mut self) -> f64 {
        let normalized = (self.step.sin() + 1.0) / 2.0;
        20.0 + normalized * (35.0 - 20.0)
    }
    pub fn generate_humidity(&mut self) -> f64 {
        let normalized = (self.step.sin() + 1.0) / 2.0;
        40.0 + normalized * (80.0 - 40.0)
    }
    pub fn generate_battery(&mut self) -> f64 {
        let battery = self.battery;
        self.battery -= 0.01;
        if self.battery < 20.0 {
            self.battery = 100.0;
        }
        battery
    }
    pub fn generate_metrics(&mut self) -> Vec<MetricRequest> {
        let temperature = self.generate_temperature();
        let humidity = self.generate_humidity();
        let battery = self.generate_battery();
        self.step += 0.1;
        vec![
            MetricRequest {
                key: "temperature".to_string(),
                value: temperature,
                unit: "celsius".to_string(),
            },
            MetricRequest {
                key: "humidity".to_string(),
                value: humidity,
                unit: "percent".to_string(),
            },
            MetricRequest {
                key: "battery".to_string(),
                value: battery,
                unit: "percent".to_string(),
            },
        ]
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn generate_temperature_should_stay_in_range() {
        let mut generator = MetricGenerator::new();
        for _ in 0..100 {
            let temperature = generator.generate_temperature();
            assert!(
                (20.0..=35.0).contains(&temperature),
                "temperature out of range : {}",
                temperature
            )
        }
    }
    #[test]
    fn generate_humidity_should_stay_in_range() {
        let mut generator = MetricGenerator::new();
        for _ in 0..100 {
            let humidity = generator.generate_humidity();
            assert!(
                (40.0..=80.0).contains(&humidity),
                "humidity out of range : {}",
                humidity
            )
        }
    }
    #[test]
    fn generate_battery_should_stay_in_range() {
        let mut generator = MetricGenerator::new();
        for _ in 0..1000 {
            let battery = generator.generate_battery();
            assert!(
                (20.0..=100.0).contains(&battery),
                "battery out of range : {}",
                battery
            )
        }
    }
    #[test]
    fn generate_battery_should_decrease() {
        let mut generator = MetricGenerator::new();
        let first = generator.generate_battery();
        let second = generator.generate_battery();
        assert!(
            second < first,
            "battery should decrease : {} < {}",
            second,
            first
        )
    }
    #[test]
    fn generate_metrics_should_return_three_metrics() {
        let mut generator = MetricGenerator::new();
        let metrics = generator.generate_metrics();
        assert_eq!(metrics.len(), 3);
        assert_eq!(metrics[0].key, "temperature");
        assert_eq!(metrics[0].unit, "celsius");
        assert_eq!(metrics[1].key, "humidity");
        assert_eq!(metrics[1].unit, "percent");
        assert_eq!(metrics[2].key, "battery");
        assert_eq!(metrics[2].unit, "percent");
    }
}
