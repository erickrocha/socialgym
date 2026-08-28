use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::data_export_json::{DataExportDownloadJson, DataExportJson};
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use business::domain::user::User;
use business::gateway::data_export_gateway::DataExportGateway;
use business::use_cases::image_storage_use_case::ImageStorageUseCase;
use chrono::Utc;

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<DataExportJson>> {
    DataExportGateway::create(&state.conn, user.person_id)
        .await
        .map(|row| Json(DataExportJson::from(row)))
        .map_err(|_| ExceptionResponse::InternalServerError(locale, ErrorKey::DataExportFailed))
}

pub async fn list(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<DataExportJson>>> {
    DataExportGateway::list_for_person(&state.conn, user.person_id)
        .await
        .map(|rows| Json(rows.into_iter().map(DataExportJson::from).collect()))
        .map_err(|_| ExceptionResponse::InternalServerError(locale, ErrorKey::DataExportFailed))
}

pub async fn get(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<DataExportJson>> {
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| ExceptionResponse::BadRequest(locale, ErrorKey::InvalidParameterValue))?;
    DataExportGateway::find_owned(&state.conn, uuid, user.person_id)
        .await
        .map_err(|_| ExceptionResponse::InternalServerError(locale, ErrorKey::DataExportFailed))?
        .map(|row| Json(DataExportJson::from(row)))
        .ok_or(ExceptionResponse::NotFound(
            locale,
            ErrorKey::DataExportNotReady,
        ))
}

pub async fn download(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<DataExportDownloadJson>> {
    let uuid = uuid::Uuid::parse_str(&id)
        .map_err(|_| ExceptionResponse::BadRequest(locale, ErrorKey::InvalidParameterValue))?;
    let row = DataExportGateway::find_owned(&state.conn, uuid, user.person_id)
        .await
        .map_err(|_| ExceptionResponse::InternalServerError(locale, ErrorKey::DataExportFailed))?
        .ok_or(ExceptionResponse::NotFound(
            locale,
            ErrorKey::DataExportNotReady,
        ))?;
    if row.status != "ready" || row.expires_at.is_none_or(|at| at <= Utc::now()) {
        return Err(ExceptionResponse::Conflict(
            locale,
            ErrorKey::DataExportNotReady,
        ));
    }
    let key = row.object_key.ok_or(ExceptionResponse::Conflict(
        locale,
        ErrorKey::DataExportNotReady,
    ))?;
    let url = ImageStorageUseCase::export_download_url(&key)
        .await
        .map_err(|_| ExceptionResponse::InternalServerError(locale, ErrorKey::DataExportFailed))?;
    Ok(Json(DataExportDownloadJson {
        url,
        expires_in_seconds: 900,
    }))
}
