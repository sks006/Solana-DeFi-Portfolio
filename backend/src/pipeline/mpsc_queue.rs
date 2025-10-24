// backend/src/pipeline/mpsc_queue.rs
use std::time::Duration;
use tokio::sync::mpsc;

use crate::models::event::PortfolioEvent;

// Step 1: Multi-producer single-consumer queue with backpressure
pub struct EventQueue {
    sender: mpsc::Sender<PortfolioEvent>,
    receiver: mpsc::Receiver<PortfolioEvent>,
    capacity: usize,
    metrics: QueueMetrics,
}

// Step 2: Queue performance metrics
#[derive(Debug, Clone)]
struct QueueMetrics {
    pub total_received: u64,
    pub total_processed: u64,
    pub current_size: usize,
    pub max_capacity: usize,
}

impl EventQueue {
    // Step 3: Create new queue with specified capacity
    pub fn new(capacity: usize) -> Self {
        let (sender, receiver) = mpsc::channel(capacity);

        Self {
            sender,
            receiver,
            capacity,
            metrics: QueueMetrics {
                total_received: 0,
                total_processed: 0,
                current_size: 0,
                max_capacity: capacity,
            },
        }
    }

    // Step 4: Send event to queue with backpressure
    pub async fn send(&mut self, event: PortfolioEvent) -> Result<(), QueueError> {
        if self.metrics.current_size >= self.capacity {
            return Err(QueueError::Full);
        }

        match tokio::time::timeout(Duration::from_secs(5), self.sender.send(event)).await {
            Ok(Ok(_)) => {
                self.metrics.total_received += 1;
                self.metrics.current_size += 1;
                Ok(())
            }
            Ok(Err(_)) => Err(QueueError::Closed),
            Err(_) => Err(QueueError::Timeout),
        }
    }

    // Step 5: Receive next event from queue
    pub async fn recv(&mut self) -> Option<PortfolioEvent> {
        match self.receiver.recv().await {
            Some(event) => {
                self.metrics.total_processed += 1;
                self.metrics.current_size -= 1;
                Some(event)
            }
            None => None,
        }
    }

    // Step 6: Get queue metrics
    pub fn metrics(&self) -> &QueueMetrics {
        &self.metrics
    }
}

// Step 7: Queue error types
#[derive(Debug)]
pub enum QueueError {
    Full,
    Closed,
    Timeout,
}

impl std::fmt::Display for QueueError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Full => write!(f, "Queue is full"),
            Self::Closed => write!(f, "Queue is closed"),
            Self::Timeout => write!(f, "Send operation timed out"),
        }
    }
}

impl std::error::Error for QueueError {}
