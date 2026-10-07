use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use business::commons::chat_hub::ChatEvent;
use business::use_cases::authentication::Authentication;
use business::use_cases::chat_session_use_case::ChatSessionUseCase;
use domain::user::User;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;

use crate::http::json::chat_json::ClientFrameJson;
use crate::infrastructure::mapper::{ClientFrameMapper, ServerEventMapper};
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct WsAuthQuery {
    pub access_token: Option<String>,
}

/// `GET /timeline/api/chat/ws?access_token=<jwt>`
///
/// Browser `WebSocket` can't set an `Authorization` header, so the token comes
/// in the query string; it is validated with the same code path as the REST
/// auth middleware. Only reachable over `wss://`.
pub async fn ws(
    ws: WebSocketUpgrade,
    State(state): State<AppState>,
    Query(query): Query<WsAuthQuery>,
) -> impl IntoResponse {
    let token = match query.access_token {
        Some(t) if !t.trim().is_empty() => t,
        _ => {
            return axum::http::StatusCode::UNAUTHORIZED.into_response();
        }
    };

    let user = match Authentication::validate(token.clone()).await {
        Ok(user) => user,
        Err(_) => return axum::http::StatusCode::UNAUTHORIZED.into_response(),
    };

    // Same mandatory consents as the REST middleware: a person who has not accepted them
    // cannot open the stream.
    let consents = business::commons::token_context::with_forwarded_token(Some(token.clone()), async {
        business::gateway::consent_gateway::ConsentGateway::require("terms").await?;
        business::gateway::consent_gateway::ConsentGateway::require("privacy").await
    })
    .await;
    if let Err(error) = consents {
        return if error.kind == domain::business_error::BusinessErrorKind::Infrastructure {
            axum::http::StatusCode::INTERNAL_SERVER_ERROR
        } else {
            axum::http::StatusCode::FORBIDDEN
        }
        .into_response();
    }

    ws.on_upgrade(move |socket| handle_socket(socket, state, user, token))
}

async fn handle_socket(socket: WebSocket, state: AppState, user: User, token: String) {
    let person_uuid = user.person_uuid.clone();
    let (conn_id, mut rx) = state.chat_hub.register(&person_uuid);
    let (mut sink, mut stream) = socket.split();

    // Task 1: hub -> client, plus a keepalive ping.
    let mut ping = tokio::time::interval(Duration::from_secs(30));
    let outbound = tokio::spawn(async move {
        loop {
            tokio::select! {
                frame = rx.recv() => match frame {
                    Some(event) => {
                        let text = ServerEventMapper::json(event).to_frame();
                        if sink.send(Message::Text(text.into())).await.is_err() {
                            break;
                        }
                    }
                    None => break,
                },
                _ = ping.tick() => {
                    if sink.send(Message::Ping(Vec::new().into())).await.is_err() {
                        break;
                    }
                }
            }
        }
    });

    // Task 2 (this task): client -> server.
    while let Some(Ok(message)) = stream.next().await {
        let text = match message {
            Message::Text(t) => t.to_string(),
            Message::Close(_) => break,
            Message::Ping(_) | Message::Pong(_) | Message::Binary(_) => continue,
        };

        let frame: ClientFrameJson = match serde_json::from_str(&text) {
            Ok(f) => f,
            Err(_) => {
                state.chat_hub.publish(
                    std::slice::from_ref(&person_uuid),
                    &ChatEvent::Error {
                        message: "Malformed frame".to_string(),
                    },
                );
                continue;
            }
        };

        ChatSessionUseCase::handle_frame(&state.chat_hub, &state.database, &user, &token, ClientFrameMapper::domain(frame)).await;
    }

    outbound.abort();
    state.chat_hub.unregister(&person_uuid, conn_id);
}

#[cfg(test)]
#[path = "../tests/chat_ws_handler_unit_test.rs"]
mod tests;
