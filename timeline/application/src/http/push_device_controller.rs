use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::push_device_json::RegisterPushDeviceJson;
use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use business::use_cases::push_device_use_case::PushDeviceUseCase;
use domain::user::User;

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
    PushDeviceUseCase::register(
        &state.database,
        &current_user.person_uuid,
        &device_uuid,
        &payload.platform,
        &payload.registration_token,
    )
    .await
    .map_err(|error| ExceptionResponse::from_business(error, Locale::En, ErrorKey::Unknown))?;

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
    PushDeviceUseCase::remove(&state.database, &current_user.person_uuid, &device_uuid)
        .await
        .map_err(|error| ExceptionResponse::from_business(error, Locale::En, ErrorKey::Unknown))?;
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
#[path = "../tests/push_device_controller_unit_test.rs"]
mod tests;
