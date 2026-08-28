use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::settings_json::SettingsJson;
use crate::infrastructure::mapper::{Mapper, SettingsMapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use business::commons::authorization::ensure_owns;
use business::commons::functions::parse_uuid;
use business::domain::user::User;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::setings_use_case::SettingsUseCase;

#[utoipa::path(
    get,
    path = "/workout/api/settings/{id}",
    params(
        ("id" = i32, Path, description = "Settings ID")
    ),
    responses(
        (status = 200, description = "Settings found", body = SettingsJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 404, description = "Settings not found", body = InternalServerErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_settings_by_id(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Path(id): Path<i32>,
) -> HttpResponse<Json<SettingsJson>> {
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));

    let result = use_case.get_by_id(id).await;
    match result {
        Ok(settings) => {
            ensure_owns(settings.person_id, current_user.person_id)
                .map_err(|error| ExceptionResponse::from_business(error, locale, ErrorKey::SettingsNotFound))?;
            let json = SettingsMapper::json(settings);
            Ok(Json(json))
        }
        Err(_e) => Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::ResourcesNotFound,
        )),
    }
}

#[utoipa::path(
    get,
    path = "/workout/api/settings/uuid/{uuid}",
    params(
        ("uuid" = String, Path, description = "Settings UUID")
    ),
    responses(
        (status = 200, description = "Settings found", body = SettingsJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 404, description = "Settings not found", body = InternalServerErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_settings_by_uuid(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Path(uuid): Path<String>,
) -> HttpResponse<Json<SettingsJson>> {
    if parse_uuid(&uuid).is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::InvalidParameterValue,
        ));
    }
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));

    let result = use_case.get_by_uuid(uuid.clone()).await;
    match result {
        Ok(settings) => {
            ensure_owns(settings.person_id, current_user.person_id)
                .map_err(|error| ExceptionResponse::from_business(error, locale, ErrorKey::SettingsNotFound))?;
            let json = SettingsMapper::json(settings);
            Ok(Json(json))
        }
        Err(_e) => Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::ResourcesNotFound,
        )),
    }
}

#[utoipa::path(
    get,
    path = "/workout/api/settings/me",
    responses(
        (status = 200, description = "Current user's settings", body = SettingsJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 404, description = "Settings not found", body = InternalServerErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_my_settings(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<SettingsJson>> {
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));

    let result = use_case.get_by_owner_id(current_user.person_id).await;
    match result {
        Ok(settings) => {
            let json = SettingsMapper::json(settings);
            Ok(Json(json))
        }
        Err(_e) => Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::SettingsNotFound,
        )),
    }
}

#[utoipa::path(
    put,
    path = "/workout/api/settings/me",
    request_body = SettingsJson,
    responses(
        (status = 200, description = "Settings updated successfully", body = SettingsJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 404, description = "Settings not found", body = InternalServerErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn update_my_settings(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<SettingsJson>,
) -> HttpResponse<Json<SettingsJson>> {
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));

    let settings = SettingsMapper::domain(payload);

    let result = use_case.persist(settings, &current_user).await;
    match result {
        Ok(saved_settings) => {
            let json = SettingsMapper::json(saved_settings);
            Ok(Json(json))
        }
        Err(_e) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::SettingsUpdatedFailed,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/workout/api/settings",
    request_body = SettingsJson,
    responses(
        (status = 201, description = "Settings created successfully", body = SettingsJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn create_settings(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<SettingsJson>,
) -> HttpResponse<Json<SettingsJson>> {
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));

    let settings = SettingsMapper::domain(payload);

    let result = use_case.persist(settings, &current_user).await;
    match result {
        Ok(saved_settings) => {
            let json = SettingsMapper::json(saved_settings);
            Ok(Json(json))
        }
        Err(_e) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::SettingsAddedFailed,
        )),
    }
}

pub async fn get_settings_by_owner_id(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Path(id): Path<i32>,
) -> HttpResponse<Json<SettingsJson>> {
    ensure_owns(id, current_user.person_id).map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::SettingsNotFound)
    })?;
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));
    let settings = use_case.get_by_owner_id(id).await.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::SettingsNotFound)
    })?;
    Ok(Json(SettingsMapper::json(settings)))
}

pub async fn get_settings_by_owner_uuid(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Path(uuid): Path<String>,
) -> HttpResponse<Json<SettingsJson>> {
    if uuid != current_user.person_uuid {
        return Err(ExceptionResponse::Forbidden(locale, ErrorKey::SettingsNotFound));
    }
    let use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));
    let settings = use_case.get_by_owner_uuid(uuid).await.map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::SettingsNotFound)
    })?;
    Ok(Json(SettingsMapper::json(settings)))
}
