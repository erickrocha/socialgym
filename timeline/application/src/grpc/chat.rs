//! ChatService: the eight unary operations of `/timeline/api/chat` and the bidirectional stream
//! that replaces the WebSocket for the mobile app. The stream and the WebSocket share one frame
//! handler (`chat_session::handle_frame`), so the participant, friendship and roster checks
//! cannot differ.
use super::auth::{BearerToken, caller, with_caller};
use super::convert::*;
use super::rate_limit::enforce;
use super::services::Services;
use super::status::status_from_business;
use crate::authentication::rate_limit::chat_limiter;
use crate::http::chat_controller::single_conversation_json;
use crate::infrastructure::chat_session::{self, presence_online};
use crate::infrastructure::mapper::{ConversationMapper, MessageMapper};
use business::proto::proto::timeline::chat_service_server::ChatService;
use business::proto::proto::timeline::*;
use business::use_cases::authentication::Authentication;
use business::use_cases::chat_use_case::ChatUseCase;
use domain::business_error::BusinessError;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

fn failed(error: BusinessError) -> Status {
    status_from_business(&error)
}

#[tonic::async_trait]
impl ChatService for Services {
    async fn list_conversations(&self, request: Request<ListConversationsRequest>) -> Result<Response<ListConversationsResponse>, Status> {
        let (db, page) = (self.state.database.clone(), request.get_ref().page);
        with_caller(&request, |user| async move {
            let views = ChatUseCase::list_conversations(&db, &user, page).await.map_err(failed)?;
            Ok(Response::new(ListConversationsResponse {
                conversations: views.into_iter().map(|v| conversation_to(ConversationMapper::view(v))).collect(),
            }))
        })
        .await
    }

    async fn get_presence(&self, request: Request<GetPresenceRequest>) -> Result<Response<GetPresenceResponse>, Status> {
        let (state, uuids) = (self.state.clone(), request.get_ref().person_uuids.clone());
        with_caller(&request, |user| async move {
            let online = presence_online(&state, &user, uuids).await.map_err(failed)?;
            Ok(Response::new(GetPresenceResponse { online }))
        })
        .await
    }

