// backend/src/main.rs
use axum::{
    extract::State,
    routing::{get, post},
    Json, Router,
};
use solana_defi_backend::{
    create_backend_app_state,
    server_functions::{
        portfolio::{get_portfolio, update_position},
        risk::{analyze_position, get_risk_alerts},
        swap::{execute_swap, get_swap_quote},
    },
    ws::client::ws_handler,
    BackendAppInfo, BackendAppState, BackendHealthCheck,
};
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    init();

    // Step 1: Create application state using lib.rs function
    let app_state = create_backend_app_state().await.map_err(|e| {
        eprintln!("❌ Failed to create app state: {}", e);
        e
    })?;

    tracing::info!("🚀 Starting Solana DeFi Portfolio Backend");

    // Step 2: Build application routes using directly imported functions
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(get_app_info))
        .route(
            "/metrics",
            get(solana_defi_backend::utils::telemetry::get_metrics),
        )
        .route("/api/portfolio/:wallet", get(get_portfolio))
        .route("/api/portfolio/positions", post(update_position))
        .route("/api/swap/quote", post(get_swap_quote))
        .route("/api/swap/execute", post(execute_swap))
        .route("/api/risk/alerts", get(get_risk_alerts))
        .route("/api/risk/analyze", post(analyze_position))
        .route("/ws", get(ws_handler))
        .layer(CorsLayer::permissive())
        .with_state(app_state);

    // Step 3: Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("📡 Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

async fn health_check() -> Json<BackendHealthCheck> {
    Json(BackendHealthCheck::new())
}

async fn get_app_info() -> Json<BackendAppInfo> {
    Json(BackendAppInfo::new())
}
