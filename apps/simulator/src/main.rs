mod client;
mod config;
mod dto;
mod generator;

use client::{ClientError, SimulatorClient};
use config::Config;
use dto::TelemetryRequest;
use generator::MetricGenerator;

#[tokio::main]
async fn main() {
    let config = Config::load().expect("Failed to load simulator config");
    let client = SimulatorClient::new(config.server_url.clone());
    let mut generator = MetricGenerator::new();
    client
        .register_device(&config.device_code, &config.device_name)
        .await
        .expect("Failed to register simulator device");
    let metrics = generator.generate_metrics();
    let telemetry = TelemetryRequest {
        device_code: config.device_code.clone(),
        metrics,
    };
    client
        .send_telemetry(&telemetry)
        .await
        .expect("Failed to send telemetry");

    let mut interval = tokio::time::interval(std::time::Duration::from_millis(config.interval_ms));

    let ctrl_c = tokio::signal::ctrl_c();
    tokio::pin!(ctrl_c);
    loop {
        tokio::select! {
            _ = interval.tick() => {
                let metrics = generator.generate_metrics();
                let telemetry = TelemetryRequest {
                    device_code: config.device_code.clone(),
                    metrics
                };
                    match client.send_telemetry(&telemetry).await {
                Ok(()) => {
                    println!("Telemetry sent successfully");
                }
                Err(ClientError::DeviceNotFound) => {
                    println!("Device not found, registering again...");
                    if let Err(error) = client
                        .register_device(
                            &config.device_code,
                            &config.device_name,
                        )
                        .await
                    {
                        eprintln!("Failed to re-register device: {error}");
                    }
                }
                Err(ClientError::Request(error)) => {
                    eprintln!("Connection error: {error}");
                }
                Err(ClientError::Server(status)) => {
                    eprintln!("Server returned error: {status}");
                }
            }
            }
             _ = &mut ctrl_c => {
                 println!("Shutting down simulator");
                 break;
             }
        }
    }
}
