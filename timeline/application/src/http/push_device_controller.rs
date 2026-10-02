use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::push_device_json::RegisterPushDeviceJson;
use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use business::gateway::push_device_gateway::PushDeviceGateway;
use domain::user::User;
use uuid::Uuid;

#[utoipa::path(
    put,
    path = "/timeline/api/push-devices/{device_uuid}",
    params(("device_uuid" = String, Path, description = "Stable installation UUID")),
    request_body = RegisterPushDeviceJson,
    responses(
        (status = 204, description = "Push device registered; registration token is never returned"),
        (status = 400, description = "Invalid device UUID, platform, or token"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_key" = []))
)]
pub async fn register_push_device(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(device_uuid): Path<String>,
    Json(payload): Json<RegisterPushDeviceJson>,
) -> HttpResponse<StatusCode> {
    if Uuid::parse_str(&device_uuid).is_err()
        || !matches!(payload.platform.as_str(), "android" | "ios")
        || payload.registration_token.trim().is_empty()
        || payload.registration_token.len() > 4096
    {
        return Err(ExceptionResponse::bad_request(
            Locale::En,
            ErrorKey::Unknown,
        ));
    }

    PushDeviceGateway::register(
        &state.database,
        &device_uuid,
        &current_user.person_uuid,
        &payload.platform,
        &payload.registration_token,
    )
    .await
    .map_err(|_| ExceptionResponse::internal_server_error(Locale::En, ErrorKey::Unknown))?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "/timeline/api/push-devices/{device_uuid}",
    params(("device_uuid" = String, Path, description = "Stable installation UUID")),
    responses(
        (status = 204, description = "Push device removed"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Device not registered for this owner"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_key" = []))
)]
pub async fn remove_push_device(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(device_uuid): Path<String>,
) -> HttpResponse<StatusCode> {
    let removed =
        PushDeviceGateway::remove_owned(&state.database, &device_uuid, &current_user.person_uuid)
            .await
            .map_err(|_| ExceptionResponse::internal_server_error(Locale::En, ErrorKey::Unknown))?;

    if !removed {
        return Err(ExceptionResponse::NotFound(Locale::En, ErrorKey::Unknown));
    }
    Ok(StatusCode::NO_CONTENT)
}
