// backend/src/models/risk_alert.rs
use serde::{Deserialize, Serialize};

// Step 1: Risk alert data structure
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAlert {
    pub id: String,
    pub wallet: String,
    pub severity: AlertSeverity,
    pub message: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub metadata: serde_json::Value,
}

// Step 2: Alert severity levels
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertSeverity {
    #[serde(rename = "low")]
    Low,
    #[serde(rename = "medium")]
    Medium,
    #[serde(rename = "high")]
    High,
    #[serde(rename = "critical")]
    Critical,
}

// Step 3: Alert status for management
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AlertStatus {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "acknowledged")]
    Acknowledged,
    #[serde(rename = "resolved")]
    Resolved,
}

// Step 4: Implement helper methods for RiskAlert
impl RiskAlert {
    pub fn new(
        wallet: String,
        severity: AlertSeverity,
        message: String,
        metadata: Option<serde_json::Value>,
    ) -> Self {
        Self {
            id: uuid::Uuid::new_v4().to_string(),
            wallet,
            severity,
            message,
            timestamp: chrono::Utc::now(),
            metadata: metadata.unwrap_or_else(|| serde_json::json!({})),
        }
    }

    pub fn is_high_priority(&self) -> bool {
        matches!(self.severity, AlertSeverity::High | AlertSeverity::Critical)
    }

    pub fn get_summary(&self) -> String {
        format!(
            "[{}] {}: {}",
            self.severity_as_str(),
            self.wallet,
            self.message
        )
    }

    pub fn severity_as_str(&self) -> &str {
        match self.severity {
            AlertSeverity::Low => "LOW",
            AlertSeverity::Medium => "MEDIUM",
            AlertSeverity::High => "HIGH",
            AlertSeverity::Critical => "CRITICAL",
        }
    }
}
