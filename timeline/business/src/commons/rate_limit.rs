use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex, OnceLock};
use std::time::{Duration, Instant};

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

    /// Counts a request from `ip` and tells whether it is within the budget of the window.
    pub fn allow(&self, ip: IpAddr) -> bool {
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
