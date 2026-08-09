use server::{
    config::Config,
    database,
    repositories::postgres::{PostgresDeviceRepository, PostgresTelemetryRepository},
    router,
    services::{DeviceService, TelemetryService},
    state::AppState,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::net::TcpListener;
use tower_http::trace::TraceLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
#[tokio::main]

async fn main() {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            "server=debug,tower_http=debug",
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();
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
    let telemetry_repository = PostgresTelemetryRepository::new(pool.clone());
    let telemetry_service = TelemetryService::new(
        PostgresDeviceRepository::new(pool.clone()),
        telemetry_repository,
    );
    let address: SocketAddr = format!("{}:{}", config.app_host, config.app_port)
        .parse()
        .expect("Invalid bind address in configuration");
    let state = AppState {
        device_service: Arc::new(device_service),
        telemetry_service: Arc::new(telemetry_service),
    };
    let app = router::create_router()
        .with_state(state)
        .layer(TraceLayer::new_for_http());

    println!("Server Running At http://{}", address);
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server Error");
}
