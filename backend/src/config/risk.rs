// backend/src/config/risk.rs
use serde::Deserialize;

use super::{get_env, get_env_parsed};

// Step 1: Risk engine configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct RiskConfig {
    pub cooldown_minutes: u64,
    pub alert_retention_days: u32,
    pub min_confidence: f64,
}

impl RiskConfig {
    // Step 2: Load risk configuration from environment
    pub fn load() -> Self {
        Self {
            cooldown_minutes: get_env_parsed("ALERT_COOLDOWN_MINUTES", 60),
            alert_retention_days: get_env_parsed("ALERT_RETENTION_DAYS", 30),
            min_confidence: get_env_parsed("MIN_CONFIDENCE", 0.7),
        }
    }

    // Step 3: Validate risk configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.cooldown_minutes == 0 {
            errors.push("ALERT_COOLDOWN_MINUTES cannot be 0".to_string());
        }

        if self.alert_retention_days == 0 {
            errors.push("ALERT_RETENTION_DAYS cannot be 0".to_string());
        }

        if self.min_confidence < 0.0 || self.min_confidence > 1.0 {
            errors.push("MIN_CONFIDENCE must be between 0.0 and 1.0".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 4: Get cooldown as Duration
    pub fn cooldown_duration(&self) -> std::time::Duration {
        std::time::Duration::from_secs(self.cooldown_minutes * 60)
    }

    // Step 5: Check if confidence threshold is high
    pub fn is_high_confidence(&self) -> bool {
        self.min_confidence > 0.8
    }
}
