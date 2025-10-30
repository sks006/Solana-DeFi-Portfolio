// backend/src/ws/client.rs
use axum::{
     extract::{State, WebSocketUpgrade},
    response::Response,
};
use futures_util::{SinkExt, StreamExt};
use uuid::Uuid;

use crate::BackendAppState;

// Step 1: WebSocket connection handler
pub async fn ws_handler(ws: WebSocketUpgrade, State(state): State<BackendAppState>) -> Response {
    ws.on_upgrade(|socket| async move { handle_websocket_connection(socket, state).await })
}

// Step 2: Handle individual WebSocket connection
async fn handle_websocket_connection(
    socket: axum::extract::ws::WebSocket,
    mut state: BackendAppState,
) {
    let client_id = Uuid::new_v4();

    // Step 3: Add client to hub
    state.ws_hub.add_client(client_id, None);
    state.metrics.record_websocket_connection().await;

    // Step 4: Subscribe to broadcast messages
    let mut rx = state.ws_hub.subscribe();

    // Step 5: Split socket for sending and receiving
    let (mut sender, mut receiver) = socket.split();

    let send_task = tokio::spawn(async move {
        while let Ok(message) = rx.recv().await {
            // Step 6: Send messages to client
            if let Ok(text) = serde_json::to_string(&message) {
                let _ = sender.send(axum::extract::ws::Message::Text(text)).await;
            }
        }
    });

    let recv_task = {
        let state_clone = state.clone();
        tokio::spawn(async move {
            while let Some(Ok(message)) = receiver.next().await {
                if let axum::extract::ws::Message::Text(text) = message {
                    handle_client_message(&text, &state_clone).await;
                }
            }
        })
    };

    // Step 8: Wait for either task to complete
    tokio::select! {
        _ = send_task => {},
        _ = recv_task => {},
    }

    // Step 9: Clean up on disconnect
    state.ws_hub.remove_client(&client_id);
}

// Step 10: Handle different types of client messages
async fn handle_client_message(text: &str, state: &BackendAppState) {
    if let Ok(message) = serde_json::from_str::<ClientMessage>(text) {
        match message {
            ClientMessage::SubscribeWallet { wallet } => {
                tracing::info!("👤 Client subscribed to wallet: {}", wallet);
                // In production, track wallet subscriptions
            }
            ClientMessage::UnsubscribeWallet { wallet } => {
                tracing::info!("👤 Client unsubscribed from wallet: {}", wallet);
                // In production, remove wallet subscriptions
            }
            ClientMessage::Ping => {
                // Respond to ping with pong
                let pong_message = crate::ws::hub::WsMessage {
                    message_type: "pong".to_string(),
                    payload: serde_json::json!({}),
                    timestamp: chrono::Utc::now(),
                };
                let _ = state.ws_hub.broadcast(pong_message);
            }
        }
    }
}

// Step 11: Client message types
#[derive(serde::Deserialize)]
#[serde(tag = "type")]
enum ClientMessage {
    SubscribeWallet { wallet: String },
    UnsubscribeWallet { wallet: String },
    Ping,
}
