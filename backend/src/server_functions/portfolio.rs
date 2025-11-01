use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use serde::{Deserialize, Serialize};
use crate::BackendAppState;
use crate::server_functions::risk::PositionForAnalysis;
use futures::future::join_all;

#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub wallet: String,
    pub total_value: f64,
    pub positions: Vec<Position>,
    pub pnl_24h: f64,
    pub risk_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub mint: String,
    pub amount: f64,
    pub value_usd: f64,
    pub pnl: f64,
    pub entry_price: f64,
}

#[derive(Debug, Deserialize)]
pub struct UpdatePositionRequest {
    pub wallet: String,
    pub mint: String,
    pub pnl_delta: f64,
}

#[derive(Debug, Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

pub async fn get_portfolio(
    Path(wallet): Path<String>,
    State(state): State<BackendAppState>,
) -> Result<Json<PortfolioResponse>, (StatusCode, Json<ErrorResponse>)> {
    tracing::info!("📊 Fetching portfolio for wallet: {}", wallet);

    state.metrics.record_api_request("get_portfolio", 200, 0.0).await;

    let token_accounts = state
        .solana_client
        .get_token_accounts(&wallet)
        .await
        .unwrap_or_default();

    let positions: Vec<Position> = token_accounts
        .into_iter()
        .map(|account| Position {
            mint: account.mint,
            amount: account.amount,
            value_usd: account.amount * 1.0,
            pnl: 0.0,
            entry_price: 1.0,
        })
        .collect();

    let total_value: f64 = positions.iter().map(|p| p.value_usd).sum();
    let leverage_ratio: f64 = 1.2;

    // ✅ Convert positions dynamically
    let futures = positions.iter().map(|p| async {
        let vol = state.solana_client.get_token_volatility(&p.mint).await;
        PositionForAnalysis {
            mint: p.mint.clone(),
            amount: p.amount,
            value_usd: p.value_usd,
            volatility: vol,
        }
    });
    let analysis_positions: Vec<PositionForAnalysis> = join_all(futures).await;

    // ✅ Call AI
    let risk_result = state
        .ai_client
        .analyze_portfolio_risk(&wallet, &analysis_positions, total_value, leverage_ratio)
        .await
        .map_err(|e| {
            tracing::error!("❌ AI call failed: {}", e);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse {
                    error: format!("AI analysis failed: {}", e),
                }),
            )
        })?;

    Ok(Json(PortfolioResponse {
        wallet,
        total_value,
        positions,
        pnl_24h: 0.0,
        risk_score: risk_result.risk_score,
    }))
}
// Step 9: Update position PnL (called from on-chain program or frontend)
pub async fn update_position(
    State(state): State<BackendAppState>,
    Json(payload): Json<UpdatePositionRequest>,
) -> Json<serde_json::Value> {
    tracing::info!(
        "🔄 Updating position for wallet: {}, mint: {}, pnl_delta: {}",
        payload.wallet,
        payload.mint,
        payload.pnl_delta
    );

    // Step 1️⃣: Record API usage
    state
        .metrics
        .record_api_request("update_position", 200, 0.0)
        .await;

    // Step 2️⃣: Emit an internal event for the async pipeline
    let event = crate::models::event::PortfolioEvent::PositionUpdate {
        wallet: payload.wallet.clone(),
        mint: payload.mint.clone(),
        pnl_delta: payload.pnl_delta,
        timestamp: chrono::Utc::now(),
    };

    // send to background event processor
    if let Err(e) = state.event_tx.send(event).await {
        tracing::error!("❌ Failed to queue position update event: {}", e);
    }

    // Step 3️⃣: Broadcast update via WebSocket for realtime frontend sync
    let ws_message = crate::ws::hub::WsMessage {
        message_type: "position_updated".to_string(),
        payload: serde_json::json!({
            "wallet": payload.wallet,
            "mint": payload.mint,
            "pnl_delta": payload.pnl_delta,
        }),
        timestamp: chrono::Utc::now(),
    };

    if let Err(e) = state.ws_hub.broadcast(ws_message) {
        tracing::warn!("⚠️ Failed to broadcast position update: {}", e);
    }

    // Step 4️⃣: Respond to frontend / client
    Json(serde_json::json!({
        "status": "success",
        "message": "Position updated successfully",
        "wallet": payload.wallet,
        "mint": payload.mint,
        "pnl_delta": payload.pnl_delta
    }))
}

