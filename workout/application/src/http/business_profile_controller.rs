use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::business_profile_address_json::BusinessProfileAddressJson;
use crate::http::json::business_profile_json::BusinessProfileJson;
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::infrastructure::mapper::{BusinessProfileAddressMapper, BusinessProfileMapper, Mapper};
use crate::AppState;
use axum::extract::{Path, Query, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::enums::ProfileType;
use business::domain::user::User;
use business::use_cases::business_profile_address_use_case::BusinessProfileAddressUseCase;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;

pub async fn get_profile_by_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    let profile = BusinessProfileUseCase::get_by_id(&state.conn, id)
        .await
        .ok_or(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        ))?;
    let profile = BusinessProfileUseCase::present(&state.conn, profile, Some(&current_user)).await;
    Ok(Json(BusinessProfileMapper::json(profile)))
}

pub async fn get_profile_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    let profile = BusinessProfileUseCase::get_by_uuid(&state.conn, uuid)
        .await
        .ok_or(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        ))?;
    let profile = BusinessProfileUseCase::present(&state.conn, profile, Some(&current_user)).await;
    Ok(Json(BusinessProfileMapper::json(profile)))
}

pub async fn get_profiles_by_owner_id(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<BusinessProfileJson>>> {
    let profiles = BusinessProfileUseCase::get_by_owner_id(&state.conn, id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    let profiles = BusinessProfileUseCase::present_all(&state.conn, profiles, Some(&current_user)).await;
    Ok(Json(BusinessProfileMapper::json_vec(profiles)))
}

pub async fn get_profiles_by_owner_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<BusinessProfileJson>>> {
    let profiles = BusinessProfileUseCase::get_by_owner_uuid(&state.conn, uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    let profiles = BusinessProfileUseCase::present_all(&state.conn, profiles, Some(&current_user)).await;
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

pub async fn delete_profile(
    State(state): State<AppState>,
    Path(id): Path<i32>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<StatusCode> {
    BusinessProfileUseCase::delete(&state.conn, id, &current_user)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
        })?;
    Ok(StatusCode::NO_CONTENT)
}

pub async fn save_address(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<BusinessProfileAddressJson>,
) -> HttpResponse<Json<BusinessProfileAddressJson>> {
    let (latitude, longitude) = (payload.latitude, payload.longitude);
    let address = BusinessProfileAddressUseCase::save(
        &state.conn,
        BusinessProfileAddressMapper::domain(payload),
        current_user.person_id,
        latitude,
        longitude,
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
    active_business_profile: Option<Extension<BusinessProfile>>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<BusinessProfileJson>> {
    // Without an Active Business Profile in the token there is nothing to show: a client error, not the
    // `500` a missing request extension used to give.
    let Some(Extension(active_business_profile)) = active_business_profile else {
        return Err(ExceptionResponse::BadRequest(locale, ErrorKey::BusinessProfileNotFound));
    };
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

#[derive(Debug, serde::Deserialize)]
pub struct DiscoverBusinessProfilesQuery {
    pub query: Option<String>,
    pub business_type: Option<String>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub radius_km: Option<f64>,
    pub limit: Option<i32>,
}

#[utoipa::path(
    get,
    path = "/workout/api/business-profiles/discover",
    params(
        ("query" = Option<String>, Query, description = "Business/social name text to search for"),
        ("business_type" = Option<String>, Query, description = "Filter by profile type: \"Professional\" or \"Company\""),
        ("latitude" = Option<f64>, Query, description = "Center latitude for a location filter"),
        ("longitude" = Option<f64>, Query, description = "Center longitude for a location filter"),
        ("radius_km" = Option<f64>, Query, description = "Search radius in kilometers (default 200, capped at 500; a negative or non-finite value uses the default)"),
        ("limit" = Option<i32>, Query, description = "Max results (default 50, capped at 100)"),
    ),
    responses(
        (status = 200, description = "Business profiles matching the combined name/type/location search", body = [BusinessProfileJson]),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn discover(
    State(state): State<AppState>,
    Query(params): Query<DiscoverBusinessProfilesQuery>,
    Extension(current_user): Extension<User>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<BusinessProfileJson>>> {
    let business_type = match params.business_type.as_deref() {
        Some("Professional") => Some(ProfileType::Professional),
        Some("Company") => Some(ProfileType::Company),
        _ => None,
    };

    let profiles = BusinessProfileUseCase::discover(
        &state.conn,
        params.query,
        business_type,
        params.latitude,
        params.longitude,
        params.radius_km,
        params.limit.unwrap_or(50),
    )
    .await
    .map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::BusinessProfileNotFound)
    })?;
    let profiles = BusinessProfileUseCase::present_all(&state.conn, profiles, Some(&current_user)).await;

    Ok(Json(BusinessProfileMapper::json_vec(profiles)))
}
