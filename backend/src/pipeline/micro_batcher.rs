// backend/src/pipeline/micro_batcher.rs
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;

use crate::models::event::PortfolioEvent;

// Step 1: Micro-batching processor for efficient event handling
pub struct MicroBatcher {
    receiver: mpsc::Receiver<PortfolioEvent>,
    batch_size: usize,
    batch_timeout: Duration,
}

impl MicroBatcher {
    pub fn new(
        receiver: mpsc::Receiver<PortfolioEvent>,
        batch_size: usize,
        batch_timeout: Duration,
    ) -> Self {
        Self {
            receiver,
            batch_size,
            batch_timeout,
        }
    }

    // Step 2: Start processing events - FIXED VERSION
    pub async fn run(mut self) {
        tracing::info!(
            "🔄 Starting micro-batcher (batch_size: {}, timeout: {:?})",
            self.batch_size,
            self.batch_timeout
        );

        loop {
            let mut batch = Vec::with_capacity(self.batch_size);
            let mut channel_closed = false;
            let deadline = tokio::time::sleep(self.batch_timeout);
            tokio::pin!(deadline);

            // Step 3: Collect events until batch size, timeout, or channel closed
            while batch.len() < self.batch_size && !channel_closed {
                tokio::select! {
                    maybe = self.receiver.recv() => {
                        match maybe {
                            Some(event) => batch.push(event),
                            None => {
                                // Channel closed
                                channel_closed = true;
                                break;
                            }
                        }
                    }
                    _ = &mut deadline => {
                        // Timeout elapsed
                        break;
                    }
                }
            }

            // Step 4: If no events, decide whether to continue waiting or shutdown
            if batch.is_empty() {
                if channel_closed {
                    tracing::info!("📭 Event channel closed, shutting down batcher");
                    return;
                } else {
                    continue; // No events received before timeout, continue waiting
                }
            }

            // Step 5: Process batch
            self.process_batch(batch).await;
        }
    }

    // Step 6: Process a batch of events
    async fn process_batch(&self, batch: Vec<PortfolioEvent>) {
        tracing::debug!("🔄 Processing batch of {} events", batch.len());

        // Step 7: Group events by wallet for efficient processing
        let mut events_by_wallet = HashMap::new();
        for event in batch {
            let wallet = event.wallet().to_string();
            events_by_wallet
                .entry(wallet)
                .or_insert_with(Vec::new)
                .push(event);
        }

        // Step 8: Process each wallet's events in parallel
        let mut tasks = Vec::new();

        for (wallet, events) in events_by_wallet {
            let wallet_clone = wallet.clone();
            let events_clone = events.clone();

            let task = tokio::spawn(async move {
                if let Err(e) = Self::process_wallet_events(&wallet_clone, events_clone).await {
                    tracing::error!(
                        "❌ Error processing events for wallet {}: {}",
                        wallet_clone,
                        e
                    );
                }
            });

            tasks.push(task);
        }

        // Wait for all wallet processing to complete
        for task in tasks {
            let _ = task.await;
        }

        tracing::debug!("✅ Batch processing completed");
    }

    // Step 9: Process events for a specific wallet
    async fn process_wallet_events(
        wallet: &str,
        events: Vec<PortfolioEvent>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        for event in events {
            match event {
                PortfolioEvent::PositionUpdate {
                    wallet,
                    mint,
                    pnl_delta,
                    timestamp,
                } => {
                    tracing::info!(
                        "📈 Updating position for {}: {} PnL delta: {}",
                        wallet,
                        mint,
                        pnl_delta
                    );
                    // In production: Update database, trigger risk recalculation
                }
                PortfolioEvent::SwapExecuted {
                    wallet,
                    input_mint,
                    output_mint,
                    amount,
                    timestamp,
                } => {
                    tracing::info!(
                        "💱 Swap executed for {}: {} {} -> {}",
                        wallet,
                        amount,
                        input_mint,
                        output_mint
                    );
                    // In production: Update portfolio, trigger liquidity analysis
                }
                PortfolioEvent::RiskAlertTriggered {
                    wallet,
                    alert_type,
                    severity,
                    message,
                    timestamp,
                } => {
                    tracing::info!("🚨 Risk alert for {}: {} - {}", wallet, severity, message);
                    // In production: Store alert, send notifications
                }
            }
        }

        Ok(())
    }
}
