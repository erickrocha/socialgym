//! ChatService: the eight unary operations of `/timeline/api/chat` and the bidirectional stream
//! that replaces the WebSocket for the mobile app. The stream and the WebSocket share one frame
//! handler (`ChatSessionUseCase::handle_frame`), so the participant, friendship and roster checks
//! cannot differ.
use crate::auth::grpc_auth::BearerToken;
use crate::infrastructure::mapper::{ChatFrameMapper, ConversationMapper, MessageMapper};
use crate::infrastructure::utils::{business_status, caller, enforce, with_caller};
use crate::proto::timeline::chat_service_server::ChatService;
use crate::proto::timeline::*;
use business::commons::chat_hub::ChatHub;
use business::commons::rate_limit::chat_limiter;
use business::use_cases::authentication::Authentication;
use business::use_cases::chat_session_use_case::ChatSessionUseCase;
use business::use_cases::chat_use_case::ChatUseCase;
use mongodb::Database;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::{Request, Response, Status, Streaming};

pub struct GrpcChatService {
    database: Arc<Database>,
    chat_hub: ChatHub,
}

impl GrpcChatService {
    pub fn new(database: Arc<Database>, chat_hub: ChatHub) -> Self {
        Self { database, chat_hub }
    }
}

#[tonic::async_trait]
impl ChatService for GrpcChatService {
    async fn list_conversations(
        &self,
        request: Request<ListConversationsRequest>,
    ) -> Result<Response<ListConversationsResponse>, Status> {
        let (db, page) = (self.database.clone(), request.get_ref().page);
        with_caller(&request, |user| async move {
            let views = ChatUseCase::list_conversations(&db, &user, page)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(ListConversationsResponse {
                conversations: views.into_iter().map(ConversationMapper::proto).collect(),
            }))
        })
        .await
    }

    async fn get_presence(
        &self,
        request: Request<GetPresenceRequest>,
    ) -> Result<Response<GetPresenceResponse>, Status> {
        let (db, hub, uuids) = (
            self.database.clone(),
            self.chat_hub.clone(),
            request.get_ref().person_uuids.clone(),
        );
        with_caller(&request, |user| async move {
            let online = ChatSessionUseCase::presence_online(&hub, &db, &user, uuids)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(GetPresenceResponse { online }))
        })
        .await
    }

    async fn create_direct_conversation(
        &self,
        request: Request<CreateDirectConversationRequest>,
    ) -> Result<Response<Conversation>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (db, target) = (
            self.database.clone(),
            request.get_ref().target_person_uuid.clone(),
        );
        with_caller(&request, |user| async move {
            let conversation = ChatUseCase::get_or_create_direct(&db, &user, &target)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(ConversationMapper::proto(
                ChatUseCase::view_of(conversation).await,
            )))
        })
        .await
    }

    async fn create_business_team_group(
        &self,
        request: Request<CreateBusinessTeamGroupRequest>,
    ) -> Result<Response<Conversation>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (db, profile) = (
            self.database.clone(),
            request.get_ref().business_profile_uuid.clone(),
        );
        with_caller(&request, |user| async move {
            let conversation = ChatUseCase::get_or_create_business_team_group(&db, &user, &profile)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(ConversationMapper::proto(
                ChatUseCase::view_of(conversation).await,
            )))
        })
        .await
    }

    async fn create_business_direct_conversation(
        &self,
        request: Request<CreateBusinessDirectConversationRequest>,
    ) -> Result<Response<Conversation>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let conversation = ChatUseCase::get_or_create_business_direct(
                &db,
                &user,
                &body.business_profile_uuid,
                body.member_person_uuid.as_deref(),
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(ConversationMapper::proto(
                ChatUseCase::view_of(conversation).await,
            )))
        })
        .await
    }

    async fn list_messages(
        &self,
        request: Request<ListMessagesRequest>,
    ) -> Result<Response<ListMessagesResponse>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let messages = if body.since_epoch_ms > 0 {
                ChatUseCase::list_messages_since(
                    &db,
                    &user,
                    &body.conversation_uuid,
                    body.since_epoch_ms,
                )
                .await
            } else {
                ChatUseCase::list_messages(&db, &user, &body.conversation_uuid, body.page).await
            }
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(ListMessagesResponse {
                messages: messages.into_iter().map(MessageMapper::proto).collect(),
            }))
        })
        .await
    }

    async fn send_message(
        &self,
        request: Request<SendMessageRequest>,
    ) -> Result<Response<Message>, Status> {
        enforce(&chat_limiter(), &request)?;
        let (db, hub, body) = (
            self.database.clone(),
            self.chat_hub.clone(),
            request.get_ref().clone(),
        );
        with_caller(&request, |user| async move {
            let media = body
                .media
                .into_iter()
                .map(MessageMapper::media_domain)
                .collect();
            let message = ChatSessionUseCase::send_message(
                &hub,
                &db,
                &user,
                &body.conversation_uuid,
                &body.body,
                media,
                &body.client_message_id,
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(MessageMapper::proto(message)))
        })
        .await
    }

    async fn mark_conversation_read(
        &self,
        request: Request<MarkConversationReadRequest>,
    ) -> Result<Response<MarkConversationReadResponse>, Status> {
        let (db, hub, body) = (
            self.database.clone(),
            self.chat_hub.clone(),
            request.get_ref().clone(),
        );
        with_caller(&request, |user| async move {
            ChatSessionUseCase::mark_read(
                &hub,
                &db,
                &user,
                &body.conversation_uuid,
                &body.last_read_message_uuid,
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(MarkConversationReadResponse { read: true }))
        })
        .await
    }

    type OpenStreamStream = ReceiverStream<Result<ServerFrame, Status>>;

    async fn open_stream(
        &self,
        request: Request<Streaming<ClientFrame>>,
    ) -> Result<Response<Self::OpenStreamStream>, Status> {
        // Same gate as every other operation: valid token, then current Terms and Privacy consent.
        with_caller(&request, |_| async { Ok(()) }).await?;
        let (user, BearerToken(token)) = caller(&request)?;
        let lifetime = Authentication::access_token_expiry(&token)
            .map(|exp| Duration::from_secs((exp - chrono::Utc::now().timestamp()).max(0) as u64))
            .unwrap_or(Duration::from_secs(3600));

        let (db, hub) = (self.database.clone(), self.chat_hub.clone());
        let (connection, mut events) = hub.register(&user.person_uuid);
        let (out, response) = mpsc::channel(64);
        let mut inbound = request.into_inner();
        tokio::spawn(async move {
            let expiry = tokio::time::sleep(lifetime);
            tokio::pin!(expiry);
            loop {
                tokio::select! {
                    frame = inbound.message() => match frame {
                        Ok(Some(frame)) => {
                            if let Some(frame) = ChatFrameMapper::client_frame(frame) {
                                ChatSessionUseCase::handle_frame(&hub, &db, &user, &token, frame).await;
                            }
                        }
                        // The client closed or cancelled the stream, or the transport failed.
                        _ => break,
                    },
                    event = events.recv() => match event {
                        Some(event) => {
                            if out.send(Ok(ChatFrameMapper::server_frame(event))).await.is_err() {
                                break;
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
            hub.unregister(&user.person_uuid, connection);
        });
        Ok(Response::new(ReceiverStream::new(response)))
    }
}
