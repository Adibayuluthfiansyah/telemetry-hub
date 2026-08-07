mod app;
mod config;
mod database;
mod dto;
mod handlers;
mod repositories;
mod router;
mod services;
mod state;

use crate::config::Config;
use crate::repositories::postgres::device_repository::PostgresDeviceRepository;
use crate::services::device_service::DeviceService;
use crate::state::AppState;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
#[tokio::main]

async fn main() {
    let config = Config::load().expect("Failed to load config");
    println!("{:#?}", config);
    let pool = database::connect(&config)
        .await
        .expect("Failed to connect to database");
    database::run(&pool)
        .await
        .expect("Failed to run database migrations");
    let device_repository = PostgresDeviceRepository::new(pool.clone());
    let device_service = DeviceService::new(device_repository);
    let address: SocketAddr = format!("{}:{}", config.app_host, config.app_port)
        .parse()
        .expect("Invalid bind address in configuration");
    let state = AppState {
        device_service: Arc::new(device_service),
    };
    let app = app::create_app(state);

    println!("Server Running At http://{}", address);
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server Error");
}
