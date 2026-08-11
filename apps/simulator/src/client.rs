use crate::dto::device::CreateDeviceRequest;
use crate::dto::telemetry::TelemetryRequest;

pub struct SimulatorClient {
    client: reqwest::Client,
    server_url: String,
}

#[derive(Debug)]
pub enum ClientError {
    DeviceNotFound,
    Request(reqwest::Error),
    Server(reqwest::StatusCode),
}

impl SimulatorClient {
    pub fn new(server_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            server_url,
        }
    }
    pub async fn register_device(
        &self,
        code: &str,
        name: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let request = CreateDeviceRequest {
            code: code.to_string(),
            name: name.to_string(),
            device_type: "SIMULATOR".to_string(),
        };
        let url = format!("{}/api/v1/devices", self.server_url);
        let response = self.client.post(&url).json(&request).send().await?;
        let status = response.status();
        if status == reqwest::StatusCode::CREATED {
            return Ok(());
        }
        if status == reqwest::StatusCode::CONFLICT {
            return Ok(());
        }
        Err(std::io::Error::other(format!("Failed to register device: {}", status)).into())
    }

    pub async fn send_telemetry(&self, telemetry: &TelemetryRequest) -> Result<(), ClientError> {
        let url = format!("{}/api/v1/telemetry", self.server_url);
        let response = self
            .client
            .post(&url)
            .json(telemetry)
            .send()
            .await
            .map_err(ClientError::Request)?;
        let status = response.status();
        if status == reqwest::StatusCode::CREATED {
            return Ok(());
        }
        if status == reqwest::StatusCode::NOT_FOUND {
            return Err(ClientError::DeviceNotFound);
        }
        Err(ClientError::Server(status))
    }
}
