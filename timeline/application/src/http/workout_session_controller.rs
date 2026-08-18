use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,NotFoundErrorJson
};
use crate::http::json::workout_session_json::WorkoutSessionJson;
use crate::infrastructure::data_tools::opt_naive_to_bson_datetime;
use crate::infrastructure::mapper::{Mapper, WorkoutMapper};
use crate::AppState;
use axum::extract::{Extension, Path, Query,State};
use axum::http::StatusCode;
use axum::Json;
use business::use_cases::workout_use_case::WorkoutSessionUseCase;
use chrono::{Duration, Utc};
use domain::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkoutFilterParams {
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
}

#[utoipa::path(
    post,
    tag = "timeline",
    path = "/timeline/api/workout-sessions",
    request_body = WorkoutSessionJson,
    responses(
        (status = 201, description = "Workout session saved", body = WorkoutSessionJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn create_workout_session(state: State<AppState>,Extension(locale): Extension<Locale>,Json(payload): Json<WorkoutSessionJson>) -> HttpResponse<(StatusCode, Json<WorkoutSessionJson>)> {
    let domain = WorkoutMapper::domain(payload);

    let entity = WorkoutSessionUseCase::add(&state.database, domain).await;

    if entity.is_err() {
        return Err(ExceptionResponse::BadRequest(locale,ErrorKey::WorkoutAddFailed));
    }

    let done_workout = entity.unwrap();
    let payload = WorkoutMapper::json(done_workout);
    Ok((StatusCode::CREATED, Json(payload)))
}

#[utoipa::path(
    get,
    tag = "timeline",
    path = "/timeline/api/workout-sessions/{workout_session_id}",
    params(
        ("workout_session_id" = String, Path, description = "The id of the workout session"),
    ),
    responses(
        (status = 200, description = "Workout session found", body = WorkoutSessionJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 404, description = "Not found", body = NotFoundErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_workout_session(state: State<AppState>,Extension(locale): Extension<Locale>,Path(workout_session_id): Path<String>) -> HttpResponse<Json<WorkoutSessionJson>> {
    let response = WorkoutSessionUseCase::find_by_id(&state.database, workout_session_id.clone()).await;
    match response {
        Some(workout) => {
            let payload = WorkoutMapper::json(workout);
            Ok(Json(payload))
        }
        None => {
            Err(ExceptionResponse::not_found(locale, ErrorKey::WorkoutNotFound))
        }
    }

}

#[utoipa::path(
    get,
    tag = "timeline",
    path = "/timeline/api/workout-sessions",
    params(
        ("startDate" = Option<String>, Query, description = "Start date ISO 8601 (e.g. 2026-03-25T00:00:00). Defaults to 7 days ago."),
        ("endDate" = Option<String>, Query, description = "End date ISO 8601 (e.g. 2026-04-01T23:59:59). Defaults to now."),
    ),
    responses(
        (status = 200, description = "A list of workout sessions", body = [WorkoutSessionJson]),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_workout_sessions(state: State<AppState>,Query(params): Query<WorkoutFilterParams>,Extension(current_user): Extension<User>) -> HttpResponse<Json<Vec<WorkoutSessionJson>>> {
    let person_uuid = current_user.person_uuid.clone();

    let end = params.end_date.unwrap_or_else(|| Utc::now().naive_utc());
    let start = params.start_date.unwrap_or_else(|| end - Duration::days(7));

    let start_bson = opt_naive_to_bson_datetime(start).unwrap();
    let end_bson = opt_naive_to_bson_datetime(end).unwrap();

    let response = WorkoutSessionUseCase::find_all_by_person(&state.database, person_uuid, start_bson, end_bson).await;

    Ok(Json(WorkoutMapper::json_vec(response)))
}
