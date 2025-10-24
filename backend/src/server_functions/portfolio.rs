// backend/src/server_functions/portfolio.rs
use axum::{
    extract::{Path, State},
    Json,
};
use serde::{Deserialize, Serialize};

// ✅ Clear, conflict-free import
use crate::BackendAppState;

// Step 1: Portfolio response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct PortfolioResponse {
    pub wallet: String,
    pub total_value: f64,
    pub positions: Vec<Position>,
    pub pnl_24h: f64,
    pub risk_score: f64,
}

// Step 2: Position details
#[derive(Debug, Serialize, Deserialize)]
pub struct Position {
    pub mint: String,
    pub amount: f64,
    pub value_usd: f64,
    pub pnl: f64,
    pub entry_price: f64,
}

// Step 3: Update position request
#[derive(Debug, Deserialize)]
pub struct UpdatePositionRequest {
    pub wallet: String,
    pub mint: String,
    pub pnl_delta: f64,
}

// Step 4: Get portfolio for a specific wallet
pub async fn get_portfolio(
    Path(wallet): Path<String>,
    State(state): State<BackendAppState>, // ✅ Using BackendAppState
) -> Json<PortfolioResponse> {
    tracing::info!("📊 Fetching portfolio for wallet: {}", wallet);

    // Step 5: Record metrics
    state
        .metrics
        .record_api_request("get_portfolio", 200, 0.0)
        .await;

    // Step 6: Fetch token accounts from Solana
    let token_accounts = state
        .solana_client
        .get_token_accounts(&wallet)
        .await
        .unwrap_or_default();

    // Step 7: Calculate portfolio metrics
    let positions: Vec<Position> = token_accounts
        .into_iter()
        .map(|account| Position {
            mint: account.mint,
            amount: account.amount,
            value_usd: account.amount * 1.0, // Mock price calculation
            pnl: 0.0,
            entry_price: 1.0,
        })
        .collect();

    let total_value: f64 = positions.iter().map(|p| p.value_usd).sum();

    // Step 8: Get risk score from AI service
    let risk_score = state
        .ai_client
        .assess_portfolio_risk(&wallet, &positions)
        .await
        .unwrap_or(0.5);

    let response = PortfolioResponse {
        wallet,
        total_value,
        positions,
        pnl_24h: 0.0, // Calculate from historical data
        risk_score,
    };

    Json(response)
}

// Step 9: Update position PnL (called from on-chain program)
pub async fn update_position(
    State(state): State<BackendAppState>, // ✅ Using BackendAppState
    Json(payload): Json<UpdatePositionRequest>,
) -> Json<serde_json::Value> {
    tracing::info!("🔄 Updating position for wallet: {}", payload.wallet);

    // Step 10: Emit event for real-time processing
    let event = crate::models::event::PortfolioEvent::PositionUpdate {
        wallet: payload.wallet.clone(),
        mint: payload.mint.clone(),
        pnl_delta: payload.pnl_delta,
        timestamp: chrono::Utc::now(),
    };

    let _ = state.event_tx.send(event).await;

    // Step 11: Send real-time update via WebSocket
    let ws_message = crate::ws::hub::WsMessage {
        message_type: "position_updated".to_string(),
        payload: serde_json::json!({
            "wallet": payload.wallet,
            "mint": payload.mint,
            "pnl_delta": payload.pnl_delta,
        }),
        timestamp: chrono::Utc::now(),
    };

    let _ = state.ws_hub.broadcast(ws_message);

    Json(serde_json::json!({
        "status": "success",
        "message": "Position updated successfully"
    }))
}
