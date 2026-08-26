use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::resource_json::ResourceJson;
use crate::infrastructure::mapper::{CountryMapper, Mapper, SettingsMapper};
use crate::AppState;
use axum::extract::State;
use axum::{Extension, Json};
use business::domain::user::User;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::resource_use_case::ResourceUseCase;
use business::use_cases::setings_use_case::SettingsUseCase;

#[utoipa::path(
	get,
	path = "/workout/api/resource",
	responses(
		(status = 200, description = "Static resources such as countries", body = ResourceJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
	),
	security(
		("bearer_auth" = [])
	)
)]
pub async fn get_resource(State(state): State<AppState>,Extension(current_user): Extension<User>,Extension(locale): Extension<Locale>) -> HttpResponse<Json<ResourceJson>> {
    let countries_response = ResourceUseCase::get_countries(state.conn.as_ref()).await;

    if countries_response.is_err() {
        return Err(ExceptionResponse::NotFound(locale,ErrorKey::ResourcesNotFound));
    }

    let setting_use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));
    let settings_response = setting_use_case.get_by_owner_id(current_user.id.unwrap()).await;
    if settings_response.is_err() {
        return Err(ExceptionResponse::NotFound(locale,ErrorKey::ResourcesNotFound));
    }

    let countries = CountryMapper::json_vec(countries_response.unwrap());
    let settings = SettingsMapper::json_opt(Some(settings_response.unwrap()));
    Ok(ResourceJson::builder()
        .countries(countries)
        .settings(settings)
        .build()
        .into())
}
