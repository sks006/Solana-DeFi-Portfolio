// backend/src/config/solana.rs
use serde::Deserialize;
use solana_rpc_client_api::config::CommitmentConfig;

use super::get_env;

// Step 1: Solana configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct SolanaConfig {
    pub rpc_url: String,
    pub ws_url: String,
    pub program_id: String,
    pub commitment: String,
}

impl SolanaConfig {
    // Step 2: Load Solana configuration from environment
    pub fn load() -> Self {
        Self {
            rpc_url: get_env("SOLANA_RPC_URL"),
            ws_url: get_env("SOLANA_WS_URL"),
            program_id: get_env("SOLANA_PROGRAM_ID"),
            commitment: get_env("SOLANA_COMMITMENT"),
        }
    }

    // Step 3: Validate Solana configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.rpc_url.is_empty() {
            errors.push("SOLANA_RPC_URL is required".to_string());
        }

        if !self.rpc_url.starts_with("http") {
            errors.push("SOLANA_RPC_URL must start with http:// or https://".to_string());
        }

        if self.ws_url.is_empty() {
            errors.push("SOLANA_WS_URL is required".to_string());
        }

        if !self.ws_url.starts_with("ws") {
            errors.push("SOLANA_WS_URL must start with ws:// or wss://".to_string());
        }

        if self.program_id.is_empty() {
            errors.push("SOLANA_PROGRAM_ID is required".to_string());
        }

        let valid_commitments = ["processed", "confirmed", "finalized"];
        if !valid_commitments.contains(&self.commitment.to_lowercase().as_str()) {
            errors.push(format!(
                "SOLANA_COMMITMENT must be one of: {}",
                valid_commitments.join(", ")
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 4: Convert commitment string to CommitmentConfig
    pub fn commitment_config(&self) -> CommitmentConfig {
        match self.commitment.to_lowercase().as_str() {
            "processed" => CommitmentConfig::processed(),
            "confirmed" => CommitmentConfig::confirmed(),
            "finalized" => CommitmentConfig::finalized(),
            _ => CommitmentConfig::confirmed(),
        }
    }

    // Step 5: Check if using mainnet
    pub fn is_mainnet(&self) -> bool {
        self.rpc_url.contains("mainnet")
    }

    // Step 6: Check if using devnet
    pub fn is_devnet(&self) -> bool {
        self.rpc_url.contains("devnet")
    }

    // Step 7: Check if using testnet
    pub fn is_testnet(&self) -> bool {
        self.rpc_url.contains("testnet")
    }
}
