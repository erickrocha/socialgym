use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::business_profile_json::BusinessProfileJson;
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::infrastructure::mapper::{BusinessProfileMapper, Mapper};
use crate::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::user::User;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;

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
    let owner_id = current_user.id.unwrap();
    log::info!("Fetching business profiles for owner_id={}", owner_id);

    let result = BusinessProfileUseCase::get_by_owner_id(&state.conn, owner_id).await;

    if result.is_err() {
        log::error!(
            "Error fetching business profiles: {:?}",
            result.err().unwrap()
        );
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
    log::info!("Fetching business profile with id={}", business_profile_id);

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
