use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::http::json::friend_json::FriendJson;
use crate::http::json::friend_page_json::FriendPageJson;
use crate::http::json::person_json::PersonJson;
use crate::infrastructure::mapper::{Mapper, PersonMapper};
use crate::AppState;
use axum::extract::{Path, Query, Request, State};
use axum::{Extension, Json};
use business::domain::user::User;
use std::collections::HashMap;

use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use business::use_cases::friend_use_case::FriendUseCase;
use business::use_cases::person_use_case::PersonUseCase;

pub async fn get_friend_relationships_by_id(
    State(state): State<AppState>,
    Path(person_id): Path<i32>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<FriendJson>>> {
    let friends = FriendUseCase::find_all_friend(&state.conn, person_id)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::FriendNotFound)
        })?;
    Ok(Json(
        friends
            .into_iter()
            .map(|friend| FriendJson {
                id: friend.id,
                person_id: friend.person_id,
                friend_id: friend.friend_id,
                person_uuid: friend.person_uuid,
                friend_uuid: friend.friend_uuid,
            })
            .collect(),
    ))
}

pub async fn get_friend_relationships_by_uuid(
    State(state): State<AppState>,
    Path(uuid): Path<String>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<Vec<FriendJson>>> {
    let person = PersonUseCase::find_by_uuid(&state.conn, uuid)
        .await
        .map_err(|error| {
            ExceptionResponse::from_business(error, locale, ErrorKey::PersonNotFound)
        })?;
    get_friend_relationships_by_id(State(state), Path(person.id.unwrap()), Extension(locale)).await
}

#[utoipa::path(
    get,
    path = "/workout/api/friends",
    params(
        ("radius_km" = Option<f64>, Query, description = "Search radius in kilometers (default: 200)"),
    ),
    responses(
        (status = 200, description = "Consolidated friend information page", body = FriendPageJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_friends(
    state: State<AppState>,
    Query(params): Query<HashMap<String, String>>,
    req: Request,
) -> HttpResponse<Json<FriendPageJson>> {
    let locale = req.extensions().get::<Locale>().copied().unwrap_or(Locale::En);
    let current_user = req
        .extensions()
        .get::<User>()
        .ok_or(ExceptionResponse::Unauthorized(locale, ErrorKey::AuthHeaderMissing))?;
    let person_id = current_user.person_id;

    let km = params
        .get("radius_km")
        .and_then(|s| s.parse::<f64>().ok())
        .unwrap_or(200.0);

    let suggestions = PersonUseCase::get_suggestions(&state.conn, person_id, km).await;

    let suggestions_json: Vec<PersonJson> =
        suggestions.into_iter().map(PersonMapper::json).collect();

    let friends = PersonUseCase::get_all_friends(&state.conn, person_id).await;

    let friends_json: Vec<PersonJson> = friends.into_iter().map(PersonMapper::json).collect();

    let receive_requests = PersonUseCase::get_all_received_requests(&state.conn, person_id).await;

    let receive_requests_json: Vec<PersonJson> = receive_requests
        .into_iter()
        .map(PersonMapper::json)
        .collect();

    let sent_requests = PersonUseCase::get_all_sent_requests(&state.conn, person_id).await;

    let sent_requests_json: Vec<PersonJson> =
        sent_requests.into_iter().map(PersonMapper::json).collect();

    let response = FriendPageJson {
        suggestions: suggestions_json,
        friends: friends_json,
        receive_requests: receive_requests_json,
        sent_requests: sent_requests_json,
    };

    Ok(Json(response))
}

#[utoipa::path(
    get,
    path = "/workout/api/friends/{friend_id}",
    params(
        ("friend_id" = i32, Path, description = "The friend id"),
    ),
    responses(
        (status = 200, description = "Friend information page", body = PersonJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_friend(
    state: State<AppState>,
    Path(friend_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(_current_user): Extension<User>,
) -> HttpResponse<Json<PersonJson>> {
    let friend_entity = PersonUseCase::get(&state.conn, friend_id).await;

    if friend_entity.is_err() {
        return Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::FriendNotFound,
        ));
    }

    Ok(Json(PersonMapper::json(friend_entity.unwrap())))
}
#[utoipa::path(
    put,
    path = "/workout/api/friends/request/{receiver_id}",
    responses(
        (status = 200, description = "Friend request sent"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("receiver_id" = i32, Path, description = "Person id will receive the friend request")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn send_friend_request(
    state: State<AppState>,
    Path(receiver_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    let person_id = current_user.person_id;
    let request_result =
        FriendUseCase::send_friend_request(&state.conn, person_id, receiver_id).await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::FriendSendRequestFailed,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/workout/api/friends/accept/{id}",
    responses(
        (status = 200, description = "Friend request accepted"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("id" = i32, Path, description = "Friend request sender person id to accept")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn accept_friend_request(
    state: State<AppState>,
    Path(receiver_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    let person_id = current_user.person_id;
    let request_result =
        FriendUseCase::accept_friend_request(&state.conn, person_id, receiver_id).await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::FriendAcceptRequestFailed,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/workout/api/friends/deny/{id}",
    responses(
        (status = 200, description = "Friend request denied"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("id" = i32, Path, description = "Friend request sender person id to deny")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn deny_friend_request(
    state: State<AppState>,
    Path(receiver_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    let person_id = current_user.person_id;
    let request_result =
        FriendUseCase::deny_friend_request(&state.conn, person_id, receiver_id).await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::FriendDenyRequestFailed,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/workout/api/friends/cancel/{sender_id}",
    responses(
        (status = 200, description = "Friend request canceled"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("sender_id" = i32, Path, description = "Who send the friend request id to be canceled")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn cancel_friend_request(
    state: State<AppState>,
    Path(sender_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    let person_id = current_user.person_id;
    let request_result =
        FriendUseCase::deny_friend_request(&state.conn, person_id, sender_id).await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::FriendCancelRequestFailed,
        ));
    }
    Ok(Json(()))
}
