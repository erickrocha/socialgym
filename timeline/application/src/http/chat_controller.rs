use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::chat_json::{
    ChatMessagesQuery, ChatPageQuery, ConversationJson, CreateBusinessDirectJson,
    CreateBusinessTeamGroupJson, CreateDirectConversationJson, MarkReadJson, MarkReadResultJson,
    MessageJson, PresenceJson, PresenceQuery, SendMessageJson,
};
use crate::infrastructure::chat_hub::ServerEvent;
use crate::infrastructure::mapper::{ConversationMapper, MessageMapper};
use crate::AppState;
use axum::extract::{Extension, Path, Query, State};
use axum::Json;
use business::use_cases::chat_use_case::{ChatUseCase, ConversationView};
use domain::user::User;

fn business_err(locale: Locale) -> impl Fn(domain::business_error::BusinessError) -> ExceptionResponse {
    move |error| ExceptionResponse::from_business(error, locale, ErrorKey::Unknown)
}

async fn single_conversation_json(
    state: &AppState,
    user: &User,
    conversation: domain::conversation::Conversation,
) -> ConversationJson {
    // Reuse the list mapper so a freshly created conversation renders exactly
    // like it will in the list (signed logo url, unread=false for the creator).
    let mut conversation = conversation;
    if let Some(key) = conversation.business_profile_logo_object_key.clone() {
        if !key.is_empty() {
            if let Ok(url) =
                business::use_cases::media_use_case::MediaUseCase::generate_cloud_front_signed_url(
                    &key,
                )
                .await
            {
                conversation.business_profile_logo_object_key = Some(url);
            }
        }
    }
    let _ = state;
    let _ = user;
    ConversationMapper::view(ConversationView {
        conversation,
        unread: false,
    })
}

#[utoipa::path(
    post,
    path = "/timeline/api/chat/conversations/direct",
    request_body = CreateDirectConversationJson,
    responses((status = 200, description = "Conversation", body = ConversationJson)),
    security(("api_key" = []))
)]
pub async fn create_direct(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateDirectConversationJson>,
) -> HttpResponse<Json<ConversationJson>> {
    let conversation =
        ChatUseCase::get_or_create_direct(&state.database, &user, &payload.target_person_uuid)
            .await
            .map_err(business_err(locale))?;
    Ok(Json(
        single_conversation_json(&state, &user, conversation).await,
    ))
}

#[utoipa::path(
    post,
    path = "/timeline/api/chat/conversations/business-team",
    request_body = CreateBusinessTeamGroupJson,
    responses((status = 200, description = "Conversation", body = ConversationJson)),
    security(("api_key" = []))
)]
pub async fn create_business_team_group(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateBusinessTeamGroupJson>,
) -> HttpResponse<Json<ConversationJson>> {
    let conversation = ChatUseCase::get_or_create_business_team_group(
        &state.database,
        &user,
        &payload.business_profile_uuid,
    )
    .await
    .map_err(business_err(locale))?;
    Ok(Json(
        single_conversation_json(&state, &user, conversation).await,
    ))
}

#[utoipa::path(
    post,
    path = "/timeline/api/chat/conversations/business-direct",
    request_body = CreateBusinessDirectJson,
    responses((status = 200, description = "Conversation", body = ConversationJson)),
    security(("api_key" = []))
)]
pub async fn create_business_direct(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Json(payload): Json<CreateBusinessDirectJson>,
) -> HttpResponse<Json<ConversationJson>> {
    let conversation = ChatUseCase::get_or_create_business_direct(
        &state.database,
        &user,
        &payload.business_profile_uuid,
        payload.member_person_uuid.as_deref(),
    )
    .await
    .map_err(business_err(locale))?;
    Ok(Json(
        single_conversation_json(&state, &user, conversation).await,
    ))
}

#[utoipa::path(
    get,
    path = "/timeline/api/chat/conversations",
    params(("page" = Option<u32>, Query, description = "Zero-based page")),
    responses((status = 200, description = "Conversations", body = [ConversationJson])),
    security(("api_key" = []))
)]
pub async fn list_conversations(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Query(query): Query<ChatPageQuery>,
) -> HttpResponse<Json<Vec<ConversationJson>>> {
    let views =
        ChatUseCase::list_conversations(&state.database, &user, query.page.unwrap_or(0))
            .await
            .map_err(business_err(locale))?;
    Ok(Json(views.into_iter().map(ConversationMapper::view).collect()))
}

