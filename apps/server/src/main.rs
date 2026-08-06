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
use crate::state::AppState;
use std::net::SocketAddr;
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
    let state = AppState { config, db: pool };
    let app = app::create_app(state);
    let address = SocketAddr::from(([127, 0, 0, 1], 3000));

    println!("Server Running At http://{}", address);
    let listener = TcpListener::bind(address)
        .await
        .expect("Failed to bind to address");

    axum::serve(listener, app).await.expect("Server Error");
}
