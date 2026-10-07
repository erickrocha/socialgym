use crate::auth::grpc_auth::BearerToken;
use business::commons::rate_limit::RateLimiter;
use business::commons::token_context::with_forwarded_token;
use business::gateway::consent_gateway::ConsentGateway;
use domain::business_error::{BusinessError, BusinessErrorKind};
use domain::user::User;
use std::future::Future;
use std::net::IpAddr;
use tonic::{Request, Status};

/// The one place a use-case error becomes a gRPC status (icd.md section 4). `Infrastructure`
/// is the kind every dependency failure (workout, MongoDB, SQS) is raised with, so it maps to
/// `UNAVAILABLE`: the only intended difference from REST, which answers `500`.
pub fn business_status(error: &BusinessError) -> Status {
    match error.kind {
        BusinessErrorKind::Validation => Status::invalid_argument(error.message.clone()),
        BusinessErrorKind::Unauthorized => Status::unauthenticated(error.message.clone()),
        BusinessErrorKind::Forbidden => Status::permission_denied(error.message.clone()),
        BusinessErrorKind::NotFound => Status::not_found(error.message.clone()),
        BusinessErrorKind::Conflict => Status::already_exists(error.message.clone()),
        BusinessErrorKind::Locked => Status::failed_precondition(error.message.clone()),
        // The message names internals (hosts, driver errors): keep it in the log, not on the wire.
        BusinessErrorKind::Infrastructure => {
            log::error!("dependency failure: {}", error.message);
            Status::unavailable("a dependency is unavailable, try again")
        }
    }
}

/// The caller of an authenticated request.
#[allow(clippy::result_large_err)]
pub fn caller<T>(request: &Request<T>) -> Result<(User, BearerToken), Status> {
    let user = request.extensions().get::<User>().cloned();
    let token = request.extensions().get::<BearerToken>().cloned();
    user.zip(token)
        .ok_or_else(|| Status::unauthenticated("missing or invalid credential"))
}

/// Runs `work` for an authenticated caller the way the REST middleware does: mandatory Terms and
/// Privacy consent first (`PERMISSION_DENIED` when missing, `UNAVAILABLE` when `workout` cannot
/// answer), then the work inside the forwarded-token scope so its own `workout` calls are
/// authenticated as the caller.
#[allow(clippy::result_large_err)]
pub async fn with_caller<T, F, Fut, R>(request: &Request<T>, work: F) -> Result<R, Status>
where
    F: FnOnce(User) -> Fut,
    Fut: Future<Output = Result<R, Status>>,
{
    let (user, BearerToken(token)) = caller(request)?;
    with_forwarded_token(Some(token), async move {
        ConsentGateway::require("terms").await.map_err(|e| business_status(&e))?;
        ConsentGateway::require("privacy").await.map_err(|e| business_status(&e))?;
        work(user).await
    })
    .await
}

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

#[cfg(test)]
#[path = "../tests/utils_unit_test.rs"]
mod tests;
