// backend/src/services/ai_client.rs
use crate::config::AIConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};

// Step 1: Risk analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAnalysis {
    pub risk_score: f64,
    pub alerts: Vec<RiskAlert>,
    pub recommendations: Vec<String>,
}

// Step 2: Risk alert from AI service
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAlert {
    pub severity: String,
    pub message: String,
    pub metric: String,
    pub value: f64,
}

// Step 3: AI client service
#[derive(Debug, Clone)]
pub struct AIClient {
    client: Client,
    base_url: String,
    enabled: bool,
}

impl AIClient {
    pub async fn new(config: &AIConfig) -> Self {
        if !config.enabled {
            tracing::warn!("🤖 AI Service is disabled");
            return Self {
                client: Client::new(),
                base_url: "".to_string(),
                enabled: false,
            };
        }

        tracing::info!(
            "🤖 AI Service configured: {} (timeout: {}s)",
            config.url,
            config.timeout_seconds
        );

        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(config.timeout_seconds))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            client,
            base_url: config.url.clone(),
            enabled: config.enabled,
        }
    }

    // Update methods to check if enabled
    pub async fn assess_portfolio_risk(
        &self,
        _wallet: &str,
        positions: &[crate::server_functions::portfolio::Position],
    ) -> Result<f64, Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(self.calculate_fallback_risk_score(positions));
        }

        // If enabled, attempt to call remote AI service; fall back on local heuristic on any failure.
        let features = self.extract_portfolio_features(positions);
        let url = format!("{}/assess", self.base_url.trim_end_matches('/'));

        let resp = self.client.post(&url).json(&features).send().await;

        match resp {
            Ok(r) if r.status().is_success() => {
                let json: serde_json::Value = r.json().await.unwrap_or(serde_json::json!({}));
                if let Some(score) = json.get("risk_score").and_then(|v| v.as_f64()) {
                    return Ok(score);
                }
                Ok(self.calculate_fallback_risk_score(positions))
            }
            _ => Ok(self.calculate_fallback_risk_score(positions)),
        }
    }

    // Step 7: Analyze position risk
    pub async fn analyze_position_risk(
        &self,
        _wallet: &str,
        positions: &[crate::server_functions::risk::PositionForAnalysis],
    ) -> Result<RiskAnalysis, Box<dyn std::error::Error>> {
        // Mock implementation
        let risk_score = self.calculate_fallback_risk_score_from_analysis(positions);

        let alerts = if risk_score > 0.7 {
            vec![RiskAlert {
                severity: "HIGH".to_string(),
                message: "High concentration risk detected".to_string(),
                metric: "concentration".to_string(),
                value: 0.85,
            }]
        } else {
            vec![]
        };

        let recommendations = vec![
            "Consider diversifying across different asset types".to_string(),
            "Set stop-loss orders for high-risk positions".to_string(),
        ];

        Ok(RiskAnalysis {
            risk_score,
            alerts,
            recommendations,
        })
    }

    // Step 8: Extract portfolio features for AI model
    fn extract_portfolio_features(
        &self,
        positions: &[crate::server_functions::portfolio::Position],
    ) -> std::collections::HashMap<String, f64> {
        let total_value: f64 = positions.iter().map(|p| p.value_usd).sum();
        let max_position = positions.iter().map(|p| p.value_usd).fold(0.0, f64::max);

        let concentration = if total_value > 0.0 {
            max_position / total_value
        } else {
            0.0
        };

        let mut features = std::collections::HashMap::new();
        features.insert("total_value".to_string(), total_value);
        features.insert("concentration".to_string(), concentration);
        features.insert("num_positions".to_string(), positions.len() as f64);

        features
    }

    // Step 9: Fallback risk calculation
    fn calculate_fallback_risk_score(
        &self,
        positions: &[crate::server_functions::portfolio::Position],
    ) -> f64 {
        let total_value: f64 = positions.iter().map(|p| p.value_usd).sum();
        if total_value == 0.0 {
            return 0.0;
        }

        let max_position = positions.iter().map(|p| p.value_usd).fold(0.0, f64::max);

        let concentration = max_position / total_value;

        // Simple risk heuristic
        if concentration > 0.5 {
            0.8
        } else if concentration > 0.3 {
            0.5
        } else {
            0.2
        }
    }

    fn calculate_fallback_risk_score_from_analysis(
        &self,
        positions: &[crate::server_functions::risk::PositionForAnalysis],
    ) -> f64 {
        let total_volatility: f64 = positions.iter().map(|p| p.volatility * p.value_usd).sum();
        let total_value: f64 = positions.iter().map(|p| p.value_usd).sum();

        if total_value == 0.0 {
            return 0.0;
        }

        let weighted_volatility = total_volatility / total_value;
        weighted_volatility.min(1.0)
    }
}
