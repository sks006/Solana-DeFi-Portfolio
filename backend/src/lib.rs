// backend/src/lib.rs

// Step 1: Re-export all public modules with clear names
pub mod config;
pub mod integration;
pub mod models;
pub mod pipeline;
pub mod server_functions;
pub mod services;
pub mod utils;
pub mod ws;

// Step 2: Re-export commonly used types with explicit names
pub use config::Config;
pub use models::event::PortfolioEvent;
pub use models::risk_alert::RiskAlert;

pub use server_functions::portfolio::{PortfolioResponse, Position, UpdatePositionRequest};
pub use server_functions::risk::{PositionForAnalysis, RiskAnalysisRequest};
pub use server_functions::swap::{SwapQuote, SwapRequest};
pub use ws::hub::WsMessage;

// Step 3: Application state with explicit naming to avoid conflicts
#[derive(Clone)]
pub struct BackendAppState {
    pub config: Config,
    pub solana_client: services::solana_client::SolanaClient,
    pub ai_client: services::ai_client::AIClient,
    pub metrics: services::metrics::MetricsService,
    pub event_tx: tokio::sync::mpsc::Sender<models::event::PortfolioEvent>,
    pub ws_hub: ws::hub::WsHub,
}

// Step 4: Implement helper methods for BackendAppState
impl BackendAppState {
    /// Create a new BackendAppState instance
    pub fn new(
        config: Config,
        solana_client: services::solana_client::SolanaClient,
        ai_client: services::ai_client::AIClient,
        metrics: services::metrics::MetricsService,
        event_tx: tokio::sync::mpsc::Sender<models::event::PortfolioEvent>,
        ws_hub: ws::hub::WsHub,
    ) -> Self {
        Self {
            config,
            solana_client,
            ai_client,
            metrics,
            event_tx,
            ws_hub,
        }
    }

    /// Get a reference to the configuration
    pub fn config(&self) -> &Config {
        &self.config
    }

    /// Get a reference to the Solana client
    pub fn solana_client(&self) -> &services::solana_client::SolanaClient {
        &self.solana_client
    }

    /// Get a reference to the AI client
    pub fn ai_client(&self) -> &services::ai_client::AIClient {
        &self.ai_client
    }

    /// Get a reference to the metrics service
    pub fn metrics(&self) -> &services::metrics::MetricsService {
        &self.metrics
    }

    /// Get a reference to the WebSocket hub
    pub fn ws_hub(&self) -> &ws::hub::WsHub {
        &self.ws_hub
    }

    /// Send an event to the processing pipeline
    pub async fn send_event(
        &self,
        event: models::event::PortfolioEvent,
    ) -> Result<(), tokio::sync::mpsc::error::SendError<models::event::PortfolioEvent>> {
        self.event_tx.send(event).await
    }

    /// Broadcast a message to all WebSocket clients
    pub fn broadcast_ws_message(
        &self,
        message: ws::hub::WsMessage,
    ) -> Result<(), tokio::sync::broadcast::error::SendError<ws::hub::WsMessage>> {
        self.ws_hub.broadcast(message)
    }
}

// Step 5: Application error type with unique naming
#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("Configuration error: {0}")]
    ConfigError(String),

    #[error("Solana RPC error: {0}")]
    SolanaError(#[from] Box<dyn std::error::Error + Send + Sync>),

    #[error("AI service error: {0}")]
    AIError(String),

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("WebSocket error: {0}")]
    WebSocketError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Channel error: {0}")]
    ChannelError(String),
}

// Step 6: Type alias for Result<T, BackendError>
pub type BackendResult<T> = std::result::Result<T, BackendError>;

// Step 7: Application initialization function
pub async fn create_backend_app_state() -> BackendResult<BackendAppState> {
    // Load configuration
    let config = Config::load();

    // Validate configuration
    if let Err(errors) = config.validate() {
        return Err(BackendError::ConfigError(errors.join(", ")));
    }

    tracing::info!("📝 Configuration loaded successfully");

    // Initialize services
    let solana_client = services::solana_client::SolanaClient::new(&config.solana);
    let ai_client = services::ai_client::AIClient::new(&config.ai).await;
    let metrics = services::metrics::MetricsService::new();

    // Create event processing pipeline
    let (event_tx, _event_rx) = tokio::sync::mpsc::channel(config.pipeline.max_queue_size);

    // Initialize WebSocket hub
    let ws_hub = ws::hub::WsHub::new();

    Ok(BackendAppState::new(
        config,
        solana_client,
        ai_client,
        metrics,
        event_tx,
        ws_hub,
    ))
}

