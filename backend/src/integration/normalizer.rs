// backend/src/ingestion/normalizer.rs
use crate::models::event::PortfolioEvent;

// Step 1: Event normalization service
pub struct EventNormalizer;

impl EventNormalizer {
    pub fn new() -> Self {
        Self
    }

    // Step 2: Normalize raw events into standardized format
    pub fn normalize_event(
        &self,
        raw_event: RawEvent,
    ) -> Result<PortfolioEvent, NormalizationError> {
        match raw_event.event_type.as_str() {
            "position_update" => self.normalize_position_update(raw_event),
            "swap_executed" => self.normalize_swap_executed(raw_event),
            _ => Err(NormalizationError::UnknownEventType),
        }
    }

    // Step 3: Normalize position update events
    fn normalize_position_update(
        &self,
        raw_event: RawEvent,
    ) -> Result<PortfolioEvent, NormalizationError> {
        let wallet = raw_event.get_string("wallet")?;
        let mint = raw_event.get_string("mint")?;
        let pnl_delta = raw_event.get_f64("pnl_delta")?;

        Ok(PortfolioEvent::PositionUpdate {
            wallet,
            mint,
            pnl_delta,
            timestamp: chrono::Utc::now(),
        })
    }

    // Step 4: Normalize swap execution events
    fn normalize_swap_executed(
        &self,
        raw_event: RawEvent,
    ) -> Result<PortfolioEvent, NormalizationError> {
        let wallet = raw_event.get_string("wallet")?;
        let input_mint = raw_event.get_string("input_mint")?;
        let output_mint = raw_event.get_string("output_mint")?;
        let amount = raw_event.get_u64("amount")?;

        Ok(PortfolioEvent::SwapExecuted {
            wallet,
            input_mint,
            output_mint,
            amount,
            timestamp: chrono::Utc::now(),
        })
    }
}

// Step 5: Raw event structure
pub struct RawEvent {
    pub event_type: String,
    pub data: std::collections::HashMap<String, serde_json::Value>,
}

impl RawEvent {
    pub fn new(event_type: String) -> Self {
        Self {
            event_type,
            data: std::collections::HashMap::new(),
        }
    }

    // Step 6: Helper methods to extract typed data
    fn get_string(&self, key: &str) -> Result<String, NormalizationError> {
        self.data
            .get(key)
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .ok_or(NormalizationError::MissingField(key.to_string()))
    }

    fn get_f64(&self, key: &str) -> Result<f64, NormalizationError> {
        self.data
            .get(key)
            .and_then(|v| v.as_f64())
            .ok_or(NormalizationError::MissingField(key.to_string()))
    }

    fn get_u64(&self, key: &str) -> Result<u64, NormalizationError> {
        self.data
            .get(key)
            .and_then(|v| v.as_u64())
            .ok_or(NormalizationError::MissingField(key.to_string()))
    }
}

// Step 7: Normalization error types
#[derive(Debug)]
pub enum NormalizationError {
    UnknownEventType,
    MissingField(String),
}

impl std::fmt::Display for NormalizationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::UnknownEventType => write!(f, "Unknown event type"),
            Self::MissingField(field) => write!(f, "Missing field: {}", field),
        }
    }
}

impl std::error::Error for NormalizationError {}
