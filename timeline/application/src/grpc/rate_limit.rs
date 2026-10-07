use crate::authentication::rate_limit::RateLimiter;
use std::net::IpAddr;
use tonic::{Request, Status};

/// Applies the same per-IP limiter REST uses to a gRPC call (`429` becomes `RESOURCE_EXHAUSTED`).
/// The address comes from `x-real-ip` (set by the gateway) or the socket peer.
#[allow(clippy::result_large_err)]
pub fn enforce<T>(limiter: &RateLimiter, request: &Request<T>) -> Result<(), Status> {
    let ip = request
        .metadata()
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<IpAddr>().ok())
        .or_else(|| request.remote_addr().map(|addr| addr.ip()))
        .unwrap_or(IpAddr::from([0, 0, 0, 0]));
    if limiter.allow(ip) {
        Ok(())
    } else {
        Err(Status::resource_exhausted("too many requests"))
    }
}
