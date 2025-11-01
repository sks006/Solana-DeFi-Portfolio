// backend/src/services/metrics.rs
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

// Step 1: Metrics service for monitoring
#[derive(Clone)]
pub struct MetricsService {
    metrics: Arc<RwLock<HashMap<String, MetricValue>>>,
}

// Step 2: Different types of metric values
#[derive(Clone, Debug)]
pub enum MetricValue {
    Counter(u64),
    Gauge(f64),
}

impl MetricsService {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    // Step 3: Increment counter metric
    pub async fn increment_counter(&self, name: &str, value: u64) {
        let mut metrics = self.metrics.write().await;
        let entry = metrics
            .entry(name.to_string())
            .or_insert(MetricValue::Counter(0));

        if let MetricValue::Counter(ref mut count) = entry {
            *count += value;
        }
    }

    // Step 4: Set gauge metric
    pub async fn set_gauge(&self, name: &str, value: f64) {
        let mut metrics = self.metrics.write().await;
        metrics.insert(name.to_string(), MetricValue::Gauge(value));
    }

    // Step 5: Get all metrics for export
    pub async fn get_metrics(&self) -> HashMap<String, MetricValue> {
        self.metrics.read().await.clone()
    }

    // Step 6: Record API request metrics
    pub async fn record_api_request(&self, endpoint: &str, status_code: u16, _duration_ms: f64) {
        self.increment_counter(
            &format!(
                "api_requests_total,endpoint={},status={}",
                endpoint, status_code
            ),
            1,
        )
        .await;
    }

    // Step 7: Record WebSocket connection metrics
    pub async fn record_websocket_connection(&self) {
        self.increment_counter("websocket_connections_total", 1)
            .await;
    }
}
