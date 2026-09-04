use std::time::Duration;

use axum::extract::ws::{Message, WebSocket, WebSocketUpgrade};
use axum::extract::{Query, State};
use axum::response::IntoResponse;
use business::commons::token_context::with_forwarded_token;
use business::gateway::conversation_gateway::ConversationGateway;
use business::use_cases::authentication::Authentication;
use business::use_cases::chat_use_case::ChatUseCase;
use domain::user::User;
use futures::{SinkExt, StreamExt};
use serde::Deserialize;

use crate::infrastructure::chat_hub::ServerEvent;
use crate::infrastructure::mapper::MessageMapper;
use crate::AppState;

#[derive(Debug, Deserialize)]
pub struct WsAuthQuery {
    pub access_token: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
enum ClientFrame {
    #[serde(rename_all = "camelCase")]
    Send {
        conversation_uuid: String,
        #[serde(default)]
        body: String,
        #[serde(default)]
        media: Vec<crate::http::json::chat_json::MessageMediaJson>,
        client_message_id: String,
    },
    #[serde(rename_all = "camelCase")]
    Read {
        conversation_uuid: String,
        last_read_message_uuid: String,
    },
    #[serde(rename_all = "camelCase")]
    Typing {
        conversation_uuid: String,
    },
    Ping,
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
                    Some(text) => {
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

        let frame: ClientFrame = match serde_json::from_str(&text) {
            Ok(f) => f,
            Err(_) => {
                state.chat_hub.publish(
                    std::slice::from_ref(&person_uuid),
                    &ServerEvent::Error {
                        message: "Malformed frame".to_string(),
                    },
                );
                continue;
            }
        };

        handle_frame(&state, &user, &token, frame).await;
    }

    outbound.abort();
    state.chat_hub.unregister(&person_uuid, conn_id);
}

async fn handle_frame(state: &AppState, user: &User, token: &str, frame: ClientFrame) {
    match frame {
        ClientFrame::Ping => {
            state
                .chat_hub
                .publish(std::slice::from_ref(&user.person_uuid), &ServerEvent::Pong);
        }
        ClientFrame::Send {
            conversation_uuid,
            body,
            media,
            client_message_id,
        } => {
            let domain_media = media.into_iter().map(MessageMapper::to_domain_media).collect();
            // gRPC calls inside the use case need the forwarded JWT.
            let result = with_forwarded_token(Some(token.to_string()), async {
                ChatUseCase::send_message(
                    &state.database,
                    user,
                    &conversation_uuid,
                    &body,
                    domain_media,
                    &client_message_id,
                )
                .await
            })
            .await;

            match result {
                Ok(outcome) => {
                    let json = MessageMapper::json(outcome.message);
                    state.chat_hub.publish(
                        &outcome.recipients,
                        &ServerEvent::MessageNew {
                            conversation_uuid,
                            conversation_type: outcome.conversation_type,
                            message: json,
                        },
                    );
                }
                Err(e) => state.chat_hub.publish(
                    std::slice::from_ref(&user.person_uuid),
                    &ServerEvent::Error { message: e.message },
                ),
            }
        }
        ClientFrame::Read {
            conversation_uuid,
            last_read_message_uuid,
        } => {
            let result = with_forwarded_token(Some(token.to_string()), async {
                ChatUseCase::mark_read(
                    &state.database,
                    user,
                    &conversation_uuid,
                    &last_read_message_uuid,
                )
                .await
            })
            .await;

            if let Ok(outcome) = result {
                state.chat_hub.publish(
                    &outcome.recipients,
                    &ServerEvent::MessageRead {
                        conversation_uuid: outcome.conversation_uuid,
                        person_uuid: outcome.reader_person_uuid,
                        last_read_message_uuid: outcome.last_read_message_uuid,
                    },
                );
            }
        }
        ClientFrame::Typing { conversation_uuid } => {
            // Ephemeral: forward to the other participants, nothing persisted.
            let Ok(Some(conversation)) = ConversationGateway::new(&state.database)
                .find_by_uuid(&conversation_uuid)
                .await
            else {
                return;
            };
            if !conversation
                .participant_person_uuids
                .iter()
                .any(|u| u == &user.person_uuid)
            {
                return;
            }
            let others: Vec<String> = conversation
                .participant_person_uuids
                .into_iter()
                .filter(|u| u != &user.person_uuid)
                .collect();
            state.chat_hub.publish(
                &others,
                &ServerEvent::Typing {
                    conversation_uuid,
                    person_uuid: user.person_uuid.clone(),
                },
            );
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ClientFrame;

    /// The whole API speaks camelCase — REST, gRPC and the socket. Serde's
    /// enum-level `rename_all` renames variants, not their fields, so each
    /// variant needs its own. Without it every frame the app sends fails to
    /// deserialize and is dropped, and the client never learns it happened.
    #[test]
    fn client_frames_parse_camel_case_fields() {
        let send: ClientFrame = serde_json::from_str(
            r#"{"type":"send","conversationUuid":"c1","body":"oi","media":[],"clientMessageId":"m1"}"#,
        )
        .expect("send frame");
        match send {
            ClientFrame::Send {
                conversation_uuid,
                client_message_id,
                body,
                ..
            } => {
                assert_eq!(conversation_uuid, "c1");
                assert_eq!(client_message_id, "m1");
                assert_eq!(body, "oi");
            }
            other => panic!("wrong variant: {other:?}"),
        }

        let read: ClientFrame = serde_json::from_str(
            r#"{"type":"read","conversationUuid":"c1","lastReadMessageUuid":"m9"}"#,
        )
        .expect("read frame");
        assert!(matches!(read, ClientFrame::Read { .. }));

        let typing: ClientFrame =
            serde_json::from_str(r#"{"type":"typing","conversationUuid":"c1"}"#)
                .expect("typing frame");
        assert!(matches!(typing, ClientFrame::Typing { .. }));
    }
}
