use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::notification_json::{MarkNotificationReadJson, NotificationJson};
use crate::infrastructure::mapper::{Mapper, NotificationMapper};
use axum::Json;
use axum::extract::{Path, Query, State};
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct NotificationQuery {
    pub unread_only: Option<bool>,
    pub limit: Option<i64>,
}

#[utoipa::path(
    get,
    path = "/timeline/api/notifications/{owner_uuid}",
    params(
        ("owner_uuid" = String, Path, description = "Owner UUID of the notifications"),
        ("unread_only" = Option<bool>, Query, description = "Return only unread notifications"),
        ("limit" = Option<i64>, Query, description = "Maximum number of notifications to return"),
    ),
    responses(
        (status = 200, description = "Notification list", body = [NotificationJson]),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_key" = []))
)]
pub async fn list_notifications(
    state: State<AppState>,
    Query(query): Query<NotificationQuery>,
    Path(owner_uuid): Path<String>,
) -> HttpResponse<Json<Vec<NotificationJson>>> {
    let unread_only = query.unread_only.unwrap_or(false);
    let limit = query.limit.unwrap_or(50).clamp(1, 100);

    let notifications = MentionNotificationUseCase::list_notifications(
        &state.database,
        &owner_uuid,
        unread_only,
        limit,
    )
    .await
    .map_err(|_e| ExceptionResponse::internal_server_error(Locale::En, ErrorKey::Unknown))?;

    Ok(Json(
        notifications
            .into_iter()
            .map(NotificationMapper::json)
            .collect(),
    ))
}

#[utoipa::path(
    put,
    path = "/timeline/api/notifications/{owner_uuid}/read/{idempotency_key}",
    params(
        ("owner_uuid" = String, Path, description = "Owner UUID of the notifications"),
        ("idempotency_key" = String, Path, description = "Notification idempotency key"),
    ),
    request_body = MarkNotificationReadJson,
    responses(
        (status = 200, description = "Notification marked as read", body = MarkNotificationReadJson),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Notification not found"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_key" = []))
)]
pub async fn mark_notification_read(
    state: State<AppState>,
    Path((owner_uuid, idempotency_key)): Path<(String, String)>,
) -> HttpResponse<Json<MarkNotificationReadJson>> {
    let updated =
        MentionNotificationUseCase::mark_as_read(&state.database, &owner_uuid, &idempotency_key)
            .await
            .map_err(|_e| {
                ExceptionResponse::internal_server_error(Locale::En, ErrorKey::Unknown)
            })?;

    if !updated {
        return Err(ExceptionResponse::bad_request(
            Locale::En,
            ErrorKey::Unknown,
        ));
    }

    Ok(Json(MarkNotificationReadJson { read: true }))
}
