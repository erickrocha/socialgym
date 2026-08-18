use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::http::json::exercise_json::{ExerciseJson, PaginatedExerciseJson};
use crate::infrastructure::mapper::{ExerciseMapper, Mapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::user::User;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use serde::Deserialize;
use utoipa::ToSchema;
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson};

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseParams {
    pub category: Option<String>,
    pub visibility: Option<String>,
    pub owners: Option<Vec<i32>>,
    pub page_number: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
}

// Add new handler function
#[utoipa::path(
    post,
    path = "/workout/api/exercises",
    request_body = ExerciseJson,
    responses(
        (status = 201, description = "Exercise created successfully", body = ExerciseJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_exercise(state: State<AppState>,Extension(locale): Extension<Locale>,Json(payload): Json<ExerciseJson>) -> HttpResponse<(StatusCode, Json<ExerciseJson>)> {
    let exercise = ExerciseMapper::domain(payload);

    // Add exercise via use case
    let result = ExerciseUseCase::persist(&state.conn, exercise).await;

    if result.is_none() {
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::ExercisesNotAdded));
    }

    let created_exercise = result.unwrap();
    let response = ExerciseMapper::json(created_exercise);

    Ok((StatusCode::CREATED,Json(response)))
}

#[utoipa::path(
    delete,
    path = "/workout/api/exercises/{exercise_id}",
    params(
        ("exercise_id" = i32, Path, description = "Exercise id")
    ),
    responses(
        (status = 204, description = "Exercise deleted successfully"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn delete_exercise(state: State<AppState>,Path(exercise_id): Path<i32>,Extension(locale): Extension<Locale>) -> HttpResponse<StatusCode> {
    let result = ExerciseUseCase::delete_by_id(&state.conn, exercise_id).await;
    if result.is_err() {
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::ExercisesNotAdded));
    }
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    post,
    path = "/workout/api/exercises/query",
    request_body = ExerciseParams,
    responses(
        (status = 200, description = "Paginated list of exercises with complex filtering", body = PaginatedExerciseJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn query_exercises(state: State<AppState>,Extension(current_user): Extension<User>,Extension(locale): Extension<Locale>,Json(payload): Json<ExerciseParams>) -> HttpResponse<Json<PaginatedExerciseJson>> {
    let person_id = current_user.person_id;

    // Validate and set defaults
    let page_number = payload.page_number.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(20);

    // Validate page_number
    if page_number < 1 {
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::InvalidParameterValue));
    }

    // Validate and cap page_size
    if page_size < 1 {
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::InvalidParameterValue));
    }

    let capped_page_size = if page_size > 100 { 100 } else { page_size };

    // Set defaults for category
    let category = payload.category.or_else(|| Some("Force".to_string()));

    // Get public_owner_ids, default to empty if not provided
    let public_owner_ids = payload.owners.unwrap_or_default();

    log::info!(
        "Querying paginated exercises with complex filters: category={:?}, public_owner_ids={:?}, page_number={}, page_size={}, sort_by={:?} for person_id={}",
        category,
        public_owner_ids,
        page_number,
        capped_page_size,
        payload.sort_by,
        person_id
    );

    let result = ExerciseUseCase::find_by_complex_filters_paginated(
        &state.conn,
        person_id,
        public_owner_ids,
        category,
        payload.visibility,
        page_number,
        capped_page_size,
        payload.sort_by,
    )
    .await;

    let (exercises, total_count, has_next_page) = result;

    let exercises_json: Vec<ExerciseJson> = ExerciseMapper::json_vec(exercises);

    let response = PaginatedExerciseJson {
        content: exercises_json,
        total_count,
        page_number,
        page_size: capped_page_size,
        has_next_page,
    };

    Ok(Json(response))
}
