use crate::infrastructure::mapper::AccessTokenMapper;
use crate::infrastructure::utils::{locale_of, localized_status, require_actor};
use crate::proto::auth::auth_service_server::AuthService;
use crate::proto::auth::{
    AccessToken, ActivateBusinessProfileRequest, DeactivateBusinessProfileRequest, LoginRequest, LogoutRequest,
    LogoutResponse, RefreshRequest, SignupRequest,
};
use business::commons::functions::parse_uuid;
use business::commons::i18n::ErrorKey;
use business::domain::business_error::BusinessErrorKind;
use business::use_cases::authentication::{AuthenticatedContext, Authentication, AuthenticationError};
use business::use_cases::logout_use_case::LogoutUseCase;
use business::use_cases::refresh_token::RefreshToken;
use business::use_cases::sign_up_use_case::{SignUpError, SignUpRequest, SignUpUseCase};
use business::use_cases::switch_business_profile::{SwitchBusinessProfile, SwitchBusinessProfileError};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

/// The gRPC twin of the REST authentication routes. It only translates: the rules live in the use cases
/// the REST controller calls, and the errors carry the REST text in the caller's locale.
pub struct GrpcAuthService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcAuthService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

fn header<T>(request: &Request<T>, name: &str) -> Option<String> {
    request.metadata().get(name).and_then(|value| value.to_str().ok()).map(str::to_string)
}

fn context<T>(request: &Request<T>) -> Result<AuthenticatedContext, Status> {
    request
        .extensions()
        .get::<AuthenticatedContext>()
        .cloned()
        .ok_or_else(|| Status::unauthenticated("missing authenticated user"))
}

#[tonic::async_trait]
impl AuthService for GrpcAuthService {
    async fn signup(&self, request: Request<SignupRequest>) -> Result<Response<AccessToken>, Status> {
        let locale = locale_of(&request);
        let (real_ip, forwarded_for) = (header(&request, "x-real-ip"), header(&request, "x-forwarded-for"));
        let payload = request.into_inner();
        let date_of_birth = chrono::NaiveDate::parse_from_str(&payload.date_of_birth, "%Y-%m-%d")
            .map_err(|_| localized_status(Code::InvalidArgument, ErrorKey::InvalidParameterValue, locale))?;
        let result = SignUpUseCase::execute(
            &self.conn,
            SignUpRequest {
                firstname: payload.firstname,
                surname: payload.surname,
                date_of_birth,
                gender: payload.gender,
                email: payload.email,
                password: payload.password,
                terms_accepted: payload.terms_accepted,
                privacy_accepted: payload.privacy_accepted,
                terms_version: payload.terms_version,
                privacy_version: payload.privacy_version,
                language: locale.to_string(),
                real_ip,
                forwarded_for,
            },
            chrono::Utc::now().date_naive(),
        )
        .await;
        match result {
            Ok(token) => Ok(Response::new(AccessTokenMapper::response(token))),
            Err(SignUpError::Underage) => Err(localized_status(Code::InvalidArgument, ErrorKey::UnderageRegistration, locale)),
            Err(SignUpError::ConsentRequired) => Err(localized_status(Code::InvalidArgument, ErrorKey::ConsentRequired, locale)),
            Err(SignUpError::WeakPassword) => Err(localized_status(Code::InvalidArgument, ErrorKey::WeakPassword, locale)),
            Err(SignUpError::Failed) => Err(localized_status(Code::InvalidArgument, ErrorKey::SignUpUserFailed, locale)),
            Err(SignUpError::Unauthorized) => Err(localized_status(Code::Unauthenticated, ErrorKey::UnknowAuthError, locale)),
        }
    }

    async fn login(&self, request: Request<LoginRequest>) -> Result<Response<AccessToken>, Status> {
        let locale = locale_of(&request);
        let payload = request.into_inner();
        match Authentication::execute(&self.conn, payload.email, payload.password).await {
            Ok(token) => Ok(Response::new(AccessTokenMapper::response(token))),
            Err(AuthenticationError::AccountLocked { .. }) => Err(localized_status(Code::FailedPrecondition, ErrorKey::AccountLocked, locale)),
            Err(AuthenticationError::InvalidCredentials) => Err(localized_status(Code::Unauthenticated, ErrorKey::BadCredentials, locale)),
            Err(AuthenticationError::AccountDisabled) => Err(localized_status(Code::PermissionDenied, ErrorKey::AccountDisabled, locale)),
        }
    }

    async fn refresh(&self, request: Request<RefreshRequest>) -> Result<Response<AccessToken>, Status> {
        let locale = locale_of(&request);
        match RefreshToken::execute(&self.conn, request.into_inner().refresh_token).await {
            Ok(token) => Ok(Response::new(AccessTokenMapper::response(token))),
            Err(error) if error.kind == BusinessErrorKind::Infrastructure => {
                Err(localized_status(Code::Unavailable, ErrorKey::UnknowAuthError, locale))
            }
            Err(_) => Err(localized_status(Code::Unauthenticated, ErrorKey::BadCredentials, locale)),
        }
    }

    async fn logout(&self, request: Request<LogoutRequest>) -> Result<Response<LogoutResponse>, Status> {
        let user = require_actor(&request)?;
        let auth = context(&request)?;
        let refresh_token = Some(request.into_inner().refresh_token).filter(|token| !token.is_empty());
        // As REST: the session ends even when the revocation could not be stored.
        let _ = LogoutUseCase::execute(&self.conn, user.id.unwrap_or_default(), auth.jti, auth.exp, refresh_token).await;
        Ok(Response::new(LogoutResponse {}))
    }

    async fn activate_business_profile(&self, request: Request<ActivateBusinessProfileRequest>) -> Result<Response<AccessToken>, Status> {
        let locale = locale_of(&request);
        let user = require_actor(&request)?;
        let auth = context(&request)?;
        let uuid = request.into_inner().business_profile_uuid;
        if parse_uuid(&uuid).is_err() {
            return Err(localized_status(Code::InvalidArgument, ErrorKey::InvalidParameterValue, locale));
        }
        match SwitchBusinessProfile::activate(&self.conn, &user, uuid, auth.jti, auth.exp).await {
            Ok(token) => Ok(Response::new(AccessTokenMapper::response(token))),
            Err(SwitchBusinessProfileError::NotFound) => Err(localized_status(Code::NotFound, ErrorKey::BusinessProfileNotFound, locale)),
            Err(SwitchBusinessProfileError::Forbidden) => Err(localized_status(Code::PermissionDenied, ErrorKey::BusinessProfileForbidden, locale)),
        }
    }

    async fn deactivate_business_profile(&self, request: Request<DeactivateBusinessProfileRequest>) -> Result<Response<AccessToken>, Status> {
        let user = require_actor(&request)?;
        let auth = context(&request)?;
        let token = SwitchBusinessProfile::deactivate(&self.conn, &user, auth.jti, auth.exp).await;
        Ok(Response::new(AccessTokenMapper::response(token)))
    }
}
