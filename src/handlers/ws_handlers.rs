use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        Query, State,
    },
    response::Response,
};
use futures_util::{sink::SinkExt, stream::StreamExt};
use serde::Deserialize;
use uuid::Uuid;

use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct WebSocketQuery {
    pub schema_id: Option<Uuid>,
}

pub async fn ws_handler(
    State(state): State<AppState>,
    Query(query): Query<WebSocketQuery>,
    ws: WebSocketUpgrade,
) -> Response {
    ws.on_upgrade(move |socket| handle_socket(socket, state, query))
}

async fn handle_socket(socket: WebSocket, state: AppState, query: WebSocketQuery) {
    let (mut sender, mut receiver) = socket.split();
    let mut rx = state.log_broadcast.subscribe();

    let mut send_task = tokio::spawn(async move {
        while let Ok(log_event) = rx.recv().await {
            let should_send = match &query.schema_id {
                Some(schema_id) => log_event.schema_id() == *schema_id,
                None => true,
            };

            if should_send {
                if let Ok(json) = serde_json::to_string(&log_event) {
                    if sender.send(Message::Text(json.into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    let mut recv_task = tokio::spawn(async move {
        while let Some(Ok(msg)) = receiver.next().await {
            match msg {
                Message::Close(_) => {
                    break;
                }
                Message::Ping(ping) => {
                    tracing::debug!("Received ping: {:?}", ping);
                }
                Message::Pong(_) => {}
                Message::Text(text) => {
                    tracing::debug!("Received text message: {}", text);
                }
                _ => {}
            }
        }
    });

    tokio::select! {
        _ = &mut send_task => {
            tracing::debug!("Send task completed");
            recv_task.abort();
        },
        _ = &mut recv_task => {
            tracing::debug!("Receive task completed");
            send_task.abort();
        },
    }

    tracing::info!("WebSocket connection closed");
}
