use crate::commons::exception_response::ExceptionResponse;
use crate::commons::i18n::{ErrorKey, Locale};
use axum::http::header::{ACCEPT_LANGUAGE, AUTHORIZATION};
use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use business::commons::token_context::with_forwarded_token;
use business::gateway::consent_gateway::ConsentGateway;
use business::use_cases::authentication::Authentication;

pub async fn authentication(
    mut req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ExceptionResponse> {
    let locale = Locale::from_accept_language(
        req.headers()
            .get(ACCEPT_LANGUAGE)
            .and_then(|value| value.to_str().ok()),
    );
    req.extensions_mut().insert(locale);

    let auth_header = req.headers_mut().get(AUTHORIZATION);
    let auth_header = match auth_header {
        Some(header) => header
            .to_str()
            .map_err(|_| ExceptionResponse::Unauthorized(locale, ErrorKey::AuthHeaderEmpty))?,
        None => {
            log::warn!("No authorization header found in headers");
            return Err(ExceptionResponse::Unauthorized(
                locale,
                ErrorKey::AuthTokenMissing,
            ));
        }
    };

    let mut header = auth_header.split_whitespace();

    // Exactly `Bearer <token>`: a header with trailing parts is malformed, as it is for gRPC.
    let (bearer, token, extra) = (header.next(), header.next(), header.next());

    if bearer != Some("Bearer") || token.is_none() || extra.is_some() {
        log::info!("Invalid auth header format");
        return Err(ExceptionResponse::Unauthorized(
            locale,
            ErrorKey::AuthTokenInvalid,
        ));
    }

    // Capture the raw token string before validation consumes the slice.
    let token_str = token.unwrap().to_string();

    let current_user = Authentication::validate(token_str.clone()).await;

    if current_user.is_err() {
        log::info!("Invalid auth token");
        return Err(ExceptionResponse::Unauthorized(
            locale,
            ErrorKey::AuthTokenMalformed,
        ));
    }

    req.extensions_mut().insert(current_user.unwrap());

    let mandatory_consents = with_forwarded_token(Some(token_str.clone()), async {
        ConsentGateway::require("terms").await?;
        ConsentGateway::require("privacy").await
    })
    .await;
    // A missing consent is 403; `workout` being unreachable is a dependency outage (500), not a
    // refusal, the same difference gRPC draws between PERMISSION_DENIED and UNAVAILABLE.
    if let Err(error) = mandatory_consents {
        return Err(match error.kind {
            domain::business_error::BusinessErrorKind::Infrastructure => {
                ExceptionResponse::InternalServerError(locale, ErrorKey::ConsentRequired)
            }
            _ => ExceptionResponse::Forbidden(locale, ErrorKey::ConsentRequired),
        });
    }

    // Run the rest of the request stack inside the token scope so gRPC
    // interceptors lower in the call chain can read the token via
    // business::commons::token_context::current_forwarded_token().
    Ok(with_forwarded_token(Some(token_str), next.run(req)).await)
}
