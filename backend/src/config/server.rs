// backend/src/config/server.rs
use serde::Deserialize;

/// Server configuration
#[derive(Debug, Deserialize, Clone)]
pub struct ServerConfig {
    /// Bind host. Use 0.0.0.0 on cloud hosts.
    pub host: String,
    /// Bind port. Many hosts inject PORT dynamically.
    pub port: u16,
    /// tracing log level: trace|debug|info|warn|error
    pub log_level: String,
    /// CORS origin(s). Comma-separated; "*" allowed for quick demos.
    pub cors_origin: String,
}

impl ServerConfig {
    /// Load from environment with safe defaults
    pub fn load() -> Self {
        let host = std::env::var("HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
        let port = std::env::var("PORT")
            .ok()
            .and_then(|s| s.parse::<u16>().ok())
            .unwrap_or(3000);
        let log_level = std::env::var("LOG_LEVEL").unwrap_or_else(|_| "info".to_string());
        let cors_origin = std::env::var("CORS_ORIGIN").unwrap_or_else(|_| "*".to_string());

        Self {
            host,
            port,
            log_level,
            cors_origin,
        }
    }

    /// Validate config (non-fatal; you can choose to warn instead of Err)
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.port == 0 || self.port > 65535 {
            errors.push("PORT must be between 1 and 65535".to_string());
        }

        if self.host.trim().is_empty() {
            errors.push("HOST cannot be empty".to_string());
        }

        let valid_levels = ["trace", "debug", "info", "warn", "error"];
        if !valid_levels.contains(&self.log_level.to_lowercase().as_str()) {
            errors.push(format!(
                "LOG_LEVEL must be one of: {}",
                valid_levels.join(", ")
            ));
        }

        if errors.is_empty() { Ok(()) } else { Err(errors) }
    }

    /// Bind address string
    pub fn address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
