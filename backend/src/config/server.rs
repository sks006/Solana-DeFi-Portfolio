// backend/src/config/server.rs
use serde::Deserialize;

use super::get_env;

// Step 1: Server configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    pub port: u16,
    pub host: String,
    pub log_level: String,
    pub cors_origin: String,
}

impl ServerConfig {
    // Step 2: Load server configuration from environment
    pub fn load() -> Self {
        Self {
            port: get_env("PORT").parse().unwrap_or(3000),
            host: get_env("HOST"),
            log_level: get_env("LOG_LEVEL"),
            cors_origin: get_env("CORS_ORIGIN"),
        }
    }

    // Step 3: Validate server configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.port == 0 {
            errors.push("PORT cannot be 0".to_string());
        }

        if self.port > 65535 {
            errors.push("PORT must be between 1 and 65535".to_string());
        }

        if self.host.is_empty() {
            errors.push("HOST cannot be empty".to_string());
        }

        if self.log_level.is_empty() {
            errors.push("LOG_LEVEL cannot be empty".to_string());
        }

        let valid_log_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_log_levels.contains(&self.log_level.to_lowercase().as_str()) {
            errors.push(format!(
                "LOG_LEVEL must be one of: {}",
                valid_log_levels.join(", ")
            ));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 4: Get server address for binding
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
