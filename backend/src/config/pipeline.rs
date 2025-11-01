// backend/src/config/pipeline.rs
use serde::Deserialize;

use super::get_env_parsed;

// Step 1: Pipeline configuration structure
#[derive(Debug, Deserialize, Clone)]
pub struct PipelineConfig {
    pub batch_size: usize,
    pub batch_timeout_ms: u64,
    pub max_queue_size: usize,
}

impl PipelineConfig {
    // Step 2: Load pipeline configuration from environment
    pub fn load() -> Self {
        Self {
            batch_size: get_env_parsed("BATCH_SIZE", 10),
            batch_timeout_ms: get_env_parsed("BATCH_TIMEOUT_MS", 100),
            max_queue_size: get_env_parsed("MAX_QUEUE_SIZE", 1000),
        }
    }

    // Step 3: Validate pipeline configuration
    pub fn validate(&self) -> Result<(), Vec<String>> {
        let mut errors = Vec::new();

        if self.batch_size == 0 {
            errors.push("BATCH_SIZE cannot be 0".to_string());
        }

        if self.batch_size > 1000 {
            errors.push("BATCH_SIZE cannot exceed 1000".to_string());
        }

        if self.batch_timeout_ms == 0 {
            errors.push("BATCH_TIMEOUT_MS cannot be 0".to_string());
        }

        if self.batch_timeout_ms > 60000 {
            errors.push("BATCH_TIMEOUT_MS cannot exceed 60000 (1 minute)".to_string());
        }

        if self.max_queue_size == 0 {
            errors.push("MAX_QUEUE_SIZE cannot be 0".to_string());
        }

        if self.max_queue_size > 100000 {
            errors.push("MAX_QUEUE_SIZE cannot exceed 100000".to_string());
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    // Step 4: Get batch timeout as Duration
    pub fn batch_timeout_duration(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.batch_timeout_ms)
    }

    // Step 5: Check if batching is enabled
    pub fn is_batching_enabled(&self) -> bool {
        self.batch_size > 1
    }
}
