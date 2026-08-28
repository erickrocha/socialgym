use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::evolution_check_in_json::EvolutionCheckInJson;
use crate::infrastructure::data_tools::opt_naive_to_bson_datetime;
use crate::infrastructure::mapper::{EvolutionCheckInMapper, Mapper};
use axum::extract::{Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use business::use_cases::evolution_check_in_use_case::EvolutionCheckInUseCase;
use chrono::{Duration, Utc};
use domain::user::User;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionCheckInFilterParams {
    pub start_date: Option<chrono::NaiveDateTime>,
    pub end_date: Option<chrono::NaiveDateTime>,
}

#[utoipa::path(
    post,
    tag = "timeline",
    path = "/timeline/api/evolution-checkin",
    request_body = EvolutionCheckInJson,
    responses(
        (status = 201, description = "Evolution check-in saved", body = EvolutionCheckInJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn add(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<EvolutionCheckInJson>,
) -> HttpResponse<(StatusCode, Json<EvolutionCheckInJson>)> {
    let domain = EvolutionCheckInMapper::domain(payload);

    let evolution_check_in_use_case =
        EvolutionCheckInUseCase::new(EvolutionCheckInGateway::new(&state.database.clone()));

    let evolution_check_in = evolution_check_in_use_case
        .add(domain, &current_user.person_uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::EvolutionCheckInAddFailed)
        })?;

    let payload = EvolutionCheckInMapper::json(evolution_check_in);
    Ok((StatusCode::CREATED, Json(payload)))
}

#[utoipa::path(
    get,
    tag = "timeline",
    path = "/timeline/api/evolution-checkin",
    params(
        ("startDate" = Option<String>, Query, description = "Start date ISO 8601 (e.g. 2026-03-25T00:00:00). Defaults to 7 days ago."),
        ("endDate" = Option<String>, Query, description = "End date ISO 8601 (e.g. 2026-04-01T23:59:59). Defaults to now."),
    ),
    responses(
        (status = 200, description = "A list of workout sessions", body = [EvolutionCheckInJson]),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("api_key" = [])
    )
)]
pub async fn get_by_owner(
    state: State<AppState>,
    Query(params): Query<EvolutionCheckInFilterParams>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<Vec<EvolutionCheckInJson>>> {
    let person_uuid = current_user.person_uuid.clone();
    let end = params.end_date.unwrap_or_else(|| Utc::now().naive_utc());
    let start = params.start_date.unwrap_or_else(|| end - Duration::days(7));

    let start_bson = opt_naive_to_bson_datetime(start).unwrap();
    let end_bson = opt_naive_to_bson_datetime(end).unwrap();

    let evolution_check_in_use_case =
        EvolutionCheckInUseCase::new(EvolutionCheckInGateway::new(&state.database.clone()));

    let response = evolution_check_in_use_case
        .find_all_by_owner(person_uuid, start_bson, end_bson)
        .await;
    Ok(Json(EvolutionCheckInMapper::json_vec(response)))
}
