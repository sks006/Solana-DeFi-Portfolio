use axum::{
    extract::{State, WebSocketUpgrade},
    response::Response,
};
use axum::extract::ws::{Message, WebSocket};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::BackendAppState;

/// Step 1️⃣: WebSocket entrypoint — Axum will upgrade to WS
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<BackendAppState>) -> Response {
    ws.on_upgrade(move |socket| handle_websocket_connection(socket, state))
}

/// Step 2️⃣: Manage individual client connection
async fn handle_websocket_connection(mut socket: WebSocket, mut state: BackendAppState) {
    let client_id = Uuid::new_v4();
    tracing::info!("🔌 WebSocket client connected: {}", client_id);

    // Add to hub + metrics
    state.ws_hub.add_client(client_id, None);
    state.metrics.record_websocket_connection().await;

    let mut rx = state.ws_hub.subscribe();

    // Split socket for concurrent read/write
    let (mut sender, mut receiver) = socket.split();

    // 📨 Task: broadcast messages to client
    let send_task = tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            if let Ok(text) = serde_json::to_string(&message) {
                if sender.send(Message::Text(text)).await.is_err() {
                    tracing::warn!("⚠️ WS send failed, closing connection");
                    break;
                }
            }
        }
    });

    // 💬 Task: handle messages from client
    let recv_task = {
        let state_clone = state.clone();
        tokio::spawn(async move {
            while let Some(Ok(msg)) = receiver.next().await {
                match msg {
                    Message::Text(text) => {
                        handle_client_message(&text, &state_clone).await;
                    }
                    Message::Ping(_) => {
                        tracing::debug!("📡 received ping");
                    }
                    Message::Close(_) => {
                        tracing::info!("❌ client closed WS");
                        break;
                    }
                    _ => {}
                }
            }
        })
    };

    // 🕓 Keep alive until one side closes
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // 🧹 Cleanup
    tracing::info!("💤 WebSocket disconnected: {}", client_id);
    state.ws_hub.remove_client(&client_id);
}

/// Step 3️⃣: Handle specific client-side messages
async fn handle_client_message(text: &str, state: &BackendAppState) {
    if let Ok(msg) = serde_json::from_str::<ClientMessage>(text) {
        match msg {
            ClientMessage::SubscribeWallet { wallet } => {
                tracing::info!("👤 Client subscribed to wallet: {}", wallet);
            }
            ClientMessage::UnsubscribeWallet { wallet } => {
                tracing::info!("👤 Client unsubscribed: {}", wallet);
            }
            ClientMessage::Ping => {
                // respond with pong via broadcast
                let pong = crate::ws::hub::WsMessage {
                    message_type: "pong".to_string(),
                    payload: serde_json::json!({}),
                    timestamp: chrono::Utc::now(),
                };
                let _ = state.ws_hub.broadcast(pong);
            }
        }
    } else {
        tracing::warn!("⚠️ Unknown WS message: {}", text);
    }
}

/// Step 4️⃣: Incoming client message types
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    SubscribeWallet { wallet: String },
    UnsubscribeWallet { wallet: String },
    Ping,
}
