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
use axum::http::HeaderMap;
use axum::http::StatusCode;
use axum::{Extension, Form, Json};
use business::commons::functions::parse_uuid;
use business::domain::business_error::BusinessErrorKind;
use business::domain::user::User;
use business::use_cases::logout_use_case::LogoutUseCase;
use business::use_cases::sign_up_use_case::{SignUpError, SignUpRequest, SignUpUseCase};
use business::use_cases::switch_business_profile::{
    SwitchBusinessProfile, SwitchBusinessProfileError,
};
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
    headers: HeaderMap,
    Json(payload): Json<SignUpJson>,
) -> HttpResponse<Json<AccessTokenJson>> {
    let header = |name: &str| headers.get(name).and_then(|value| value.to_str().ok()).map(str::to_string);
    let result = SignUpUseCase::execute(
        &state.conn,
        SignUpRequest {
            firstname: payload.firstname,
            surname: payload.surname,
            date_of_birth: payload.date_of_birth,
            gender: payload.gender,
            email: payload.email,
            password: payload.password,
            terms_accepted: payload.terms_accepted,
            privacy_accepted: payload.privacy_accepted,
            terms_version: payload.terms_version,
            privacy_version: payload.privacy_version,
            language: locale.to_string(),
            real_ip: header("x-real-ip"),
            forwarded_for: header("x-forwarded-for"),
        },
        chrono::Utc::now().date_naive(),
    )
    .await;
    match result {
        Ok(token) => Ok(Json(AccessTokenMapper::json(token))),
        Err(SignUpError::Underage) => Err(ExceptionResponse::BadRequest(locale, ErrorKey::UnderageRegistration)),
        Err(SignUpError::ConsentRequired) => Err(ExceptionResponse::BadRequest(locale, ErrorKey::ConsentRequired)),
        Err(SignUpError::WeakPassword) => Err(ExceptionResponse::BadRequest(locale, ErrorKey::WeakPassword)),
        Err(SignUpError::Failed) => Err(ExceptionResponse::BadRequest(locale, ErrorKey::SignUpUserFailed)),
        Err(SignUpError::Unauthorized) => Err(ExceptionResponse::Unauthorized(locale, ErrorKey::UnknowAuthError)),
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
        Err(AuthenticationError::AccountDisabled) => Err(ExceptionResponse::Forbidden(
            locale,
            ErrorKey::AccountDisabled,
        )),
    }
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshTokenRequest,
    responses(
        (status = 200, description = "Token refreshed successfully", body = AccessTokenJson),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error", body = InternalServerErrorJson)
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
        Err(error) if error.kind == BusinessErrorKind::Infrastructure => Err(
            ExceptionResponse::InternalServerError(locale, ErrorKey::UnknowAuthError),
        ),
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
