use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::exercise_json::ExerciseJson;
use crate::http::json::workout_json::WorkoutJson;
use crate::infrastructure::mapper::{ExerciseMapper, Mapper, WorkoutMapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::exercise::Exercise;
use business::domain::user::User;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;

fn workout_error(
    error: business::domain::business_error::BusinessError,
    locale: Locale,
) -> ExceptionResponse {
    ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutNotFound)
}

#[utoipa::path(get, path = "/workout/api/workouts/id/{id}")]
pub async fn get_workout_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<WorkoutJson>> {
    let workout = WorkoutUseCase::get(&state.conn, id)
        .await
        .map_err(|error| workout_error(error, locale))?;
    WorkoutUseCase::ensure_readable(&workout, current_user.person_id)
        .map_err(|error| workout_error(error, locale))?;
    Ok(Json(WorkoutMapper::json(workout)))
}

#[utoipa::path(get, path = "/workout/api/workouts/uuid/{uuid}")]
pub async fn get_workout_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<WorkoutJson>> {
    let workout = WorkoutUseCase::get_by_uuid(&state.conn, uuid)
        .await
        .map_err(|error| workout_error(error, locale))?;
    WorkoutUseCase::ensure_readable(&workout, current_user.person_id)
        .map_err(|error| workout_error(error, locale))?;
    Ok(Json(WorkoutMapper::json(workout)))
}

#[utoipa::path(get, path = "/workout/api/workouts/owner/uuid/{uuid}")]
pub async fn get_workouts_by_owner_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<WorkoutJson>>> {
    let workouts = WorkoutUseCase::find_all_by_owner_uuid(&state.conn, uuid)
        .await
        .map_err(|error| workout_error(error, locale))?;
    Ok(Json(WorkoutMapper::json_vec(workouts)))
}

/// Workouts the acting business profile has assigned to its team members
/// (every status). Requires an active business profile.
#[utoipa::path(get, path = "/workout/api/workouts/assigned-by-profile")]
pub async fn get_workouts_assigned_by_profile(
    State(state): State<AppState>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<WorkoutJson>>> {
    let profile_id = active_profile
        .as_deref()
        .and_then(|p| p.id)
        .ok_or(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::WorkoutNotFound,
        ))?;
    let workouts = WorkoutUseCase::find_all_assigned_by_profile(&state.conn, profile_id)
        .await
        .map_err(|error| workout_error(error, locale))?;
    Ok(Json(WorkoutMapper::json_vec(workouts)))
}

#[utoipa::path(put, path = "/workout/api/workouts")]
pub async fn update_workout(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<WorkoutJson>,
) -> HttpResponse<Json<WorkoutJson>> {
    let workout = WorkoutUseCase::persist(
        &state.conn,
        WorkoutMapper::domain(payload),
        &current_user,
        active_profile.as_deref(),
        None,
    )
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutAddFailed)
        })?;
    Ok(Json(WorkoutMapper::json(workout)))
}

#[utoipa::path(delete, path = "/workout/api/workouts/id/{id}")]
pub async fn delete_workout_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    WorkoutUseCase::delete_by_id(&state.conn, id, current_user.person_id)
        .await
        .map_err(|error| workout_error(error, locale))?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(delete, path = "/workout/api/workouts/uuid/{uuid}")]
pub async fn delete_workout_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    WorkoutUseCase::delete_by_uuid(&state.conn, uuid, current_user.person_id)
        .await
        .map_err(|error| workout_error(error, locale))?;
    Ok(StatusCode::NO_CONTENT)
}

/// The assigned person accepts a pending workout assignment.
#[utoipa::path(put, path = "/workout/api/workouts/uuid/{uuid}/accept")]
pub async fn accept_workout_assignment(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<WorkoutJson>> {
    let workout = WorkoutUseCase::accept_assignment(&state.conn, uuid, current_user.person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutAcceptAssignmentFailed)
        })?;
    Ok(Json(WorkoutMapper::json(workout)))
}

/// The assigned person rejects a pending workout assignment.
#[utoipa::path(put, path = "/workout/api/workouts/uuid/{uuid}/reject")]
pub async fn reject_workout_assignment(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<WorkoutJson>> {
    let workout = WorkoutUseCase::reject_assignment(&state.conn, uuid, current_user.person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutRejectAssignmentFailed)
        })?;
    Ok(Json(WorkoutMapper::json(workout)))
}

/// The assigning business profile cancels a pending workout assignment.
/// Requires an active business profile.
#[utoipa::path(put, path = "/workout/api/workouts/uuid/{uuid}/cancel")]
pub async fn cancel_workout_assignment(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(_current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<WorkoutJson>> {
    let profile_id = active_profile
        .as_deref()
        .and_then(|p| p.id)
        .ok_or(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::WorkoutCancelAssignmentFailed,
        ))?;
    let workout = WorkoutUseCase::cancel_assignment(&state.conn, uuid, profile_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutCancelAssignmentFailed)
        })?;
    Ok(Json(WorkoutMapper::json(workout)))
}

