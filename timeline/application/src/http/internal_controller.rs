use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::AppState;
use axum::extract::{Path, State};
use axum::Json;
use business::use_cases::account_data_deletion_use_case::AccountDataDeletionUseCase;

/// Internal, service-to-service endpoint (see `internal_auth_middleware`) that
/// `workout`'s account-deletion sweep calls to cascade-delete every piece of
/// timeline data belonging to a person as part of account deletion.
pub async fn delete_person_data(
    state: State<AppState>,
    Path(person_uuid): Path<String>,
) -> HttpResponse<Json<()>> {
    AccountDataDeletionUseCase::delete_all_for_person(&state.database, &person_uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, Locale::En, ErrorKey::AccountDataDeletionFailed)
        })?;

    Ok(Json(()))
}
