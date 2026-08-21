use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::person_json::PersonJson;
use crate::infrastructure::mapper::{Mapper, PersonInfoMapper, PersonMapper};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::{Extension, Json};
use business::commons::functions::parse_uuid;
use business::domain::person::Person;
use business::domain::user::User;
use business::use_cases::person_use_case::PersonUseCase;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct SearchPersonsQuery {
    pub query: String,
    pub limit: Option<i32>,
}

pub async fn get_person_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<PersonJson>> {
    let person = PersonUseCase::get(&state.conn, id).await.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::PersonNotFound)
    })?;
    Ok(Json(PersonMapper::json(person)))
}

pub async fn get_person_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<PersonJson>> {
    let person = PersonUseCase::find_by_uuid(&state.conn, uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::PersonNotFound)
        })?;
    Ok(Json(PersonMapper::json(person)))
}

pub async fn search_mentionable_friends(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Query(params): Query<SearchPersonsQuery>,
) -> HttpResponse<Json<Vec<PersonJson>>> {
    let people = PersonUseCase::search_mentionable_friends(
        &state.conn,
        person_id,
        &params.query,
        params.limit.unwrap_or(10),
    )
    .await;
    Ok(Json(PersonMapper::json_vec(people)))
}

#[utoipa::path(
    put,
    path = "/workout/api/people/me",
    request_body = PersonJson,
    responses(
        (status = 200, description = "Current user profile updated", body = PersonJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_person(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<PersonJson>,
) -> HttpResponse<Json<PersonJson>> {
    // `/people/me`: the person updated is always the caller, never `payload.id`.
    let person_id = current_user.person_id;

    let person_result = PersonUseCase::get(&state.conn, person_id).await;
    if person_result.is_err() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::PersonNotUpdated,
        ));
    };

    let person = person_result.unwrap();

    let firstname = match payload.firstname {
        Some(firstname) => firstname,
        None => person.firstname,
    };

    let surname = match payload.surname {
        Some(surname) => surname,
        None => person.surname,
    };

    let date_of_birt = match payload.date_of_birth {
        Some(date_of_birth) => date_of_birth,
        None => person.date_of_birth,
    };

    let gender = match payload.gender {
        Some(gender) => gender,
        None => person.gender,
    };

    let avatar = match payload.avatar {
        Some(image_url) => image_url,
        None => person.avatar.unwrap_or_default(),
    };

    let cover = match payload.cover {
        Some(image_url) => image_url,
        None => person.cover.unwrap_or_default(),
    };

    let person_info = match payload.person_info {
        Some(json) => Some(PersonInfoMapper::domain(json)),
        None => person.person_info,
    };

    let person = Person::update(
        Some(person_id),
        firstname,
        surname,
        date_of_birt,
        gender,
        Some(avatar),
        Some(cover),
        None,
        person_info,
        Vec::new(),
        Vec::new(),
    );

    let person_response = PersonUseCase::update(&state.conn, person).await;

    match person_response {
        Ok(person) => Ok(Json(PersonMapper::json(person))),
        Err(_) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PersonNotUpdated,
        )),
    }
}

#[utoipa::path(
    get,
    path = "/workout/api/people/me",
    responses(
        (status = 200, description = "Current user profile", body = PersonJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_me(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<PersonJson>> {
    let person_id = current_user.person_id;

    let person_entity = PersonUseCase::get(&state.conn, person_id).await;

    if person_entity.is_err() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::PersonNotFound,
        ));
    }

    Ok(Json(PersonMapper::json(person_entity.unwrap())))
}

#[utoipa::path(
    get,
    path = "/workout/api/people/me/friend/{friend_id}",
    params(
        ("friend_id" = i32, Path, description = "Friend person id")
    ),
    responses(
        (status = 200, description = "Friend profile", body = PersonJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_my_friend(
    state: State<AppState>,
    Path(friend_id): Path<i32>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<PersonJson>> {
    let person_entity = PersonUseCase::get(&state.conn, friend_id).await;

    if person_entity.is_err() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::PersonNotFound,
        ));
    }

    Ok(Json(PersonMapper::json(person_entity.unwrap())))
}

#[utoipa::path(
    get,
    path = "/workout/api/people/search",
    params(
        ("query" = String, Query, description = "Search query for name or email"),
        ("limit" = Option<i32>, Query, description = "Maximum number of results (default: 50, max: 100)"),
    ),
    responses(
        (status = 200, description = "Search results", body = Vec<PersonJson>),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn search_persons(
    state: State<AppState>,
    Query(params): Query<SearchPersonsQuery>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<Vec<PersonJson>>> {
    let current_user_person_id = current_user.person_id;

    let query = params.query.clone();
    let limit = params.limit.unwrap_or(50);

    // Cap the limit at 100
    let limit = if limit > 100 {
        100
    } else if limit < 1 {
        50
    } else {
        limit
    };

    // Call the use case
    let persons =
        PersonUseCase::search_persons(&state.conn, &query, current_user_person_id, limit).await;
    Ok(Json(PersonMapper::json_vec(persons)))
}

#[utoipa::path(
    get,
    path = "/workout/api/people/me/{uuid}",
    params(
        ("uuid" = String, Path, description = "Person UUID")
    ),
    responses(
        (status = 200, description = "Search results", body = Vec<PersonJson>),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_me_by_uuid(
    state: State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<PersonJson>> {
    if parse_uuid(&uuid).is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::InvalidParameterValue,
        ));
    }
    let person_entity = PersonUseCase::find_by_uuid(&state.conn, uuid).await;

    if person_entity.is_err() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::PersonNotFound,
        ));
    }

    Ok(Json(PersonMapper::json(person_entity.unwrap())))
}