    async fn create_direct_conversation(&self, request: Request<CreateDirectConversationRequest>) -> Result<Response<Conversation>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (state, target) = (self.state.clone(), request.get_ref().target_person_uuid.clone());
        with_caller(&request, |user| async move {
            let conversation = ChatUseCase::get_or_create_direct(&state.database, &user, &target).await.map_err(failed)?;
            Ok(Response::new(conversation_to(single_conversation_json(&state, &user, conversation).await)))
        })
        .await
    }

    async fn create_business_team_group(&self, request: Request<CreateBusinessTeamGroupRequest>) -> Result<Response<Conversation>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (state, profile) = (self.state.clone(), request.get_ref().business_profile_uuid.clone());
        with_caller(&request, |user| async move {
            let conversation = ChatUseCase::get_or_create_business_team_group(&state.database, &user, &profile).await.map_err(failed)?;
            Ok(Response::new(conversation_to(single_conversation_json(&state, &user, conversation).await)))
        })
        .await
    }

    async fn create_business_direct_conversation(&self, request: Request<CreateBusinessDirectConversationRequest>) -> Result<Response<Conversation>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (state, body) = (self.state.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let conversation = ChatUseCase::get_or_create_business_direct(
                &state.database,
                &user,
                &body.business_profile_uuid,
                body.member_person_uuid.as_deref(),
            )
            .await
            .map_err(failed)?;
            Ok(Response::new(conversation_to(single_conversation_json(&state, &user, conversation).await)))
        })
        .await
    }

    async fn list_messages(&self, request: Request<ListMessagesRequest>) -> Result<Response<ListMessagesResponse>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (db, body) = (self.state.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let messages = if body.since_epoch_ms > 0 {
                ChatUseCase::list_messages_since(&db, &user, &body.conversation_uuid, body.since_epoch_ms).await
            } else {
                ChatUseCase::list_messages(&db, &user, &body.conversation_uuid, body.page).await
            }
            .map_err(failed)?;
            Ok(Response::new(ListMessagesResponse {
                messages: messages.into_iter().map(|m| message_to(MessageMapper::json(m))).collect(),
            }))
        })
        .await
    }

    async fn send_message(&self, request: Request<SendMessageRequest>) -> Result<Response<Message>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (state, body) = (self.state.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let media = body.media.into_iter().map(|m| MessageMapper::to_domain_media(message_media_from(m))).collect();
            let outcome = ChatUseCase::send_message(&state.database, &user, &body.conversation_uuid, &body.body, media, &body.client_message_id)
                .await
                .map_err(failed)?;
            let json = MessageMapper::json(outcome.message);
            state.chat_hub.publish(
                &outcome.recipients,
                &crate::infrastructure::chat_hub::ServerEvent::MessageNew {
                    conversation_uuid: body.conversation_uuid,
                    conversation_type: outcome.conversation_type,
                    message: json.clone(),
                },
            );
            Ok(Response::new(message_to(json)))
        })
        .await
    }

    async fn mark_conversation_read(&self, request: Request<MarkConversationReadRequest>) -> Result<Response<MarkConversationReadResponse>, Status> {
        let (state, body) = (self.state.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let outcome = ChatUseCase::mark_read(&state.database, &user, &body.conversation_uuid, &body.last_read_message_uuid)
                .await
                .map_err(failed)?;
            state.chat_hub.publish(
                &outcome.recipients,
                &crate::infrastructure::chat_hub::ServerEvent::MessageRead {
                    conversation_uuid: outcome.conversation_uuid,
                    person_uuid: outcome.reader_person_uuid,
                    last_read_message_uuid: outcome.last_read_message_uuid,
                },
            );
            Ok(Response::new(MarkConversationReadResponse { read: true }))
        })
        .await
    }

    type OpenStreamStream = ReceiverStream<Result<ServerFrame, Status>>;

    async fn open_stream(&self, request: Request<Streaming<ClientFrame>>) -> Result<Response<Self::OpenStreamStream>, Status> {
        // Same gate as every other operation: valid token, then current Terms and Privacy consent.
        with_caller(&request, |_| async { Ok(()) }).await?;
        let (user, BearerToken(token)) = caller(&request)?;
        let lifetime = Authentication::access_token_expiry(&token)
            .map(|exp| Duration::from_secs((exp - chrono::Utc::now().timestamp()).max(0) as u64))
            .unwrap_or(Duration::from_secs(3600));

        let state = self.state.clone();
        let (connection, mut events) = state.chat_hub.register(&user.person_uuid);
        let (out, response) = mpsc::channel(64);
        let mut inbound = request.into_inner();
        tokio::spawn(async move {
            let expiry = tokio::time::sleep(lifetime);
            tokio::pin!(expiry);
            loop {
                tokio::select! {
                    frame = inbound.message() => match frame {
                        Ok(Some(frame)) => {
                            if let Some(frame) = client_frame_from(frame) {
                                chat_session::handle_frame(&state, &user, &token, frame).await;
                            }
                        }
                        // The client closed or cancelled the stream, or the transport failed.
                        _ => break,
                    },
                    event = events.recv() => match event {
                        Some(text) => {
                            if let Some(frame) = server_frame_from_hub_text(&text) {
                                if out.send(Ok(frame)).await.is_err() {
                                    break;
                                }
                            }
                        }
                        None => break,
                    },
                    _ = &mut expiry => {
                        let _ = out.send(Err(Status::unauthenticated("access token expired"))).await;
                        break;
                    }
                }
            }
            state.chat_hub.unregister(&user.person_uuid, connection);
        });
        Ok(Response::new(ReceiverStream::new(response)))
    }
}