// Step 8: Health check response
#[derive(Debug, serde::Serialize, Clone)]
pub struct BackendHealthCheck {
    pub status: String,
    pub version: &'static str,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub service: String,
}

impl BackendHealthCheck {
    pub fn new() -> Self {
        Self {
            status: "healthy".to_string(),
            version: env!("CARGO_PKG_VERSION"),
            timestamp: chrono::Utc::now(),
            service: "solana-defi-backend".to_string(),
        }
    }

    pub fn unhealthy(reason: String) -> Self {
        Self {
            status: "unhealthy".to_string(),
            version: env!("CARGO_PKG_VERSION"),
            timestamp: chrono::Utc::now(),
            service: reason,
        }
    }
}

impl Default for BackendHealthCheck {
    fn default() -> Self {
        Self::new()
    }
}

// Step 9: Application information
#[derive(Debug, serde::Serialize, Clone)]
pub struct BackendAppInfo {
    pub name: &'static str,
    pub version: &'static str,
    pub description: &'static str,
    pub authors: &'static str,
    pub repository: &'static str,
}

impl BackendAppInfo {
    pub fn new() -> Self {
        Self {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            description: env!("CARGO_PKG_DESCRIPTION"),
            authors: env!("CARGO_PKG_AUTHORS"),
            repository: env!("CARGO_PKG_REPOSITORY"),
        }
    }
}

impl Default for BackendAppInfo {
    fn default() -> Self {
        Self::new()
    }
}

// Step 10: Service status for monitoring
#[derive(Debug, serde::Serialize, Clone)]
pub struct BackendServiceStatus {
    pub solana_rpc: bool,
    pub ai_service: bool,
    pub database: bool,
    pub websocket: bool,
    pub event_processor: bool,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl BackendServiceStatus {
    pub fn new(
        solana_rpc: bool,
        ai_service: bool,
        database: bool,
        websocket: bool,
        event_processor: bool,
    ) -> Self {
        Self {
            solana_rpc,
            ai_service,
            database,
            websocket,
            event_processor,
            timestamp: chrono::Utc::now(),
        }
    }
}

// Step 11: Utility functions for the backend
pub mod backend_utils {
    use super::*;

    /// Initialize the backend application
    pub async fn initialize_backend() -> BackendResult<BackendAppState> {
        create_backend_app_state().await
    }

    /// Create a WebSocket message with standard format
    pub fn create_ws_message(
        message_type: impl Into<String>,
        payload: serde_json::Value,
    ) -> WsMessage {
        WsMessage {
            message_type: message_type.into(),
            payload,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a portfolio event for position update
    pub fn create_position_update_event(
        wallet: String,
        mint: String,
        pnl_delta: f64,
    ) -> PortfolioEvent {
        PortfolioEvent::PositionUpdate {
            wallet,
            mint,
            pnl_delta,
            timestamp: chrono::Utc::now(),
        }
    }

    /// Create a portfolio event for swap execution
    pub fn create_swap_executed_event(
        wallet: String,
        input_mint: String,
        output_mint: String,
        amount: u64,
    ) -> PortfolioEvent {
        PortfolioEvent::SwapExecuted {
            wallet,
            input_mint,
            output_mint,
            amount,
            timestamp: chrono::Utc::now(),
        }
    }
}

// Step 12: Pre-import commonly used types for convenience
pub mod prelude {
    pub use super::{
        BackendAppState, BackendError, BackendResult, Config, PortfolioEvent, PortfolioResponse,
        Position, PositionForAnalysis, RiskAlert, RiskAnalysisRequest, SwapQuote, SwapRequest,
        UpdatePositionRequest, WsMessage,
    };

    // Re-export common traits and types
    pub use axum::{
        extract::{Path, State},
        Json,
    };
    pub use serde::{Deserialize, Serialize};
    pub use tracing::{debug, error, info, warn};
}