#[utoipa::path(post, path = "/workout/api/workouts/uuid/{uuid}/exercises")]
pub async fn add_exercises_by_workout_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(exercises): Json<Vec<ExerciseJson>>,
) -> HttpResponse<Json<WorkoutJson>> {
    let mut workout = WorkoutUseCase::get_by_uuid(&state.conn, uuid)
        .await
        .map_err(|error| workout_error(error, locale))?;
    let acting_owner_id = active_profile
        .as_deref()
        .and_then(|p| p.id)
        .unwrap_or(current_user.person_id);
    business::commons::authorization::ensure_owns(workout.owner_id, acting_owner_id)
        .map_err(|error| workout_error(error, locale))?;
    workout.exercises = ExerciseUseCase::add_all_to_workout(
        &state.conn,
        workout.id.unwrap(),
        ExerciseMapper::domain_vec(exercises),
        &current_user,
        active_profile.as_deref(),
    )
    .await
    .map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::ExercisesNotAdded)
    })?;
    Ok(Json(WorkoutMapper::json(workout)))
}

#[utoipa::path(
    post,
    path = "/workout/api/workouts",
    request_body = WorkoutJson,
    responses(
        (status = 200, description = "Workout added successfully", body = WorkoutJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),

    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_workout(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<WorkoutJson>,
) -> HttpResponse<(StatusCode, Json<WorkoutJson>)> {
    let assign_to_person_uuid = payload.target_person_uuid.clone();
    let domain = WorkoutMapper::domain(payload);

    let workout = WorkoutUseCase::persist(
        &state.conn,
        domain,
        &current_user,
        active_profile.as_deref(),
        assign_to_person_uuid.as_deref(),
    )
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutAddFailed)
        })?;
    let payload = WorkoutMapper::json(workout);
    Ok((StatusCode::CREATED, Json(payload)))
}

#[utoipa::path(
    get,
    path = "/workout/api/workouts/{owner_id}",
    params(
        ("owner_id" = i32, Path, description = "Owner id")
    ),
    responses(
        (status = 200, description = "List of workouts", body = Vec<WorkoutJson>),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_workouts(
    state: State<AppState>,
    Path(person_id): Path<i32>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<WorkoutJson>>> {
    let result = WorkoutUseCase::find_all_by_owner_id(&state.conn, person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutNotFound)
        })?;
    let response: Vec<WorkoutJson> = WorkoutMapper::json_vec(result);
    Ok(Json(response))
}

#[utoipa::path(
    post,
    path = "/workout/api/workouts/{workout_id}/exercises",
    request_body = Vec<ExerciseJson>,
    params(
        ("workout_id" = i32, Path, description = "Workout id")
    ),
    responses(
        (status = 200, description = "Exercises linked to workout", body = Vec<ExerciseJson>),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn add_exercises(
    state: State<AppState>,
    Path(workout_id): Path<i32>,
    Extension(current_user): Extension<User>,
    active_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
    Json(exercises): Json<Vec<ExerciseJson>>,
) -> HttpResponse<(StatusCode, Json<Vec<ExerciseJson>>)> {
    let workout = WorkoutUseCase::get(&state.conn, workout_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::WorkoutNotFound)
        })?;
    let acting_owner_id = active_profile
        .as_deref()
        .and_then(|p| p.id)
        .unwrap_or(current_user.person_id);
    business::commons::authorization::ensure_owns(workout.owner_id, acting_owner_id)
        .map_err(|error| workout_error(error, locale))?;

    let domain_exercises: Vec<Exercise> = ExerciseMapper::domain_vec(exercises);
    let result = ExerciseUseCase::add_all_to_workout(
        &state.conn,
        workout_id,
        domain_exercises,
        &current_user,
        active_profile.as_deref(),
    )
    .await;
    let added_exercises = result.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::ExercisesNotAdded)
    })?;
    let payload: Vec<ExerciseJson> = ExerciseMapper::json_vec(added_exercises);

    Ok((StatusCode::CREATED, Json(payload))) // Placeholder response
}

#[utoipa::path(
    get,
    path = "/workout/api/workouts/{workout_id}/exercises",
    params(
        ("workout_id" = i32, Path, description = "Workout id")
    ),
    responses(
        (status = 200, description = "Exercises for the workout", body = Vec<ExerciseJson>),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_exercises(
    state: State<AppState>,
    Path(workout_id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<ExerciseJson>>> {
    let workout = WorkoutUseCase::get(&state.conn, workout_id)
        .await
        .map_err(|error| workout_error(error, locale))?;
    WorkoutUseCase::ensure_readable(&workout, current_user.person_id)
        .map_err(|error| workout_error(error, locale))?;

    let result = ExerciseUseCase::find_all_by_workout_id(&state.conn, workout_id).await;
    let exercises = result.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::ExercisesFetchFailed)
    })?;
    Ok(Json(ExerciseMapper::json_vec(exercises)))
}
