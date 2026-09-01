use crate::commons::exception_response::ExceptionResponse;
use crate::commons::i18n::{ErrorKey, Locale};
use axum::body::Body;
use axum::extract::Request;
use axum::http::header::HeaderName;
use axum::middleware::Next;
use axum::response::Response;
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

const REAL_IP_HEADER: HeaderName = HeaderName::from_static("x-real-ip");

// ponytail: single fixed-window counter behind one process-wide mutex — fine for
// one instance, resets on restart, and doesn't coordinate across replicas.
// Upgrade to a shared store (Redis) with a sliding window if this API ever runs
// behind more than one instance.
#[derive(Clone)]
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    buckets: Arc<Mutex<HashMap<IpAddr, (Instant, u32)>>>,
}

impl RateLimiter {
    fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    fn check(&self, ip: IpAddr) -> bool {
        let mut buckets = self.buckets.lock().unwrap_or_else(|e| e.into_inner());
        let now = Instant::now();
        let entry = buckets.entry(ip).or_insert((now, 0));
        if now.duration_since(entry.0) >= self.window {
            *entry = (now, 0);
        }
        entry.1 += 1;
        entry.1 <= self.max_requests
    }
}

/// Shared across every content-creation request (posts, comments, reactions,
/// workout sessions): 60 requests/minute per IP is generous for real usage,
/// tight enough to blunt a content-spam script.
pub fn content_limiter() -> RateLimiter {
    static LIMITER: OnceLock<RateLimiter> = OnceLock::new();
    LIMITER
        .get_or_init(|| RateLimiter::new(60, Duration::from_secs(60)))
        .clone()
}

/// Chat sends and conversation creation get their own bucket so a chatty user
/// can't exhaust the shared content budget (and vice versa). 120/minute per IP.
pub fn chat_limiter() -> RateLimiter {
    static LIMITER: OnceLock<RateLimiter> = OnceLock::new();
    LIMITER
        .get_or_init(|| RateLimiter::new(120, Duration::from_secs(60)))
        .clone()
}

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

    if !limiter.check(ip) {
        let locale = req
            .extensions()
            .get::<Locale>()
            .copied()
            .unwrap_or(Locale::En);
        return Err(ExceptionResponse::TooManyRequests(locale, ErrorKey::RateLimited));
    }

    Ok(next.run(req).await)
}
