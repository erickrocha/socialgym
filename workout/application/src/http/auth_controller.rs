use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::access_token_json::AccessTokenJson;
use crate::http::json::error_response_json::{
    BadRequestErrorJson, ForbiddenErrorJson, InternalServerErrorJson, UnauthorizedErrorJson,
};
use crate::http::json::login_request::LoginRequest;
use crate::http::json::logout_request::LogoutRequest;
use crate::http::json::refresh_token_request::RefreshTokenRequest;
use crate::http::json::sign_up_json::SignUpJson;
use crate::infrastructure::mapper::{AccessTokenMapper, Mapper};
use crate::AppState;
use axum::extract::{Path, State};
use axum::http::StatusCode;
use axum::{Extension, Form, Json};
use business::commons::functions::parse_uuid;
use business::domain::enums::Position;
use business::domain::person::Person;
use business::domain::settings::Settings;
use business::domain::user::User;
use business::gateway::settings_gateway::SettingsGateway;
use business::use_cases::logout_use_case::LogoutUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use business::use_cases::setings_use_case::SettingsUseCase;
use business::use_cases::switch_business_profile::{
    SwitchBusinessProfile, SwitchBusinessProfileError,
};
use business::use_cases::user_use_case::{UserUseCase, UserUseCaseError};
use business::use_cases::{
    authentication::{AuthenticatedContext, Authentication, AuthenticationError},
    refresh_token::RefreshToken,
};

