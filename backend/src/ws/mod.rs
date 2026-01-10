use axum::{
    extract::ws::{Message, WebSocket, WebSocketUpgrade},
    extract::State,
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::broadcast;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct WebSocketMessage {
    pub event_type: String,
    pub payload: serde_json::Value,
}

pub struct Broadcaster {
    tx: broadcast::Sender<WebSocketMessage>,
}

impl Broadcaster {
    pub fn new(capacity: usize) -> Self {
        let (tx, _) = broadcast::channel(capacity);
        Self { tx }
    }

    pub fn broadcast(&self, msg: WebSocketMessage) {
        // We ignore the error if there are no subscribers
        let _ = self.tx.send(msg);
    }

    pub fn subscribe(&self) -> broadcast::Receiver<WebSocketMessage> {
        self.tx.subscribe()
    }
}

pub async fn ws_handler(
    ws: WebSocketUpgrade,
    State(state): State<crate::AppState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_socket(socket, state.broadcaster))
}

async fn handle_socket(mut socket: WebSocket, broadcaster: Arc<Broadcaster>) {
    let mut rx = broadcaster.subscribe();

    loop {
        tokio::select! {
            msg = rx.recv() => {
                match msg {
                    Ok(ws_msg) => {
                        if let Ok(json) = serde_json::to_string(&ws_msg) {
                            if socket.send(Message::Text(json.into())).await.is_err() {
                                break;
                            }
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(_)) => {
                        // Client is too slow, we could send a message or just ignore
                        continue;
                    }
                    Err(broadcast::error::RecvError::Closed) => break,
                }
            }
            result = socket.recv() => {
                match result {
                    Some(Ok(Message::Close(_))) | None | Some(Err(_)) => break,
                    _ => continue,
                }
            }
        }
    }
}
