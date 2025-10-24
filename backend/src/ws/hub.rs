// backend/src/ws/hub.rs
use serde::Serialize;
use std::collections::HashMap;
use tokio::sync::broadcast;
use uuid::Uuid;

// Step 1: WebSocket hub for managing connections
#[derive(Debug, Clone)]
pub struct WsHub {
    tx: broadcast::Sender<WsMessage>,
    clients: HashMap<Uuid, ClientInfo>,
}

// Step 2: WebSocket message structure
#[derive(Debug, Clone, Serialize)]
pub struct WsMessage {
    pub message_type: String,
    pub payload: serde_json::Value,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// Step 3: Client connection information
#[derive(Debug, Clone)]
struct ClientInfo {
    pub wallet: Option<String>,
    pub connected_at: chrono::DateTime<chrono::Utc>,
}

impl WsHub {
    pub fn new() -> Self {
        let (tx, _) = broadcast::channel(100);
        Self {
            tx,
            clients: HashMap::new(),
        }
    }

    // Step 4: Subscribe to messages
    pub fn subscribe(&self) -> broadcast::Receiver<WsMessage> {
        self.tx.subscribe()
    }

    // Step 5: Broadcast message to all clients
    pub fn broadcast(
        &self,
        message: WsMessage,
    ) -> Result<(), broadcast::error::SendError<WsMessage>> {
        self.tx.send(message).map(|_| ())
    }

    // Step 6: Add client to hub
    pub fn add_client(&mut self, id: Uuid, wallet: Option<String>) {
        self.clients.insert(
            id,
            ClientInfo {
                wallet,
                connected_at: chrono::Utc::now(),
            },
        );
        tracing::info!("➕ Client connected: {}", id);
    }

    // Step 7: Remove client from hub
    pub fn remove_client(&mut self, id: &Uuid) {
        self.clients.remove(id);
        tracing::info!("➖ Client disconnected: {}", id);
    }

    // Step 8: Get connected clients count
    pub fn connected_clients(&self) -> usize {
        self.clients.len()
    }
}
