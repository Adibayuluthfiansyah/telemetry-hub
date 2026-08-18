use crate::state::AppState;
use axum::{
    extract::{
        Query, State,
        ws::{Message, WebSocket, WebSocketUpgrade},
    },
    response::Response,
};
use serde::Deserialize;
use tokio::sync::broadcast;
use uuid::Uuid;

#[derive(Debug, Deserialize)]
pub struct StreamParams {
    device_id: Option<Uuid>,
}

pub async fn stream(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(params): Query<StreamParams>,
) -> Response {
    ws.on_upgrade(move |socket| stream_socket(socket, state, params))
}

async fn stream_socket(mut socket: WebSocket, state: AppState, params: StreamParams) {
    let mut rx = state.event_tx.subscribe();
    loop {
        tokio::select! {
                event = rx.recv() => match event {
                    Ok(envelope) => {
                        if params.device_id.is_some_and(|f| envelope.device_id != f) {
                            continue;
                        }
                        let Ok(json) = serde_json::to_string(&envelope) else { continue; };
                        if socket.send(Message::Text(json.into())).await.is_err() {
                            break;
                        }
                    }
                    Err(broadcast::error::RecvError::Lagged(skipped)) => {
                        tracing::warn!(skipped, "stream client fell behind; live only stream ")
                    }
                    Err(broadcast::error::RecvError::Closed) => break
                },
                 incoming = socket.recv() => match incoming {
                    Some(Ok(Message::Close(_))) | None => break,
                    _ => {}
            }
        }
    }
}
