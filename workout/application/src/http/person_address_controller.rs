use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::person_address_json::PersonAddressJson;
use crate::infrastructure::mapper::{Mapper, PersonAddressMapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Json};
use business::domain::user::User;
use business::use_cases::person_address_use_case::PersonAddressUseCase;

#[utoipa::path(
  post,
  path = "/workout/api/people/me/address",
	request_body = PersonAddressJson,
	responses(
        (status = 201, description = "Person address created", body = PersonAddressJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
	),
	security(
		("bearer_auth" = [])
	)
)]
pub async fn add_person_address(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<PersonAddressJson>,
) -> HttpResponse<(StatusCode, Json<PersonAddressJson>)> {
    let person_id = current_user.person_id;
    let mut address = PersonAddressMapper::domain(payload);
    address.person_id = person_id;

    let result = PersonAddressUseCase::add_person_address(&state.conn, address).await;

    match result {
        Ok(address) => Ok((
            StatusCode::CREATED,
            Json(PersonAddressMapper::json(address)),
        )),
        Err(_) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PersonAddressNotAdded,
        )),
    }
}

#[utoipa::path(
  put,
  path = "/workout/api/people/me/address/{person_address_id}",
  request_body = PersonAddressJson,
	responses(
    (status = 200, description = "Person address updated", body = PersonAddressJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
	),
	params(
    ("person_address_id" = i32, Path, description = "Person address id")
	),
	security(
    ("bearer_auth" = [])
	)
)]
pub async fn update_person_address(
    state: State<AppState>,
    Path(person_address_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
    Json(payload): Json<PersonAddressJson>,
) -> HttpResponse<Json<PersonAddressJson>> {
    let person_id = current_user.person_id;

    let mut address = PersonAddressMapper::domain(payload);
    address.id = Some(person_address_id);
    address.person_id = person_id;

    let result =
        PersonAddressUseCase::update_person_address(&state.conn, address, person_id).await;

    match result {
        Ok(address) => Ok(Json(PersonAddressMapper::json(address))),
        Err(_) => Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::PersonAddressNotUpdated,
        )),
    }
}

#[utoipa::path(
	delete,
	path = "/workout/api/people/me/address/{person_address_id}",
	responses(
    (status = 200, description = "Person address deleted"),
    (status = 400, description = "Bad request", body = BadRequestErrorJson),
    (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
    (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
    (status = 500, description = "Internal server error", body = InternalServerErrorJson)),
	params(
    ("person_address_id" = i32, Path, description = "Person address id")),
	security(
    ("bearer_auth" = []))
)]
pub async fn delete_person_address(
    state: State<AppState>,
    Path(person_address_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    PersonAddressUseCase::delete_person_address(
        &state.conn,
        person_address_id,
        current_user.person_id,
    )
    .await
    .map_err(|error| {
        ExceptionResponse::from_business(error, locale, ErrorKey::PersonAddressNotDeleted)
    })?;
    Ok(Json(()))
}

pub async fn delete_person_address_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    PersonAddressUseCase::delete_person_address_by_uuid(&state.conn, uuid, current_user.person_id)
        .await
        .map_err(|error| ExceptionResponse::from_business(error, locale, ErrorKey::PersonAddressNotDeleted))?;
    Ok(Json(()))
}