#[utoipa::path(
    post,
    path = "/signup",
    request_body = SignUpJson,
    responses(
        (status = 200, description = "User created successfully and access token returned", body = AccessTokenJson),
        (status = 400, description = "Bad request")
    )
)]
pub async fn sign_up(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Json(payload): Json<SignUpJson>,
) -> HttpResponse<Json<AccessTokenJson>> {
    let person = Person::new(
        payload.firstname,
        payload.surname,
        payload.date_of_birth,
        payload.gender,
    );

    let person_added = PersonUseCase::add(&state.conn, person).await;
    if person_added.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::SignUpPersonFailed,
        ));
    }

    let person_domain = person_added.unwrap();
    let person_id = person_domain.id.unwrap();
    let person_uuid = person_domain.uuid.unwrap();

    let settings = Settings::new(
        person_id,
        person_uuid.clone(),
        locale.to_string(),
        "default".to_string(),
        true,
        Position::Left,
        "Feed".to_string(),
    );
    let settings_use_case = SettingsUseCase::new(SettingsGateway::new((*state.conn).clone()));
    let settings_added = settings_use_case.persist(settings).await;
    if settings_added.is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::SettingsAddedFailed,
        ));
    }

    let name = format!("{} {}", person_domain.firstname, person_domain.surname);
    let user: User = User::new(
        Some(name),
        payload.email,
        payload.password,
        person_id,
        person_uuid,
    );

    let password = user.password.clone();
    let user_added = UserUseCase::add(&state.conn, user).await;

    let current = match user_added {
        Ok(current) => current,
        Err(UserUseCaseError::WeakPassword(_violations)) => {
            return Err(ExceptionResponse::BadRequest(
                locale,
                ErrorKey::WeakPassword,
            ));
        }
        Err(_e) => {
            return Err(ExceptionResponse::BadRequest(
                locale,
                ErrorKey::SignUpUserFailed,
            ));
        }
    };

    let access_token = Authentication::execute(&state.conn, current.email, password).await;
    match access_token {
        Ok(token) => Ok(Json(AccessTokenMapper::json(token))),
        Err(_e) => Err(ExceptionResponse::Unauthorized(
            locale,
            ErrorKey::UnknowAuthError,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/login",
    request_body(content = LoginRequest, content_type = "application/x-www-form-urlencoded"),
    responses(
        (status = 200, description = "Login successful", body = AccessTokenJson),
        (status = 401, description = "Unauthorized"),
        (status = 423, description = "Account locked")
    )
)]
pub async fn sign_in(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Form(login_request): Form<LoginRequest>,
) -> HttpResponse<Json<AccessTokenJson>> {
    let access_token =
        Authentication::execute(&state.conn, login_request.email, login_request.password).await;
    match access_token {
        Ok(token) => Ok(Json(AccessTokenMapper::json(token))),
        Err(AuthenticationError::AccountLocked { .. }) => {
            Err(ExceptionResponse::Locked(locale, ErrorKey::AccountLocked))
        }
        Err(AuthenticationError::InvalidCredentials) => Err(ExceptionResponse::Unauthorized(
            locale,
            ErrorKey::BadCredentials,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AccessTokenJson),
        (status = 401, description = "Unauthorized")
    )
)]
pub async fn refresh_token(
    state: State<AppState>,
    Extension(locale): Extension<Locale>,
    Json(request): Json<RefreshTokenRequest>,
) -> HttpResponse<Json<AccessTokenJson>> {
    let access_token = RefreshToken::execute(&state.conn, request.refresh_token).await;
    match access_token {
        Ok(token) => Ok(Json(AccessTokenMapper::json(token))),
        Err(_) => Err(ExceptionResponse::Unauthorized(
            locale,
            ErrorKey::BadCredentials,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/logout",
    request_body = LogoutRequest,
    responses(
        (status = 204, description = "Logged out successfully"),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn logout(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(auth_context): Extension<AuthenticatedContext>,
    Json(payload): Json<LogoutRequest>,
) -> StatusCode {
    let result = LogoutUseCase::execute(
        &state.conn,
        current_user.id.unwrap(),
        auth_context.jti,
        auth_context.exp,
        payload.refresh_token,
    )
    .await;
    if let Err(_e) = result {}
    StatusCode::NO_CONTENT
}

#[utoipa::path(
    post,
    path = "/auth/profile/{business_profile_uuid}/activate",
    params(
        ("business_profile_uuid" = String, Path, description = "Business profile uuid")
    ),
    responses(
        (status = 200, description = "New access token with the business profile active", body = AccessTokenJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 404, description = "Business profile not found", body = InternalServerErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn activate(
    state: State<AppState>,
    Path(business_profile_uuid): Path<String>,
    Extension(current_user): Extension<User>,
    Extension(auth_context): Extension<AuthenticatedContext>,
    Extension(locale): Extension<Locale>,
) -> HttpResponse<Json<AccessTokenJson>> {
    if parse_uuid(&business_profile_uuid).is_err() {
        return Err(ExceptionResponse::BadRequest(
            locale,
            ErrorKey::InvalidParameterValue,
        ));
    }

    let result = SwitchBusinessProfile::activate(
        &state.conn,
        &current_user,
        business_profile_uuid,
        auth_context.jti,
        auth_context.exp,
    )
    .await;

    match result {
        Ok(access_token) => Ok(Json(AccessTokenMapper::json(access_token))),
        Err(SwitchBusinessProfileError::NotFound) => Err(ExceptionResponse::NotFound(
            locale,
            ErrorKey::BusinessProfileNotFound,
        )),
        Err(SwitchBusinessProfileError::Forbidden) => Err(ExceptionResponse::Forbidden(
            locale,
            ErrorKey::BusinessProfileForbidden,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/auth/profile/deactivate",
    responses(
        (status = 200, description = "New access token back in personal context", body = AccessTokenJson),
        (status = 400, description = "Bad request", body = BadRequestErrorJson),
        (status = 401, description = "Unauthorized", body = UnauthorizedErrorJson),
        (status = 403, description = "Forbidden", body = ForbiddenErrorJson),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson),
    ),
    security(
        ("bearer_auth" = [])
    )
)]
pub async fn deactivate(
    state: State<AppState>,
    Extension(current_user): Extension<User>,
    Extension(auth_context): Extension<AuthenticatedContext>,
) -> HttpResponse<Json<AccessTokenJson>> {
    let access_token = SwitchBusinessProfile::deactivate(
        &state.conn,
        &current_user,
        auth_context.jti,
        auth_context.exp,
    )
    .await;

    Ok(Json(AccessTokenMapper::json(access_token)))
}
