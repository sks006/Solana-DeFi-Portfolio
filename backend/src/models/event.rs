// backend/src/models/event.rs
use serde::{Deserialize, Serialize};

// Step 1: Portfolio event types for processing
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PortfolioEvent {
    PositionUpdate {
        wallet: String,
        mint: String,
        pnl_delta: f64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    SwapExecuted {
        wallet: String,
        input_mint: String,
        output_mint: String,
        amount: u64,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
    RiskAlertTriggered {
        wallet: String,
        alert_type: String,
        severity: String,
        message: String,
        timestamp: chrono::DateTime<chrono::Utc>,
    },
}

// Step 2: Implement helper methods for events
impl PortfolioEvent {
    pub fn wallet(&self) -> &str {
        match self {
            PortfolioEvent::PositionUpdate { wallet, .. } => wallet,
            PortfolioEvent::SwapExecuted { wallet, .. } => wallet,
            PortfolioEvent::RiskAlertTriggered { wallet, .. } => wallet,
        }
    }

    pub fn timestamp(&self) -> &chrono::DateTime<chrono::Utc> {
        match self {
            PortfolioEvent::PositionUpdate { timestamp, .. } => timestamp,
            PortfolioEvent::SwapExecuted { timestamp, .. } => timestamp,
            PortfolioEvent::RiskAlertTriggered { timestamp, .. } => timestamp,
        }
    }
}
