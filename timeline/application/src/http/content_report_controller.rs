use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::use_cases::content_report_use_case::ContentReportUseCase;
use domain::content_report::ContentReport;
use domain::user::User;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateReportJson {
    target_type: String,
    target_id: String,
    post_id: String,
    reason: String,
    details: Option<String>,
}
#[derive(Deserialize)]
pub struct ReportQuery {
    status: Option<String>,
}
#[derive(Deserialize)]
pub struct DecisionJson {
    decision: String,
    reason: String,
}

pub async fn create(
    State(state): State<AppState>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<CreateReportJson>,
) -> HttpResponse<(StatusCode, Json<ContentReport>)> {
    ContentReportUseCase::create(
        &state.database,
        &user.person_uuid,
        payload.target_type,
        payload.target_id,
        payload.post_id,
        payload.reason,
        payload.details,
    )
    .await
    .map(|report| (StatusCode::CREATED, Json(report)))
    .map_err(|e| ExceptionResponse::from_business(e, locale, ErrorKey::Unknown))
}
pub async fn list(
    State(state): State<AppState>,
    Extension(locale): Extension<Locale>,
    Query(query): Query<ReportQuery>,
) -> HttpResponse<Json<Vec<ContentReport>>> {
    ContentReportUseCase::list(&state.database, query.status.as_deref())
        .await
        .map(Json)
        .map_err(|e| ExceptionResponse::from_business(e, locale, ErrorKey::Unknown))
}
pub async fn decide(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Extension(user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<DecisionJson>,
) -> HttpResponse<Json<ContentReport>> {
    ContentReportUseCase::decide(
        &state.database,
        &id,
        &user.person_uuid,
        &payload.decision,
        &payload.reason,
    )
    .await
    .map(Json)
    .map_err(|e| ExceptionResponse::from_business(e, locale, ErrorKey::Unknown))
}
