// backend/src/config/mod.rs
use serde::Deserialize;
use std::env;

// Step 1: Re-export all config structures
pub use ai::AIConfig;
pub use database::DatabaseConfig;
pub use pipeline::PipelineConfig;
pub use risk::RiskConfig;
pub use server::ServerConfig;
pub use solana::SolanaConfig;

// Step 2: Import sub-modules
mod ai;
mod database;
mod pipeline;
mod risk;
mod server;
mod solana;

// Step 3: Main configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub server: ServerConfig,
    pub solana: SolanaConfig,
    pub ai: AIConfig,
    pub database: DatabaseConfig,
    pub pipeline: PipelineConfig,
    pub risk: RiskConfig,
}

impl Config {
    // Step 4: Load configuration from environment variables
    pub fn load() -> Self {
        // Load .env file if it exists
        dotenvy::dotenv().ok();

        tracing::info!("📝 Loading configuration from environment...");

        Self {
            server: ServerConfig::load(),
            solana: SolanaConfig::load(),
            ai: AIConfig::load(),
            database: DatabaseConfig::load(),
            pipeline: PipelineConfig::load(),
            risk: RiskConfig::load(),
        }
    }

    // Step 5: Validate configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        // Validate server config
        if let Err(server_errors) = self.server.validate() {
            errors.extend(server_errors);
        }

        // Validate solana config
        if let Err(solana_errors) = self.solana.validate() {
            errors.extend(solana_errors);
        }

        // Validate AI config
        if let Err(ai_errors) = self.ai.validate() {
            errors.extend(ai_errors);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 6: Check if running in production
    pub fn is_production(&self) -> bool {
        env::var("RUST_ENV")
            .map(|env| env == "production")
            .unwrap_or(false)
    }

    // Step 7: Check if running in development
    pub fn is_development(&self) -> bool {
        !self.is_production()
    }
}

// Step 8: Helper function to get environment variables
pub fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| {
        // Provide default values for common keys
        match key {
            "PORT" => "3000".to_string(),
            "HOST" => "0.0.0.0".to_string(),
            "LOG_LEVEL" => "info".to_string(),
            "CORS_ORIGIN" => "*".to_string(),
            "SOLANA_RPC_URL" => "https://api.devnet.solana.com".to_string(),
            "SOLANA_WS_URL" => "wss://api.devnet.solana.com".to_string(),
            "SOLANA_PROGRAM_ID" => "Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS".to_string(),
            "SOLANA_COMMITMENT" => "confirmed".to_string(),
            "AI_SERVICE_URL" => "http://localhost:8001".to_string(),
            "AI_SERVICE_TIMEOUT" => "30".to_string(),
            "AI_SERVICE_ENABLED" => "true".to_string(),
            "DATABASE_URL" => {
                "postgresql://user:password@localhost:5432/defi_portfolio".to_string()
            }
            "DATABASE_MAX_CONNECTIONS" => "20".to_string(),
            "BATCH_SIZE" => "10".to_string(),
            "BATCH_TIMEOUT_MS" => "100".to_string(),
            "MAX_QUEUE_SIZE" => "1000".to_string(),
            "ALERT_COOLDOWN_MINUTES" => "60".to_string(),
            "ALERT_RETENTION_DAYS" => "30".to_string(),
            "MIN_CONFIDENCE" => "0.7".to_string(),
            _ => "".to_string(),
        }
    })
}

// Step 9: Helper function to get environment variables as specific types
pub fn get_env_parsed<T: std::str::FromStr>(key: &str, default: T) -> T
where
    <T as std::str::FromStr>::Err: std::fmt::Debug,
{
    get_env(key).parse().unwrap_or(default)
}
