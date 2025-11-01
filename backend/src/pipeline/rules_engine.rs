// backend/src/pipeline/rules_engine.rs
use std::collections::HashMap;

use crate::{models::event::PortfolioEvent, models::risk_alert::RiskAlert, ws::hub::WsHub};

// Step 1: Rules engine for real-time risk detection
pub struct RulesEngine {
    rules: HashMap<String, RiskRule>,
    ws_hub: WsHub,
}

// Step 2: Individual risk rule definition
#[derive(Clone)]
struct RiskRule {
    pub name: String,
    pub condition: RuleCondition,
    pub severity: crate::models::risk_alert::AlertSeverity,
    pub message_template: String,
}

// Step 3: Rule condition types
#[derive(Clone)]
enum RuleCondition {
    PositionConcentration { threshold: f64 },
    LargeTrade { min_amount_usd: f64 },
}

impl RulesEngine {
    pub fn new() -> Self {
        let mut rules = HashMap::new();

        // Step 4: Define risk rules
        rules.insert(
            "high_concentration".to_string(),
            RiskRule {
                name: "High Position Concentration".to_string(),
                condition: RuleCondition::PositionConcentration { threshold: 0.3 },
                severity: crate::models::risk_alert::AlertSeverity::High,
                message_template: "Position {} represents {:.1}% of portfolio".to_string(),
            },
        );

        rules.insert(
            "large_trade".to_string(),
            RiskRule {
                name: "Large Trade Detected".to_string(),
                condition: RuleCondition::LargeTrade {
                    min_amount_usd: 10000.0,
                },
                severity: crate::models::risk_alert::AlertSeverity::Medium,
                message_template: "Large trade of ${:.0} detected".to_string(),
            },
        );

        Self {
            rules,
            ws_hub: WsHub::new(),
        }
    }

    // Step 5: Process events through rules engine
    pub async fn process_event(&self, event: &PortfolioEvent) -> Vec<RiskAlert> {
        let mut alerts = Vec::new();

        for (rule_id, rule) in &self.rules {
            if self.evaluate_condition(rule, event).await {
                let alert = self.create_alert(rule, event).await;
                alerts.push(alert.clone());

                // Step 6: Send real-time alert via WebSocket
                let ws_message = crate::ws::hub::WsMessage {
                    message_type: "risk_alert".to_string(),
                    payload: serde_json::json!({
                        "wallet": alert.wallet,
                        "severity": format!("{:?}", alert.severity),
                        "message": alert.message,
                    }),
                    timestamp: chrono::Utc::now(),
                };

                let _ = self.ws_hub.broadcast(ws_message);
            }
        }

        alerts
    }

    // Step 7: Evaluate individual rule condition
    async fn evaluate_condition(&self, rule: &RiskRule, event: &PortfolioEvent) -> bool {
        match &rule.condition {
            RuleCondition::PositionConcentration { threshold } => {
                self.check_position_concentration(event, *threshold).await
            }
            RuleCondition::LargeTrade { min_amount_usd } => {
                self.check_large_trade(event, *min_amount_usd).await
            }
        }
    }

    // Step 8: Check position concentration rule
    async fn check_position_concentration(&self, event: &PortfolioEvent, threshold: f64) -> bool {
        if let PortfolioEvent::PositionUpdate { pnl_delta, .. } = event {
            // Simplified check - in production, would check actual portfolio concentration
            pnl_delta.abs() > threshold * 1000.0 // Mock logic
        } else {
            false
        }
    }

    // Step 9: Check large trade rule
    async fn check_large_trade(&self, event: &PortfolioEvent, min_amount: f64) -> bool {
        if let PortfolioEvent::SwapExecuted { amount, .. } = event {
            let amount_usd = *amount as f64 * 0.01; // Mock conversion
            amount_usd >= min_amount
        } else {
            false
        }
    }

    // Step 10: Create risk alert from triggered rule
    async fn create_alert(&self, rule: &RiskRule, event: &PortfolioEvent) -> RiskAlert {
        let wallet = event.wallet().to_string();
        let message = rule.message_template.clone();

        RiskAlert::new(
            wallet,
            rule.severity.clone(),
            message,
            Some(serde_json::json!({
                "rule_id": rule.name,
                "triggered_at": chrono::Utc::now().to_rfc3339(),
            })),
        )
    }

    // Step 11: Process alerts in background
    pub async fn process_alerts(self) {
        tracing::info!("🚨 Starting rules engine alert processing");
        // Background processing logic
    }
}
