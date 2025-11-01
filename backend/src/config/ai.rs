// backend/src/config/ai.rs
use serde::Deserialize;

use super::{get_env, get_env_parsed};

// Step 1: AI service configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct AIConfig {
    pub url: String,
    pub timeout_seconds: u64,
    pub enabled: bool,
}

impl AIConfig {
    // Step 2: Load AI configuration from environment
    pub fn load() -> Self {
        Self {
            url: get_env("AI_SERVICE_URL"),
            timeout_seconds: get_env_parsed("AI_SERVICE_TIMEOUT", 30),
            enabled: get_env_parsed("AI_SERVICE_ENABLED", true),
        }
    }

    // Step 3: Validate AI configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.enabled && self.url.is_empty() {
            errors.push("AI_SERVICE_URL is required when AI service is enabled".to_string());
        }

        if self.enabled && !self.url.starts_with("http") {
            errors.push("AI_SERVICE_URL must start with http:// or https://".to_string());
        }

        if self.timeout_seconds == 0 {
            errors.push("AI_SERVICE_TIMEOUT cannot be 0".to_string());
        }

        if self.timeout_seconds > 300 {
            errors.push("AI_SERVICE_TIMEOUT cannot exceed 300 seconds".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 4: Get timeout as Duration
    pub fn timeout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.timeout_seconds)
    }
}
