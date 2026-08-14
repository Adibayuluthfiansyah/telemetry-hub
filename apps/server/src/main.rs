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
    if let Err(error) = run().await {
        tracing::error!(error = ?error, "Failed to start server");
        std::process::exit(1);
    }
}

async fn run() -> anyhow::Result<()> {
    let config = Config::load()?;
    tracing::info!(
        app_name = %config.app_name,
        app_host = %config.app_host,
        app_port = config.app_port,
        "Server configuration loaded"
    );
    let pool = database::connect(&config).await?;
    database::run(&pool).await?;
    let device_repository = PostgresDeviceRepository::new(pool.clone());
    let device_service = DeviceService::new(device_repository);
    let telemetry_repository = PostgresTelemetryRepository::new(pool.clone());
    let telemetry_service = TelemetryService::new(
        PostgresDeviceRepository::new(pool.clone()),
        telemetry_repository,
    );
    let address: SocketAddr = format!("{}:{}", config.app_host, config.app_port).parse()?;
    let state = AppState {
        db: pool.clone(),
        device_service: Arc::new(device_service),
        telemetry_service: Arc::new(telemetry_service),
    };
    let trace_layer = TraceLayer::new_for_http()
        .make_span_with(|request: &axum::http::Request<_>| {
            tracing::info_span!(
                "http_request",
                method = %request.method(),
                path = %request.uri().path(),
            )
        })
        .on_response(
            |response: &axum::http::Response<_>,
             latency: std::time::Duration,
             _span: &tracing::Span| {
                tracing::info!(
                      status = %response.status(),
                    latency_ms = latency.as_millis(),
                    "request completed"
                );
            },
        );
    let app = router::create_router().with_state(state).layer(trace_layer);
    tracing::info!(%address, "Starting server");
    let listener = TcpListener::bind(address).await?;
    axum::serve(listener, app).await?;
    Ok(())
}
