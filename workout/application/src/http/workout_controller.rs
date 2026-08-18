use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson};
use crate::http::json::exercise_json::ExerciseJson;
use crate::http::json::workout_json::WorkoutJson;
use crate::infrastructure::mapper::{ExerciseMapper, Mapper, WorkoutMapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::exercise::Exercise;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;

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
pub async fn add_workout(state: State<AppState>,Extension(locale): Extension<Locale>,Json(payload): Json<WorkoutJson>) -> HttpResponse<(StatusCode, Json<WorkoutJson>)> {
    log::info!("Adding workout {:?}", payload);

    let domain = WorkoutMapper::domain(payload);

    let entity = WorkoutUseCase::persist(&state.conn, domain).await;

    if entity.is_err() {
        let error = entity.err().unwrap();
        log::error!("Error adding workout: {}", error.message);
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::WorkoutAddFailed));
    }

    let workout = entity.unwrap();
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
pub async fn get_workouts(state: State<AppState>,Path(person_id): Path<i32>) -> HttpResponse<Json<Vec<WorkoutJson>>> {
    log::info!("Fetching workouts for person_id={}", person_id);
    let result = WorkoutUseCase::find_all_by_owner_id(&state.conn, person_id).await;
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
pub async fn add_exercises(state: State<AppState>,Path(workout_id): Path<i32>,Extension(locale): Extension<Locale>,Json(exercises): Json<Vec<ExerciseJson>>) -> HttpResponse<(StatusCode, Json<Vec<ExerciseJson>>)> {
    log::info!("Adding exercises for workout_id={}", workout_id);

    let workout_result = WorkoutUseCase::get(&state.conn, workout_id).await;
    
    if workout_result.is_none() {
        log::error!("Workout not found for workout_id={}", workout_id);
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::WorkoutNotFound));
    }

    let domain_exercises: Vec<Exercise> = ExerciseMapper::domain_vec(exercises);
    let result = ExerciseUseCase::add_all_to_workout(&state.conn, workout_id, domain_exercises).await;
    if result.is_err() {
        log::error!("Error adding exercises: {:?}", result.err().unwrap());
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::ExercisesNotAdded));
    }
    let added_exercises = result.unwrap();
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
pub async fn get_exercises(state: State<AppState>,Path(workout_id): Path<i32>,Extension(locale): Extension<Locale>) -> HttpResponse<Json<Vec<ExerciseJson>>> {
    log::info!("Fetching exercises for workout_id={}", workout_id);
    let result = ExerciseUseCase::find_all_by_workout_id(&state.conn, workout_id).await;
    if result.is_err() {
        log::error!("Error fetching exercises: {}", result.err().unwrap());
        return Err(ExceptionResponse::BadRequest(locale,ErrorKey::ExercisesFetchFailed));
    }
    Ok(Json(ExerciseMapper::json_vec(result.unwrap()))) // Placeholder response
}
