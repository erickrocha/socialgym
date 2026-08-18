use axum::extract::{Path, State};
use axum::{Extension, Json};
use business::use_cases::person_info_use_case::PersonInfoUseCase;
use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson};
use crate::http::json::person_info_json::PersonInfoJson;
use crate::infrastructure::mapper::{Mapper, PersonInfoMapper};

#[utoipa::path(
	put,
	path = "/workout/api/people/me/info/{person_info_id}",
	request_body = PersonInfoJson,
	responses(
        (status = 200, description = "Person Info updated", body = PersonInfoJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
	),
	params(
		("person_info_id" = i32, Path, description = "Person info id")
	),
	security(
		("bearer_auth" = [])
	)
)]
pub async fn update_person_info(
	state: State<AppState>,
	Path(person_info_id): Path<i32>,
	Extension(locale): Extension<Locale>,
	Json(payload): Json<PersonInfoJson>,
) -> HttpResponse<Json<PersonInfoJson>> {
	let domain = PersonInfoMapper::domain(payload);

	let persisted = PersonInfoUseCase::update(&state.conn, person_info_id, domain).await;

	match persisted {
		Ok(person_info) => Ok(Json(PersonInfoMapper::json(person_info))),
		Err(_) => Err(ExceptionResponse::BadRequest(locale,ErrorKey::PersonInfoNotUpdated)),
	}
}