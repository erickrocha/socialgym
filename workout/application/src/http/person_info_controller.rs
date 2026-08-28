use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::person_info_json::PersonInfoJson;
use crate::infrastructure::mapper::{Mapper, PersonInfoMapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::{Extension, Json};
use business::commons::legal_documents;
use business::domain::user::User;
use business::use_cases::consent_use_case::ConsentUseCase;
use business::use_cases::person_info_use_case::PersonInfoUseCase;

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
    Extension(current_user): Extension<User>,
    Json(payload): Json<PersonInfoJson>,
) -> HttpResponse<Json<PersonInfoJson>> {
    let existing = PersonInfoUseCase::get(&state.conn, person_info_id)
        .await
        .map_err(|_| ExceptionResponse::BadRequest(locale, ErrorKey::PersonInfoNotUpdated))?;
    if existing.person_id != current_user.person_id {
        return Err(ExceptionResponse::Forbidden(
            locale,
            ErrorKey::PersonInfoNotUpdated,
        ));
    }
    if payload.weight.is_some() || payload.height.is_some() {
        ConsentUseCase::require_current(
            &state.conn,
            current_user.person_id,
            legal_documents::HEALTH_DATA,
        )
        .await
        .map_err(|_| ExceptionResponse::Forbidden(locale, ErrorKey::ConsentRequired))?;
    }
    let domain = PersonInfoMapper::domain(payload);

    let persisted = PersonInfoUseCase::update(&state.conn, person_info_id, domain).await;

    match persisted {
        Ok(person_info) => Ok(Json(PersonInfoMapper::json(person_info))),
        Err(_) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PersonInfoNotUpdated,
        )),
    }
}
