// backend/src/server_functions/swap.rs
use axum::{extract::State, Json};
use chrono; // Add chrono for timestamp
use serde::{Deserialize, Serialize};
use serde_json;
use tracing; // Add serde_json for the return type

// Use the full path from your crate
use crate::BackendAppState;

// Step 1: Swap quote response
#[derive(Debug, Serialize, Deserialize)]
pub struct SwapQuote {
    pub input_mint: String,
    pub output_mint: String,
    pub in_amount: u64,
    pub out_amount: u64,
    pub price_impact: f64,
    pub fee_amount: u64,
    pub route: Vec<String>,
}

// Step 2: Swap execution request
#[derive(Debug, Deserialize)]
pub struct SwapRequest {
    pub wallet: String,
    pub input_mint: String,
    pub output_mint: String,
    pub amount: u64,
    pub slippage_bps: u64,
}

// Step 3: Get swap quote from AMM pool
#[axum::debug_handler] // Add this for better error messages
pub async fn get_swap_quote(
    State(state): State<BackendAppState>,
    Json(payload): Json<SwapRequest>,
) -> Json<SwapQuote> {
    tracing::info!("💱 Getting swap quote for {:?}", payload);

    state
        .metrics
        .record_api_request("get_swap_quote", 200, 0.0)
        .await;

    // Step 4: Simulate AMM swap calculation
    let out_amount = calculate_swap_output(
        &payload.input_mint,
        &payload.output_mint,
        payload.amount as f64,
    )
    .await;

    let fee_amount = (out_amount as f64 * 0.003) as u64; // 0.3% fee
    let final_out = out_amount - fee_amount;

    let quote = SwapQuote {
        input_mint: payload.input_mint,
        output_mint: payload.output_mint,
        in_amount: payload.amount,
        out_amount: final_out,
        price_impact: 0.12, // Mock calculation
        fee_amount,
        route: vec!["SOL".to_string(), "USDC".to_string()],
    };

    Json(quote)
}

// Step 5: Execute swap transaction
#[axum::debug_handler] // Add this for better error messages
pub async fn execute_swap(
    State(state): State<BackendAppState>,
    Json(payload): Json<SwapRequest>,
) -> Json<serde_json::Value> {
    tracing::info!("⚡ Executing swap for wallet: {}", payload.wallet);

    // Step 6: Build and send transaction to Solana
    let signature = state
        .solana_client
        .execute_swap_transaction(
            &payload.wallet,
            &payload.input_mint,
            &payload.output_mint,
            payload.amount as f64,
            payload.slippage_bps,
        )
        .await;

    match signature {
        Ok(sig) => {
            // Step 7: Record successful swap
            state
                .metrics
                .record_api_request("execute_swap", 200, 0.0)
                .await;

            // Step 8: Emit swap event
            let event = crate::models::event::PortfolioEvent::SwapExecuted {
                wallet: payload.wallet.clone(),
                input_mint: payload.input_mint,
                output_mint: payload.output_mint,
                amount: payload.amount,
                timestamp: chrono::Utc::now(),
            };

            let _ = state.event_tx.send(event).await;

            Json(serde_json::json!({
                "status": "success",
                "signature": sig,
                "message": "Swap executed successfully"
            }))
        }
        Err(e) => {
            state
                .metrics
                .record_api_request("execute_swap", 500, 0.0)
                .await;
            Json(serde_json::json!({
                "status": "error",
                "error": e.to_string()
            }))
        }
    }
}

// Step 9: Calculate swap output using constant product formula
async fn calculate_swap_output(input_mint: &str, output_mint: &str, amount: f64) -> u64 {
    // Mock AMM calculation - would query on-chain pool state
    let base_rate = match (input_mint, output_mint) {
        ("SOL", "USDC") => 98.0,
        ("USDC", "SOL") => 0.0102,
        _ => 1.0,
    };

    (amount as f64 * base_rate) as u64
}
