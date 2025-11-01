// backend/src/config/database.rs
use serde::Deserialize;

use super::{get_env, get_env_parsed};

// Step 1: Database configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
}

impl DatabaseConfig {
    // Step 2: Load database configuration from environment
    pub fn load() -> Self {
        Self {
            url: get_env("DATABASE_URL"),
            max_connections: get_env_parsed("DATABASE_MAX_CONNECTIONS", 20),
        }
    }

    // Step 3: Validate database configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.url.is_empty() {
            errors.push("DATABASE_URL is required".to_string());
        }

        if !self.url.starts_with("postgresql://") {
            errors.push("DATABASE_URL must start with postgresql://".to_string());
        }

        if self.max_connections == 0 {
            errors.push("DATABASE_MAX_CONNECTIONS cannot be 0".to_string());
        }

        if self.max_connections > 100 {
            errors.push("DATABASE_MAX_CONNECTIONS cannot exceed 100".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 4: Extract database name from URL
    pub fn database_name(&self) -> Option<String> {
        self.url
            .split('/')
            .last()
            .map(|s| s.split('?').next().unwrap_or("").to_string())
    }

    // Step 5: Check if using local database
    pub fn is_local(&self) -> bool {
        self.url.contains("localhost") || self.url.contains("127.0.0.1")
    }
}
