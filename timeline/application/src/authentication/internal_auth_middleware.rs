use crate::commons::exception_response::ExceptionResponse;
use crate::commons::i18n::{ErrorKey, Locale};
use axum::http::header::HeaderName;
use axum::{body::Body, extract::Request, http::Response, middleware::Next};
use std::env;

const INTERNAL_SECRET_HEADER: &str = "X-Internal-Secret";
const INTERNAL_SERVICE_SECRET: &str = "INTERNAL_SERVICE_SECRET";

/// Authenticates service-to-service calls (e.g. `workout`'s account-purge
/// sweep) via a shared secret header, instead of the end-user JWT middleware.
/// Only ever used for internal-only routes never exposed to end users.
pub async fn internal_authentication(
    req: Request<Body>,
    next: Next,
) -> Result<Response<Body>, ExceptionResponse> {
    let expected = env::var(INTERNAL_SERVICE_SECRET).expect("INTERNAL_SERVICE_SECRET must be set");

    let provided = req
        .headers()
        .get(HeaderName::from_static("x-internal-secret"))
        .and_then(|value| value.to_str().ok());

    match provided {
        Some(secret) if secret == expected => Ok(next.run(req).await),
        _ => {
            log::warn!("Rejected internal request: missing or invalid {}", INTERNAL_SECRET_HEADER);
            Err(ExceptionResponse::Unauthorized(Locale::En, ErrorKey::InternalAuthInvalid))
        }
    }
}
