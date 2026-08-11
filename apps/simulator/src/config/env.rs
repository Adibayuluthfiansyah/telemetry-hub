use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub interval_ms: u64,
    pub server_url: String,
    pub device_code: String,
    pub device_name: String,
}

impl Config {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        dotenvy::dotenv().ok();
        Self::from_env()
    }

    fn from_env() -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            interval_ms: env::var("SIMULATOR_INTERVAL_MS")
                .unwrap_or_else(|_| "1000".to_string())
                .parse()?,
            server_url: env::var("SIMULATOR_SERVER_URL")
                .unwrap_or_else(|_| "http://localhost:3000".to_string()),
            device_code: env::var("SIMULATOR_DEVICE_CODE")
                .unwrap_or_else(|_| "SIMULATOR-001".to_string()),
            device_name: env::var("SIMULATOR_DEVICE_NAME")
                .unwrap_or_else(|_| "Simulator Device".to_string()),
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn config_should_read_env_vars_and_fall_back_to_defaults() {
        unsafe {
            std::env::set_var("SIMULATOR_INTERVAL_MS", "500");
            std::env::set_var("SIMULATOR_SERVER_URL", "http://example.test");
            std::env::set_var("SIMULATOR_DEVICE_CODE", "TEST-DEV");
            std::env::set_var("SIMULATOR_DEVICE_NAME", "Test Device");
        }

        let config = Config::from_env().unwrap();
        assert_eq!(config.interval_ms, 500);
        assert_eq!(config.server_url, "http://example.test");
        assert_eq!(config.device_code, "TEST-DEV");
        assert_eq!(config.device_name, "Test Device");

        for var in [
            "SIMULATOR_INTERVAL_MS",
            "SIMULATOR_SERVER_URL",
            "SIMULATOR_DEVICE_CODE",
            "SIMULATOR_DEVICE_NAME",
        ] {
            unsafe {
                std::env::remove_var(var);
            }
        }

        let config = Config::from_env().unwrap();
        assert_eq!(config.interval_ms, 1000);
        assert_eq!(config.server_url, "http://localhost:3000");
        assert_eq!(config.device_code, "SIMULATOR-001");
        assert_eq!(config.device_name, "Simulator Device");
    }
}
