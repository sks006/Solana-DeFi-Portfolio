use crate::config::AIConfig;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use anyhow::Result;

// Step 1: Risk analysis result
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAnalysis {
    pub risk_score: f64,
    pub alerts: Vec<RiskAlert>,
    pub recommendations: Vec<String>,
}

// ✅ Detailed response from AI service
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAnalysisResponse {
    pub risk_score: f64,
    pub risk_level: String,
    pub alerts: Option<Vec<RiskAlert>>,
    pub recommendations: Option<Vec<String>>,
}

// Step 2: Risk alert from AI service
#[derive(Debug, Serialize, Deserialize)]
pub struct RiskAlert {
    pub severity: String,
    pub message: String,
    pub metric: Option<String>,
    pub value: Option<f64>,
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

    // ✅ Updated: Analyze portfolio risk
    pub async fn analyze_portfolio_risk(
        &self,
        wallet: &str,
        positions: &[crate::server_functions::risk::PositionForAnalysis],
        total_value: f64,
        leverage_ratio: f64,
    ) -> Result<RiskAnalysisResponse> {
        if !self.enabled {
            tracing::warn!("⚠️ AI service disabled, using fallback");
            return Ok(RiskAnalysisResponse {
                risk_score: self.calculate_fallback_risk_score_from_analysis(positions),
                risk_level: "fallback".to_string(),
                alerts: Some(vec![RiskAlert {
                    severity: "info".to_string(),
                    message: "AI service disabled, using fallback risk calculation.".to_string(),
                    metric: None,
                    value: None,
                }]),
                recommendations: Some(vec!["Enable AI service for live analysis".to_string()]),
            });
        }

        // 🧠 Prevent invalid payloads
        if total_value <= 0.0 {
            tracing::warn!("⚠️ Skipping AI analysis: total_value <= 0");
            return Ok(RiskAnalysisResponse {
                risk_score: 0.0,
                risk_level: "none".to_string(),
                alerts: Some(vec![RiskAlert {
                    severity: "info".to_string(),
                    message: "Portfolio total_value must be > 0 for AI analysis".to_string(),
                    metric: Some("total_value".to_string()),
                    value: Some(total_value),
                }]),
                recommendations: Some(vec![
                    "Fund portfolio or add positions before analysis".to_string()
                ]),
            });
        }

        // ✅ Correct payload structure (matches Hugging Face API)
        let payload = serde_json::json!({
            "wallet": wallet,
            "total_value": total_value.max(0.0),
            "leverage_ratio": leverage_ratio,
            "positions": positions.iter().map(|p| serde_json::json!({
                "symbol": p.symbol,
                "mint": p.mint,
                "amount": p.amount,
                "value_usd": p.value_usd,
                "volatility": p.volatility
            })).collect::<Vec<_>>()
        });

        let url = format!("{}/analyze/portfolio", self.base_url.trim_end_matches('/'));
        tracing::info!("📡 Sending request to AI: {}", url);

        let response = self.client.post(&url).json(&payload).send().await?;

        if response.status().is_success() {
            let parsed: RiskAnalysisResponse = response.json().await?;
            tracing::info!("✅ AI analysis success: score = {}", parsed.risk_score);
            Ok(parsed)
        } else {
            let err_text = response.text().await.unwrap_or_default();
            tracing::error!("❌ AI request failed: {}", err_text);
            Err(anyhow::anyhow!("AI service error: {}", err_text))
        }
    }

    // ✅ Local fallback (no AI)
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
