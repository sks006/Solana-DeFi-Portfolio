use axum::{
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
    BackendAppInfo, BackendHealthCheck,
};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};
use axum::http::HeaderValue;

use tracing_subscriber::fmt::init;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    // Initialize logging with formatted output
    tracing_subscriber::fmt()
        .with_target(false) // hide the target (module path)
        .compact()          // compact, single-line logs
        .init();
    // Step 1: Create application state using helper from lib.rs
    let app_state = create_backend_app_state().await.map_err(|e| {
        eprintln!("❌ Failed to create app state: {}", e);
        e
    })?;

    tracing::info!("🚀 Starting Solana DeFi Portfolio Backend");

    // Step 2: Read allowed origins from environment (comma-separated)
    let origins = std::env::var("CORS_ORIGINS").unwrap_or_default();
    let allow_origins: Vec<HeaderValue> = origins
        .split(',')
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();

    // Default fallback if none specified (local dev)
    let cors = if allow_origins.is_empty() {
        tracing::warn!("⚠️  No CORS_ORIGINS specified, using permissive CORS (dev only).");
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
            .allow_origin(allow_origins)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Step 3: Build application routes
    let app = Router::new()
        .route("/health", get(health_check))
        .route("/info", get(get_app_info))
        .route("/metrics", get(solana_defi_backend::utils::telemetry::get_metrics))
        .route("/api/portfolio/:wallet", get(get_portfolio))
        .route("/api/portfolio/positions", post(update_position))
        .route("/api/swap/quote", post(get_swap_quote))
        .route("/api/swap/execute", post(execute_swap))
        .route("/api/risk/alerts", get(get_risk_alerts))
        .route("/api/risk/analyze", post(analyze_position))
        .route("/ws", get(ws_handler))
        .layer(cors) // 👈 attach our CORS layer
        .with_state(app_state);

    // Step 4: Start server
    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    tracing::info!("📡 Server running on http://{}", addr);

    axum::serve(tokio::net::TcpListener::bind(addr).await?, app).await?;

    Ok(())
}

// Health + info endpoints
async fn health_check() -> Json<BackendHealthCheck> {
    Json(BackendHealthCheck::new())
}

async fn get_app_info() -> Json<BackendAppInfo> {
    Json(BackendAppInfo::new())
}