#[utoipa::path(
    get,
    path = "/timeline/api/chat/conversations/{conversation_uuid}/messages",
    params(
        ("conversation_uuid" = String, Path, description = "Conversation uuid"),
        ("page" = Option<u32>, Query, description = "Zero-based page"),
        ("since" = Option<i64>, Query, description = "Epoch ms; return messages newer than this"),
    ),
    responses((status = 200, description = "Messages", body = [MessageJson])),
    security(("api_key" = []))
)]
pub async fn list_messages(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Path(conversation_uuid): Path<String>,
    Query(query): Query<ChatMessagesQuery>,
) -> HttpResponse<Json<Vec<MessageJson>>> {
    let messages = match query.since {
        Some(since) => {
            ChatUseCase::list_messages_since(&state.database, &user, &conversation_uuid, since)
                .await
        }
        None => {
            ChatUseCase::list_messages(
                &state.database,
                &user,
                &conversation_uuid,
                query.page.unwrap_or(0),
            )
            .await
        }
    }
    .map_err(business_err(locale))?;

    Ok(Json(messages.into_iter().map(MessageMapper::json).collect()))
}

#[utoipa::path(
    post,
    path = "/timeline/api/chat/conversations/{conversation_uuid}/messages",
    params(("conversation_uuid" = String, Path, description = "Conversation uuid")),
    request_body = SendMessageJson,
    responses((status = 200, description = "Sent message", body = MessageJson)),
    security(("api_key" = []))
)]
pub async fn send_message(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Path(conversation_uuid): Path<String>,
    Json(payload): Json<SendMessageJson>,
) -> HttpResponse<Json<MessageJson>> {
    let media = payload
        .media
        .into_iter()
        .map(MessageMapper::to_domain_media)
        .collect();

    let outcome = ChatUseCase::send_message(
        &state.database,
        &user,
        &conversation_uuid,
        &payload.body,
        media,
        &payload.client_message_id,
    )
    .await
    .map_err(business_err(locale))?;

    let json = MessageMapper::json(outcome.message);
    state.chat_hub.publish(
        &outcome.recipients,
        &ServerEvent::MessageNew {
            conversation_uuid: conversation_uuid.clone(),
            conversation_type: outcome.conversation_type,
            message: json.clone(),
        },
    );
    Ok(Json(json))
}

#[utoipa::path(
    put,
    path = "/timeline/api/chat/conversations/{conversation_uuid}/read",
    params(("conversation_uuid" = String, Path, description = "Conversation uuid")),
    request_body = MarkReadJson,
    responses((status = 200, description = "Marked read", body = MarkReadResultJson)),
    security(("api_key" = []))
)]
pub async fn mark_read(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(user): Extension<User>,
    Path(conversation_uuid): Path<String>,
    Json(payload): Json<MarkReadJson>,
) -> HttpResponse<Json<MarkReadResultJson>> {
    let outcome = ChatUseCase::mark_read(
        &state.database,
        &user,
        &conversation_uuid,
        &payload.last_read_message_uuid,
    )
    .await
    .map_err(business_err(locale))?;

    state.chat_hub.publish(
        &outcome.recipients,
        &ServerEvent::MessageRead {
            conversation_uuid: outcome.conversation_uuid,
            person_uuid: outcome.reader_person_uuid,
            last_read_message_uuid: outcome.last_read_message_uuid,
        },
    );
    Ok(Json(MarkReadResultJson { read: true }))
}

/// Online/offline for a batch of people. Callers poll this when they render a
/// list of people to message — there is no presence push on the socket.
#[utoipa::path(
    get,
    path = "/timeline/api/chat/presence",
    params(("uuids" = String, Query, description = "Comma-separated person uuids")),
    responses((status = 200, description = "Online subset", body = PresenceJson)),
    security(("api_key" = []))
)]
pub async fn presence(
    state: State<AppState>,
    Extension(_user): Extension<User>,
    Query(query): Query<PresenceQuery>,
) -> HttpResponse<Json<PresenceJson>> {
    // ponytail: cap the batch so a caller cannot ask about the whole userbase
    // in one request; paginate client-side if a list ever grows past this.
    let candidates: Vec<String> = query
        .uuids
        .split(',')
        .map(|uuid| uuid.trim().to_string())
        .filter(|uuid| !uuid.is_empty())
        .take(200)
        .collect();
    Ok(Json(PresenceJson {
        online: state.chat_hub.online_among(&candidates),
    }))
}
