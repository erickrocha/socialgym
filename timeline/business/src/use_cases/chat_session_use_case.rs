use domain::business_error::BusinessError;
use domain::message::{Message, MessageMedia};
use domain::user::User;
use mongodb::Database;

use crate::commons::chat_hub::{ChatEvent, ChatHub};
use crate::commons::token_context::with_forwarded_token;
use crate::gateway::conversation_gateway::ConversationGateway;
use crate::use_cases::chat_use_case::ChatUseCase;

/// What a connected client can ask for over a live chat connection, in domain terms: the
/// WebSocket and the gRPC stream both translate their wire frames into this.
#[derive(Debug)]
pub enum ClientFrame {
    Send {
        conversation_uuid: String,
        body: String,
        media: Vec<MessageMedia>,
        client_message_id: String,
    },
    Read {
        conversation_uuid: String,
        last_read_message_uuid: String,
    },
    Typing {
        conversation_uuid: String,
    },
    Ping,
}

/// Chat operations that also tell the live connections about their outcome, so every transport
/// applies the same participant, friendship and roster checks and notifies the same people.
pub struct ChatSessionUseCase;

impl ChatSessionUseCase {
    /// Stores a message and pushes it to the conversation's other live connections.
    pub async fn send_message(
        hub: &ChatHub,
        db: &Database,
        user: &User,
        conversation_uuid: &str,
        body: &str,
        media: Vec<MessageMedia>,
        client_message_id: &str,
    ) -> Result<Message, BusinessError> {
        let outcome =
            ChatUseCase::send_message(db, user, conversation_uuid, body, media, client_message_id)
                .await?;
        hub.publish(
            &outcome.recipients,
            &ChatEvent::MessageNew {
                conversation_uuid: conversation_uuid.to_string(),
                conversation_type: outcome.conversation_type,
                message: outcome.message.clone(),
            },
        );
        Ok(outcome.message)
    }

    /// Moves the caller's read pointer and tells the other participants.
    pub async fn mark_read(
        hub: &ChatHub,
        db: &Database,
        user: &User,
        conversation_uuid: &str,
        last_read_message_uuid: &str,
    ) -> Result<(), BusinessError> {
        let outcome =
            ChatUseCase::mark_read(db, user, conversation_uuid, last_read_message_uuid).await?;
        hub.publish(
            &outcome.recipients,
            &ChatEvent::MessageRead {
                conversation_uuid: outcome.conversation_uuid,
                person_uuid: outcome.reader_person_uuid,
                last_read_message_uuid: outcome.last_read_message_uuid,
            },
        );
        Ok(())
    }

    /// Runs one client frame for `user`, publishing its outcome through the hub. `token` is the
    /// caller's bearer token, forwarded to the calls the use cases make to `workout`.
    pub async fn handle_frame(
        hub: &ChatHub,
        db: &Database,
        user: &User,
        token: &str,
        frame: ClientFrame,
    ) {
        let me = std::slice::from_ref(&user.person_uuid);
        match frame {
            ClientFrame::Ping => hub.publish(me, &ChatEvent::Pong),
            ClientFrame::Send {
                conversation_uuid,
                body,
                media,
                client_message_id,
            } => {
                let sent = with_forwarded_token(Some(token.to_string()), async {
                    Self::send_message(
                        hub,
                        db,
                        user,
                        &conversation_uuid,
                        &body,
                        media,
                        &client_message_id,
                    )
                    .await
                })
                .await;
                if let Err(error) = sent {
                    hub.publish(
                        me,
                        &ChatEvent::Error {
                            message: error.message,
                        },
                    );
                }
            }
            ClientFrame::Read {
                conversation_uuid,
                last_read_message_uuid,
            } => {
                // A failed read pointer is not worth an error frame: the client retries on its next read.
                let _ = with_forwarded_token(Some(token.to_string()), async {
                    Self::mark_read(hub, db, user, &conversation_uuid, &last_read_message_uuid)
                        .await
                })
                .await;
            }
            ClientFrame::Typing { conversation_uuid } => {
                // Ephemeral: forward to the other participants, nothing persisted.
                let Ok(Some(conversation)) = ConversationGateway::new(db)
                    .find_by_uuid(&conversation_uuid)
                    .await
                else {
                    return;
                };
                if !conversation
                    .participant_person_uuids
                    .iter()
                    .any(|uuid| uuid == &user.person_uuid)
                {
                    return;
                }
                let others: Vec<String> = conversation
                    .participant_person_uuids
                    .into_iter()
                    .filter(|uuid| uuid != &user.person_uuid)
                    .collect();
                hub.publish(
                    &others,
                    &ChatEvent::Typing {
                        conversation_uuid,
                        person_uuid: user.person_uuid.clone(),
                    },
                );
            }
        }
    }

    /// The online subset of `candidates` that `user` may see (friends and conversation peers), at
    /// most 200 asked about at once. Anyone else is simply absent, like an offline person.
    pub async fn presence_online(
        hub: &ChatHub,
        db: &Database,
        user: &User,
        candidates: Vec<String>,
    ) -> Result<Vec<String>, BusinessError> {
        let candidates: Vec<String> = candidates.into_iter().take(200).collect();
        let visible = ChatUseCase::visible_presence_candidates(db, user, candidates).await?;
        Ok(hub.online_among(&visible))
    }
}
