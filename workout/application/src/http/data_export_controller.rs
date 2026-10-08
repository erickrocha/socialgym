use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::data_export_json::{DataExportDownloadJson, DataExportJson};
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use business::domain::user::User;
use business::use_cases::data_export_use_case::DataExportUseCase;

/// How each failure of the export use case reads over REST.
fn failure(error: BusinessError, locale: Locale) -> ExceptionResponse {
    match error.kind {
        BusinessErrorKind::Validation => ExceptionResponse::BadRequest(locale, ErrorKey::InvalidParameterValue),
        BusinessErrorKind::NotFound => ExceptionResponse::NotFound(locale, ErrorKey::DataExportNotReady),
        BusinessErrorKind::Conflict => ExceptionResponse::Conflict(locale, ErrorKey::DataExportNotReady),
        _ => ExceptionResponse::InternalServerError(locale, ErrorKey::DataExportFailed),
    }
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<DataExportJson>> {
    DataExportUseCase::create(&state.conn, user.person_id)
        .await
        .map(|row| Json(DataExportJson::from(row)))
        .map_err(|e| failure(e, locale))
}

pub async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<DataExportJson>>> {
    DataExportUseCase::list(&state.conn, user.person_id)
        .await
        .map(|rows| Json(rows.into_iter().map(DataExportJson::from).collect()))
        .map_err(|e| failure(e, locale))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<DataExportJson>> {
    DataExportUseCase::get(&state.conn, user.person_id, &id)
        .await
        .map(|row| Json(DataExportJson::from(row)))
        .map_err(|e| failure(e, locale))
}

pub async fn download(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<DataExportDownloadJson>> {
    DataExportUseCase::download(&state.conn, user.person_id, &id)
        .await
        .map(|link| Json(DataExportDownloadJson { url: link.url, expires_in_seconds: link.expires_in_seconds }))
        .map_err(|e| failure(e, locale))
}
