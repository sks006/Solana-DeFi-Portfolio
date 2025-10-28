use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};
use crate::BackendAppState;

// Step 1: Risk analysis request
#[derive(Debug, Deserialize)]
pub struct RiskAnalysisRequest {
    pub wallet: String,
    pub positions: Vec<PositionForAnalysis>,
    pub total_value: f64,
    pub leverage_ratio: f64,
}

// Step 2: Position data for risk analysis
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PositionForAnalysis {
    pub mint: String,
    pub amount: f64,
    pub value_usd: f64,
    pub volatility: f64,
}

// Step 3: Get risk alerts for a wallet
pub async fn get_risk_alerts(
    State(state): State<BackendAppState>,
) -> Json<Vec<crate::models::risk_alert::RiskAlert>> {
    state
        .metrics
        .record_api_request("get_risk_alerts", 200, 0.0)
        .await;

    // Step 4: Mock alerts - in production, query database
    let mock_alerts = vec![
        crate::models::risk_alert::RiskAlert::new(
            "mock_wallet".to_string(),
            crate::models::risk_alert::AlertSeverity::Medium,
            "High concentration in meme coins".to_string(),
            Some(serde_json::json!({"concentration": 0.45})),
        ),
        crate::models::risk_alert::RiskAlert::new(
            "mock_wallet".to_string(),
            crate::models::risk_alert::AlertSeverity::High,
            "Large position size relative to portfolio".to_string(),
            Some(serde_json::json!({"position_size_ratio": 0.32})),
        ),
    ];

    Json(mock_alerts)
}

// Step 5: Analyze position risk using AI
pub async fn analyze_position(
    State(state): State<BackendAppState>,
    Json(payload): Json<RiskAnalysisRequest>,
) -> Json<serde_json::Value> {
    tracing::info!(
        "🔍 Analyzing risk for wallet: {}, total_value: {}, leverage: {}",
        payload.wallet,
        payload.total_value,
        payload.leverage_ratio
    );

    state
        .metrics
        .record_api_request("analyze_position", 200, 0.0)
        .await;

    // Step 6: Use AI service to analyze risk
    let risk_analysis = state
        .ai_client
        .analyze_portfolio_risk(
            &payload.wallet,
            &payload.positions,
            payload.total_value,
            payload.leverage_ratio,
        )
        .await;

    match risk_analysis {
        Ok(analysis) => Json(serde_json::json!({
            "status": "success",
            "wallet": payload.wallet,
            "risk_score": analysis.risk_score,
            "risk_level": analysis.risk_level,
            "alerts": analysis.alerts,
            "recommendations": analysis.recommendations,
        })),
        Err(e) => {
            tracing::error!("❌ Risk analysis error: {}", e);
            Json(serde_json::json!({
                "status": "error",
                "error": e.to_string()
            }))
        }
    }
}
