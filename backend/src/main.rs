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

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // simple, compact logs
    tracing_subscriber::fmt().with_target(false).compact().init();

    // Create shared state
    let app_state = create_backend_app_state().await.map_err(|e| {
        eprintln!("❌ Failed to create app state: {e}");
        e
    })?;

    tracing::info!("🚀 Starting Solana DeFi Portfolio Backend");

    // CORS: comma-separated origins; default permissive for hackathon/demo
    let origins = std::env::var("CORS_ORIGINS").unwrap_or_default();
    let allow_origins: Vec<HeaderValue> = origins
        .split(',')
        .filter_map(|o| HeaderValue::from_str(o.trim()).ok())
        .collect();
    let cors = if allow_origins.is_empty() {
        tracing::warn!("⚠️  No CORS_ORIGINS set; using permissive CORS (dev/demo only).");
        CorsLayer::permissive()
    } else {
        CorsLayer::new()
            .allow_origin(allow_origins)
            .allow_methods(Any)
            .allow_headers(Any)
    };

    // Router
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
        .layer(cors)
        .with_state(app_state);

    // ✅ Dynamic host/port (clouds inject PORT)
    let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".into());
    let port = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<u16>().ok())
        .unwrap_or(3000);

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    tracing::info!("📡 Server running on http://{addr}");

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
