use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::address_candidate_json::AddressCandidateJson;
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::infrastructure::mapper::{AddressCandidateMapper, Mapper};
use axum::extract::Query;
use axum::Json;
use business::domain::business_error::BusinessErrorKind;
use business::use_cases::address_search_use_case::AddressSearchUseCase;
use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
pub struct AddressSearchParams {
    pub text: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
}

#[utoipa::path(
	get,
	path = "/workout/api/address/search",
	params(AddressSearchParams),
	responses(
		(status = 200, description = "Candidate addresses matching the typed text, biased by GPS when provided", body = [AddressCandidateJson]),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
	),
	security(
		("bearer_auth" = [])
	)
)]
pub async fn search_address(
    Query(params): Query<AddressSearchParams>,
    axum::Extension(locale): axum::Extension<Locale>,
) -> HttpResponse<Json<Vec<AddressCandidateJson>>> {
    let result =
        AddressSearchUseCase::search(&params.text, params.latitude, params.longitude).await;

    match result {
        Ok(candidates) => Ok(Json(AddressCandidateMapper::json_vec(candidates))),
        Err(error) => {
            let key = match error.kind {
                BusinessErrorKind::Forbidden => ErrorKey::AddressSearchDisabled,
                _ => ErrorKey::AddressSearchFailed,
            };
            Err(ExceptionResponse::from_business(error, locale, key))
        }
    }
}
