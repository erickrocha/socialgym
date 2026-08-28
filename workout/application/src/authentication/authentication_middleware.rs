use crate::commons::exception_response::ExceptionResponse;
use crate::commons::i18n::{ErrorKey, Locale};
use crate::AppState;
use axum::extract::State;
use axum::http::header::{ACCEPT_LANGUAGE, AUTHORIZATION};
use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use business::commons::legal_documents::{PRIVACY, TERMS};
use business::use_cases::authentication::{Authentication, ValidateError};
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use business::use_cases::consent_use_case::ConsentUseCase;

pub async fn authentication(
    state: State<AppState>,
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ExceptionResponse> {
    let locale = Locale::from_accept_language(
        req.headers()
            .get(ACCEPT_LANGUAGE)
            .and_then(|value| value.to_str().ok()),
    );
    req.extensions_mut().insert(locale);

    if req.uri().path().starts_with("/login") || req.uri().path().starts_with("/signup") {
        return Ok(next.run(req).await);
    }

    let auth_header = req.headers_mut().get(AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| ExceptionResponse::Forbidden(locale, ErrorKey::AuthHeaderMissing))?,
        None => {
            return Err(ExceptionResponse::Forbidden(
                locale,
                ErrorKey::RequiredHeaderValueMissing,
            ))
        }
    };

    let mut header = auth_header.split_whitespace();

    let (bearer, token) = (header.next(), header.next());

    if bearer != Some("Bearer") || token.is_none() {
        return Err(ExceptionResponse::Forbidden(
            locale,
            ErrorKey::InvalidJwtToken,
        ));
    }

    let auth_context = Authentication::validate(&state.conn, token.unwrap().to_string())
        .await
        .map_err(|e| match e {
            ValidateError::Revoked => {
                ExceptionResponse::Unauthorized(locale, ErrorKey::TokenRevoked)
            }
            ValidateError::Invalid => {
                ExceptionResponse::Unauthorized(locale, ErrorKey::BadCredentials)
            }
        })?;

    // Revoking a mandatory document enters restricted mode: authentication is
    // retained solely to re-accept, export data, or delete the account.
    let path = req.uri().path();
    let restricted_mode_path = path.contains("/consents")
        || path.contains("/data-exports")
        || path.contains("/account/delete");
    if !restricted_mode_path {
        ConsentUseCase::require_current(&state.conn, auth_context.user.person_id, TERMS)
            .await
            .map_err(|_| ExceptionResponse::Forbidden(locale, ErrorKey::ConsentRequired))?;
        ConsentUseCase::require_current(&state.conn, auth_context.user.person_id, PRIVACY)
            .await
            .map_err(|_| ExceptionResponse::Forbidden(locale, ErrorKey::ConsentRequired))?;
    }

    if let Some(business_profile_id) = auth_context.active_business_profile_id {
        let current_profile = BusinessProfileUseCase::get_by_id(&state.conn, business_profile_id)
            .await
            .ok_or(ExceptionResponse::Unauthorized(
                locale,
                ErrorKey::BusinessProfileNotFound,
            ))?;
        req.extensions_mut().insert(current_profile);
    }

    req.extensions_mut().insert(auth_context.user.clone());
    req.extensions_mut().insert(auth_context);
    Ok(next.run(req).await)
}
