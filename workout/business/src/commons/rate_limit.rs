use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

// ponytail: single fixed-window counter behind one process-wide mutex — fine for
// one instance, resets on restart, and doesn't coordinate across replicas.
// Upgrade to a shared store (Redis) with a sliding window if this API ever runs
// behind more than one instance.
/// Per-IP fixed-window request counter shared by the REST middleware and the gRPC authentication layer.
#[derive(Clone)]
pub struct RateLimiter {
    max_requests: u32,
    window: Duration,
    buckets: Arc<Mutex<HashMap<IpAddr, (Instant, u32)>>>,
}

impl RateLimiter {
    pub fn new(max_requests: u32, window: Duration) -> Self {
        Self {
            max_requests,
            window,
            buckets: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// Counts one request from `ip`; false once the window's allowance is used up.
    pub fn check(&self, ip: IpAddr) -> bool {
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

#[cfg(test)]
#[path = "../tests/rate_limit_unit_test.rs"]
mod tests;
