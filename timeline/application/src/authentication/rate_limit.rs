use crate::commons::exception_response::ExceptionResponse;
use crate::commons::i18n::{ErrorKey, Locale};
use axum::body::Body;
use axum::extract::Request;
use axum::http::header::HeaderName;
use axum::middleware::Next;
use axum::response::Response;
use business::commons::rate_limit::RateLimiter;
use std::net::IpAddr;

const REAL_IP_HEADER: HeaderName = HeaderName::from_static("x-real-ip");

/// Per-IP throttle for abuse-prone endpoints. The acting IP is read from
/// `X-Real-IP`, which the nginx gateway always sets (see
/// infra/dev/locations.conf, infra/prod/nginx.conf) — falls back to the
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

    if !limiter.allow(ip) {
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
