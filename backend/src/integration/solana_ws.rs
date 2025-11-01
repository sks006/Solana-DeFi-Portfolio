// backend/src/ingestion/solana_ws.rs
use futures_util::{SinkExt, StreamExt};
use serde_json::json;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

use crate::models::event::PortfolioEvent;

// Step 1: Solana WebSocket client for real-time on-chain data
pub struct SolanaWebSocket {
    event_tx: tokio::sync::mpsc::Sender<PortfolioEvent>,
    rpc_url: String,
}

impl SolanaWebSocket {
    pub fn new(event_tx: tokio::sync::mpsc::Sender<PortfolioEvent>) -> Self {
        let rpc_url = std::env::var("SOLANA_WS_URL")
            .unwrap_or_else(|_| "wss://api.devnet.solana.com".to_string());

        Self { event_tx, rpc_url }
    }

    // Step 2: Start listening to Solana events
    pub async fn start(&self) {
        tracing::info!("🌐 Starting Solana WebSocket ingestion");

        loop {
            match self.connect().await {
                Ok(_) => tracing::info!("✅ Solana WebSocket connected"),
                Err(e) => {
                    tracing::error!("❌ Solana WebSocket error: {}, reconnecting...", e);
                    tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                }
            }
        }
    }

    // Step 3: Establish WebSocket connection
    async fn connect(&self) -> Result<(), Box<dyn std::error::Error>> {
        let (mut ws_stream, _) = connect_async(&self.rpc_url).await?;

        // Step 4: Subscribe to program logs
        let subscribe_message = json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "logsSubscribe",
            "params": [
                {
                    "mentions": [
                        "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS" // Your program ID
                    ]
                },
                {
                    "commitment": "confirmed"
                }
            ]
        });

        ws_stream
            .send(Message::Text(subscribe_message.to_string()))
            .await?;

        // Step 5: Process incoming messages
        while let Some(message) = ws_stream.next().await {
            match message {
                Ok(Message::Text(text)) => {
                    if let Err(e) = self.handle_message(&text).await {
                        tracing::error!("Error handling message: {}", e);
                    }
                }
                Err(e) => {
                    tracing::error!("WebSocket error: {}", e);
                    break;
                }
                _ => {}
            }
        }

        Ok(())
    }

    // Step 6: Parse and handle incoming Solana events
    async fn handle_message(&self, message: &str) -> Result<(), Box<dyn std::error::Error>> {
        let value: serde_json::Value = serde_json::from_str(message)?;

        // Step 7: Extract transaction logs
        if let Some(params) = value.get("params") {
            if let Some(result) = params.get("result") {
                if let Some(logs) = result.get("value").and_then(|v| v.get("logs")) {
                    if let Some(logs_array) = logs.as_array() {
                        // Step 8: Process each log entry
                        for log in logs_array {
                            if let Some(log_str) = log.as_str() {
                                self.process_log_entry(log_str).await?;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

    // Step 9: Process individual log entries
    async fn process_log_entry(&self, log: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Step 10: Look for our program's event signatures
        if log.contains("Program log: Instruction: UpdatePosition") {
            self.handle_position_update(log).await?;
        } else if log.contains("Program log: Instruction: ExecuteSwap") {
            self.handle_swap_executed(log).await?;
        }

        Ok(())
    }

    // Step 11: Handle position update events
    async fn handle_position_update(&self, _log: &str) -> Result<(), Box<dyn std::error::Error>> {
        // Extract data from logs (simplified)
        let event = PortfolioEvent::PositionUpdate {
            wallet: "extracted_wallet".to_string(),
            mint: "SOL".to_string(),
            pnl_delta: 150.0,
            timestamp: chrono::Utc::now(),
        };

        let _ = self.event_tx.send(event).await;
        Ok(())
    }

    // Step 12: Handle swap execution events
    async fn handle_swap_executed(&self, _log: &str) -> Result<(), Box<dyn std::error::Error>> {
        let event = PortfolioEvent::SwapExecuted {
            wallet: "extracted_wallet".to_string(),
            input_mint: "SOL".to_string(),
            output_mint: "USDC".to_string(),
            amount: 1000000,
            timestamp: chrono::Utc::now(),
        };

        let _ = self.event_tx.send(event).await;
        Ok(())
    }
}
