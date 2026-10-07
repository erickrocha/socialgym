use business::commons::token_context::with_forwarded_token;
use business::gateway::conversation_gateway::ConversationGateway;
use business::use_cases::chat_use_case::ChatUseCase;
use domain::business_error::BusinessError;
use domain::user::User;
use serde::Deserialize;

use crate::infrastructure::chat_hub::ServerEvent;
use crate::infrastructure::mapper::MessageMapper;
use crate::AppState;

#[derive(Debug, Deserialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ClientFrame {
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

/// Runs one client frame for `user`, publishing its outcome through the hub. Shared by the
/// WebSocket and the gRPC stream so both apply the same participant, friendship and roster checks.
pub async fn handle_frame(state: &AppState, user: &User, token: &str, frame: ClientFrame) {
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


/// The online subset of `candidates` that `user` may see (friends and conversation peers), at most
/// 200 asked about at once. Anyone else is simply absent, like an offline person.
pub async fn presence_online(state: &AppState, user: &User, candidates: Vec<String>) -> Result<Vec<String>, BusinessError> {
    let candidates: Vec<String> = candidates.into_iter().take(200).collect();
    let visible = ChatUseCase::visible_presence_candidates(&state.database, user, candidates).await?;
    Ok(state.chat_hub.online_among(&visible))
}
