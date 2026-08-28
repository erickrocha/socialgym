use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::account_deletion_json::{AccountDeletionRequestJson, AccountDeletionStatusJson};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use business::domain::business_error::BusinessErrorKind;
use business::domain::user::User;
use business::use_cases::account_deletion_use_case::AccountDeletionUseCase;

#[utoipa::path(
    post,
    path = "/workout/api/people/me/account/delete",
    request_body = AccountDeletionRequestJson,
    responses(
        (status = 200, description = "Account deletion scheduled", body = AccountDeletionStatusJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn request_account_deletion(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<AccountDeletionRequestJson>,
) -> HttpResponse<Json<AccountDeletionStatusJson>> {
    let result =
        AccountDeletionUseCase::request_deletion(&state.conn, current_user.id.unwrap(), payload.immediate)
            .await;

    match result {
        Ok(status) => Ok(Json(AccountDeletionStatusJson {
            requested_at: status.requested_at.naive_utc(),
            scheduled_at: status.scheduled_at.naive_utc(),
        })),
        Err(_) => Err(ExceptionResponse::InternalServerError(
            locale,
            ErrorKey::AccountDeletionRequestFailed,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/workout/api/people/me/account/cancel-deletion",
    responses(
        (status = 200, description = "Pending account deletion cancelled"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn cancel_account_deletion(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<()>> {
    let result = AccountDeletionUseCase::cancel_deletion(&state.conn, current_user.id.unwrap()).await;

    match result {
        Ok(()) => Ok(Json(())),
        Err(error) => {
            let key = match error.kind {
                BusinessErrorKind::Validation => ErrorKey::AccountDeletionNotPending,
                BusinessErrorKind::NotFound => ErrorKey::AccountDeletionNotPending,
                _ => ErrorKey::AccountDeletionCancelFailed,
            };
            Err(ExceptionResponse::from_business(error, locale, key))
        }
    }
}
