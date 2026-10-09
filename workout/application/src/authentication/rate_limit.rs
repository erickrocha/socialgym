use crate::commons::exception_response::ExceptionResponse;
use crate::commons::i18n::{ErrorKey, Locale};
use axum::body::Body;
use axum::extract::Request;
use axum::http::header::HeaderName;
use axum::middleware::Next;
use axum::response::Response;
use business::commons::rate_limit::RateLimiter;
use std::net::IpAddr;
use std::time::Duration;

const REAL_IP_HEADER: HeaderName = HeaderName::from_static("x-real-ip");

/// Per-IP throttle for abuse-prone endpoints (login/signup, content creation).
/// The acting IP is read from `X-Real-IP`, which the nginx gateway always sets
/// (see infra/dev/locations.conf, infra/prod/nginx.conf) — falls back to the
/// socket's peer address for direct/local access outside the gateway.
pub async fn rate_limit(
    axum::extract::State(limiter): axum::extract::State<RateLimiter>,
    req: Request<Body>,
    next: Next,
) -> Result<Response, ExceptionResponse> {
    let ip = req
        .headers()
        .get(REAL_IP_HEADER)
        .and_then(|v| v.to_str().ok())
        .and_then(|s| s.parse::<IpAddr>().ok())
        .or_else(|| {
            req.extensions()
                .get::<axum::extract::ConnectInfo<std::net::SocketAddr>>()
                .map(|ci| ci.0.ip())
        })
        .unwrap_or(IpAddr::from([0, 0, 0, 0]));

    if !limiter.check(ip) {
        let locale = req
            .extensions()
            .get::<Locale>()
            .copied()
            .unwrap_or(Locale::En);
        return Err(ExceptionResponse::TooManyRequests(
            locale,
            ErrorKey::RateLimited,
        ));
    }

    Ok(next.run(req).await)
}

use std::sync::OnceLock;

/// Shared across every request to `/login` and `/signup`: 20 attempts/minute
/// per IP is enough for normal retry-after-typo use, tight enough to blunt a
/// credential-stuffing or signup-spam script.
pub fn auth_limiter() -> RateLimiter {
    static LIMITER: OnceLock<RateLimiter> = OnceLock::new();
    LIMITER
        .get_or_init(|| RateLimiter::new(20, Duration::from_secs(60)))
        .clone()
}

/// `/refresh` is reachable with a refresh token only, so a guessing script gets the same allowance as the
/// sign-in routes (C-010 owner decision 2026-10-07), counted apart from them.
pub fn refresh_limiter() -> RateLimiter {
    static LIMITER: OnceLock<RateLimiter> = OnceLock::new();
    LIMITER
        .get_or_init(|| RateLimiter::new(20, Duration::from_secs(60)))
        .clone()
}

/// The legal documents are public static text a client reads a few at a time (sign-up shows two or three),
/// so the allowance is wider than for credentials; it still stops a script from hammering the route.
pub fn legal_limiter() -> RateLimiter {
    static LIMITER: OnceLock<RateLimiter> = OnceLock::new();
    LIMITER
        .get_or_init(|| RateLimiter::new(60, Duration::from_secs(60)))
        .clone()
}

#[cfg(test)]
#[path = "../tests/rate_limit_unit_test.rs"]
mod tests;
