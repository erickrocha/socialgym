use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::business_profile_address_json::BusinessProfileAddressJson;
use crate::http::json::business_profile_json::BusinessProfileJson;
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::infrastructure::mapper::{BusinessProfileAddressMapper, BusinessProfileMapper, Mapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::user::User;
use business::use_cases::business_profile_address_use_case::BusinessProfileAddressUseCase;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;

pub async fn get_profile_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    BusinessProfileUseCase::get_by_id(&state.conn, id)
        .await
        .map(BusinessProfileMapper::json)
        .map(Json)
        .ok_or(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        ))
}

pub async fn get_profile_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    BusinessProfileUseCase::get_by_uuid(&state.conn, uuid)
        .await
        .map(BusinessProfileMapper::json)
        .map(Json)
        .ok_or(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        ))
}

pub async fn get_profiles_by_owner_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<BusinessProfileJson>>> {
    let profiles = BusinessProfileUseCase::get_by_owner_id(&state.conn, id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    Ok(Json(BusinessProfileMapper::json_vec(profiles)))
}

pub async fn get_profiles_by_owner_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<BusinessProfileJson>>> {
    let profiles = BusinessProfileUseCase::get_by_owner_uuid(&state.conn, uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    Ok(Json(BusinessProfileMapper::json_vec(profiles)))
}

pub async fn add_profile(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<BusinessProfileJson>,
) -> HttpResponse<(StatusCode, Json<BusinessProfileJson>)> {
    let profile = BusinessProfileUseCase::add(
        &state.conn,
        BusinessProfileMapper::domain(payload),
        &current_user,
    )
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    Ok((
        StatusCode::CREATED,
        Json(BusinessProfileMapper::json(profile)),
    ))
}

pub async fn update_profile(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<BusinessProfileJson>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    let profile =
        BusinessProfileUseCase::update(&state.conn, BusinessProfileMapper::domain(payload), &current_user)
            .await
            .map_err(|error| {
                ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
            })?;
    Ok(Json(BusinessProfileMapper::json(profile)))
}

pub async fn save_address(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<BusinessProfileAddressJson>,
) -> HttpResponse<Json<BusinessProfileAddressJson>> {
    let address = BusinessProfileAddressUseCase::save(
        &state.conn,
        BusinessProfileAddressMapper::domain(payload),
        current_user.person_id,
    )
    .await
    .map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
    })?;
    Ok(Json(BusinessProfileAddressMapper::json(address)))
}

pub async fn delete_address_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    BusinessProfileAddressUseCase::delete_by_id(&state.conn, id, current_user.person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_address_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    BusinessProfileAddressUseCase::delete_by_uuid(&state.conn, uuid, current_user.person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    get,
    path = "/workout/api/business-profiles",
    responses(
        (status = 200, description = "Authenticated user's business profiles", body = Vec<BusinessProfileJson>),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_by_owner_id(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<BusinessProfileJson>>> {
    let owner_id = current_user.person_id;

    let result = BusinessProfileUseCase::get_by_owner_id(&state.conn, owner_id).await;

    if result.is_err() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        ));
    }

    let business_profiles = result.unwrap();
    let response: Vec<BusinessProfileJson> = business_profiles
        .into_iter()
        .map(BusinessProfileMapper::json)
        .collect();

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/workout/api/business-profiles/active",
    params(
        ("business_profile_id" = i32, Path, description = "Business profile id")
    ),
    responses(
        (status = 200, description = "Business profile with addresses", body = BusinessProfileJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_active(
    state: State<AppState>,
    Extension(active_business_profile): Extension<BusinessProfile>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    let business_profile_id = active_business_profile.id.unwrap();

    let result = BusinessProfileUseCase::get_by_id(&state.conn, business_profile_id).await;

    if result.is_none() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        ));
    }

    let business_profile = result.unwrap();
    let response = BusinessProfileMapper::json(business_profile);

    Ok(Json(response))
}
