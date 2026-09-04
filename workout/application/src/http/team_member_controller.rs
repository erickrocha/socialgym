use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::team_member_json::TeamMemberJson;
use crate::http::json::team_member_page_json::TeamMemberPageJson;
use crate::infrastructure::mapper::{
    BusinessProfileMapper, Mapper, PersonMapper, TeamMemberMapper,
};
use crate::AppState;
use axum::extract::{Path, Request, State};
use axum::{Extension, Json};
use business::domain::business_profile::BusinessProfile;
use business::domain::enums::InviteStatus;
use business::domain::user::User;
use business::use_cases::team_member_use_case::TeamMemberUseCase;

/// The team-member endpoints have two possible actors, and the JWT says which one is active:
/// a business profile invites and cancels, the invited person accepts and denies.
fn active_business_profile(
    req: &Request,
    locale: Locale,
) -> Result<BusinessProfile, ExceptionResponse> {
    req.extensions()
        .get::<BusinessProfile>()
        .cloned()
        .ok_or(ExceptionResponse::Forbidden(
            locale,
            ErrorKey::BusinessProfileForbidden,
        ))
}

#[utoipa::path(
    get,
    path = "/workout/api/team-members",
    responses(
        (status = 200, description = "Consolidated team member information page", body = TeamMemberPageJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_team_members(
    state: State<AppState>,
    req: Request,
) -> HttpResponse<Json<TeamMemberPageJson>> {
    let locale = req.extensions().get::<Locale>().copied().unwrap_or(Locale::En);
    let current_user = req
        .extensions()
        .get::<User>()
        .ok_or(ExceptionResponse::Unauthorized(locale, ErrorKey::AuthHeaderMissing))?;
    let active_profile = req.extensions().get::<BusinessProfile>();

    // With an active business profile the caller is the team owner and sees its people; without
    // one the caller is a person and sees the teams that invited them.
    let (members, sent_requests) = match active_profile {
        Some(profile) => {
            let business_profile_id = profile.id.unwrap();
            (
                TeamMemberUseCase::find_all_persons(
                    &state.conn,
                    business_profile_id,
                    InviteStatus::Accepted,
                )
                .await,
                TeamMemberUseCase::find_all_persons(
                    &state.conn,
                    business_profile_id,
                    InviteStatus::Pending,
                )
                .await,
            )
        }
        None => (Vec::new(), Vec::new()),
    };

    let person_id = current_user.person_id;

    let teams = TeamMemberUseCase::find_all_business_profiles(
        &state.conn,
        person_id,
        InviteStatus::Accepted,
    )
    .await;
    let received_requests = TeamMemberUseCase::find_all_business_profiles(
        &state.conn,
        person_id,
        InviteStatus::Pending,
    )
    .await;

    Ok(Json(TeamMemberPageJson {
        members: PersonMapper::json_vec(members),
        sent_requests: PersonMapper::json_vec(sent_requests),
        teams: BusinessProfileMapper::json_vec(teams),
        received_requests: BusinessProfileMapper::json_vec(received_requests),
    }))
}

#[utoipa::path(
    get,
    path = "/workout/api/team-members/{person_id}",
    params(
        ("person_id" = i32, Path, description = "The person id"),
    ),
    responses(
        (status = 200, description = "Team membership of the active business profile and the person", body = TeamMemberJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn get_team_member(
    state: State<AppState>,
    Path(person_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    req: Request,
) -> HttpResponse<Json<TeamMemberJson>> {
    let business_profile_id = active_business_profile(&req, locale)?.id.unwrap();

    let result = TeamMemberUseCase::find_membership(&state.conn, business_profile_id, person_id)
        .await
        .map_err(|_e| ExceptionResponse::NotFound(locale, ErrorKey::TeamMemberNotFound))?;

    Ok(Json(TeamMemberMapper::json(result)))
}

#[utoipa::path(
    put,
    path = "/workout/api/team-members/request/{person_id}",
    responses(
        (status = 200, description = "Team member request sent"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("person_id" = i32, Path, description = "Person id that will receive the team member request")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn send_team_member_request(
    state: State<AppState>,
    Path(person_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    req: Request,
) -> HttpResponse<Json<()>> {
    let business_profile_id = active_business_profile(&req, locale)?.id.unwrap();

    let request_result =
        TeamMemberUseCase::send_team_member_request(&state.conn, business_profile_id, person_id)
            .await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::TeamMemberSendRequestFailed,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/workout/api/team-members/accept/{business_profile_id}",
    responses(
        (status = 200, description = "Team member request accepted"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("business_profile_id" = i32, Path, description = "Business profile id that sent the team member request")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn accept_team_member_request(
    state: State<AppState>,
    Path(business_profile_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    let request_result = TeamMemberUseCase::accept_team_member_request(
        &state.conn,
        business_profile_id,
        current_user.person_id,
    )
    .await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::TeamMemberAcceptRequestFailed,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/workout/api/team-members/deny/{business_profile_id}",
    responses(
        (status = 200, description = "Team member request denied"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("business_profile_id" = i32, Path, description = "Business profile id that sent the team member request")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn deny_team_member_request(
    state: State<AppState>,
    Path(business_profile_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    Extension(current_user): Extension<User>,
) -> HttpResponse<Json<()>> {
    let request_result = TeamMemberUseCase::deny_team_member_request(
        &state.conn,
        business_profile_id,
        current_user.person_id,
    )
    .await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::TeamMemberDenyRequestFailed,
        ));
    }
    Ok(Json(()))
}

#[utoipa::path(
    put,
    path = "/workout/api/team-members/cancel/{person_id}",
    responses(
        (status = 200, description = "Team member request canceled"),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    params(
        ("person_id" = i32, Path, description = "Person id whose team member request will be canceled")
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn cancel_team_member_request(
    state: State<AppState>,
    Path(person_id): Path<i32>,
    Extension(locale): Extension<Locale>,
    req: Request,
) -> HttpResponse<Json<()>> {
    let business_profile_id = active_business_profile(&req, locale)?.id.unwrap();

    let request_result =
        TeamMemberUseCase::cancel_team_member_request(&state.conn, business_profile_id, person_id)
            .await;
    if request_result.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::TeamMemberCancelRequestFailed,
        ));
    }
    Ok(Json(()))
}
