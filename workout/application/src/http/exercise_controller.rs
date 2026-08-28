use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::exercise_json::{ExerciseJson, PaginatedExerciseJson};
use crate::infrastructure::mapper::{ExerciseMapper, Mapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::user::User;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct ExerciseParams {
    pub owner_uuid: Option<String>,
    pub category: Option<String>,
    pub visibility: Option<String>,
    pub owners: Option<Vec<i32>>,
    pub owner_uuids: Option<Vec<String>>,
    pub page_number: Option<u64>,
    pub page_size: Option<u64>,
    pub sort_by: Option<String>,
}

fn exercise_error(
    error: business::domain::business_error::BusinessError,
    locale: Locale,
) -> ExceptionResponse {
    ExceptionResponse::from_business(error, locale, ErrorKey::InvalidParameterValue)
}

#[utoipa::path(get, path = "/workout/api/exercises/id/{id}")]
pub async fn get_exercise_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<ExerciseJson>> {
    let exercise = ExerciseUseCase::get(&state.conn, id)
        .await
        .map_err(|error| exercise_error(error, locale))?;
    ExerciseUseCase::ensure_readable(&exercise, current_user.person_id)
        .map_err(|error| exercise_error(error, locale))?;
    Ok(Json(ExerciseMapper::json(exercise)))
}

#[utoipa::path(get, path = "/workout/api/exercises/uuid/{uuid}")]
pub async fn get_exercise_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<ExerciseJson>> {
    let exercise = ExerciseUseCase::get_by_uuid(&state.conn, uuid)
        .await
        .map_err(|error| exercise_error(error, locale))?;
    ExerciseUseCase::ensure_readable(&exercise, current_user.person_id)
        .map_err(|error| exercise_error(error, locale))?;
    Ok(Json(ExerciseMapper::json(exercise)))
}

#[utoipa::path(put, path = "/workout/api/exercises")]
pub async fn update_exercise(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<ExerciseJson>,
) -> HttpResponse<Json<ExerciseJson>> {
    let exercise = ExerciseUseCase::persist(
        &state.conn,
        ExerciseMapper::domain(payload),
        &current_user,
        active_profile.as_deref(),
    )
        .await
        .map_err(|error| exercise_error(error, locale))?;
    Ok(Json(ExerciseMapper::json(exercise)))
}

#[utoipa::path(delete, path = "/workout/api/exercises/uuid/{uuid}")]
pub async fn delete_exercise_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    ExerciseUseCase::delete_by_uuid(&state.conn, uuid, current_user.person_id)
        .await
        .map_err(|error| exercise_error(error, locale))?;
    Ok(StatusCode::NO_CONTENT)
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
pub async fn add_exercise(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<ExerciseJson>,
) -> HttpResponse<(StatusCode, Json<ExerciseJson>)> {
    let exercise = ExerciseMapper::domain(payload);

    // Add exercise via use case
    let result =
        ExerciseUseCase::persist(&state.conn, exercise, &current_user, active_profile.as_deref())
            .await;

    let created_exercise = result.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::ExercisesNotAdded)
    })?;
    let response = ExerciseMapper::json(created_exercise);

    Ok((StatusCode::CREATED, Json(response)))
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
pub async fn delete_exercise(
    state: State<AppState>,
    Path(exercise_id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    ExerciseUseCase::delete_by_id(&state.conn, exercise_id, current_user.person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::ExercisesNotAdded)
        })?;
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
pub async fn query_exercises(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<ExerciseParams>,
) -> HttpResponse<Json<PaginatedExerciseJson>> {
    // `ownerUuid` only selects the uuid-keyed query shape; the identity it runs
    // as is always the caller's acting identity (their active business
    // profile, if any, else themself), never the one in the body.
    if payload.owner_uuid.is_some() {
        let owner_uuid = active_profile
            .as_deref()
            .and_then(|p| p.uuid.clone())
            .unwrap_or_else(|| current_user.person_uuid.clone());
        let page_number = payload.page_number.unwrap_or(1);
        let page_size = payload.page_size.unwrap_or(20).min(100);
        if page_number == 0 || page_size == 0 {
            return Err(ExceptionResponse::BadRequest(
                locale,
                ErrorKey::InvalidParameterValue,
            ));
        }
        let result = ExerciseUseCase::find_by_complex_filters_paginated_uuid(
            &state.conn,
            owner_uuid,
            payload.owner_uuids.unwrap_or_default(),
            payload.category,
            payload.visibility,
            page_number,
            page_size,
            payload.sort_by,
        )
        .await;
        return Ok(Json(PaginatedExerciseJson {
            content: ExerciseMapper::json_vec(result.0),
            total_count: result.1,
            page_number,
            page_size,
            has_next_page: result.2,
        }));
    }
    let person_id = current_user.person_id;

    // Validate and set defaults
    let page_number = payload.page_number.unwrap_or(1);
    let page_size = payload.page_size.unwrap_or(20);

    // Validate page_number
    if page_number < 1 {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::InvalidParameterValue,
        ));
    }

    // Validate and cap page_size
    if page_size < 1 {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::InvalidParameterValue,
        ));
    }

    let capped_page_size = if page_size > 100 { 100 } else { page_size };

    // Set defaults for category
    let category = payload.category.or_else(|| Some("Force".to_string()));

    // Get public_owner_ids, default to empty if not provided
    let public_owner_ids = payload.owners.unwrap_or_default();

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

    let (exercises, total_count, has_next_page) = result.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::InvalidParameterValue)
    })?;

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
