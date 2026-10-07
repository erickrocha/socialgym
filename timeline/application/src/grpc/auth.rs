use business::commons::token_context::with_forwarded_token;
use business::gateway::consent_gateway::ConsentGateway;
use business::use_cases::authentication::Authentication;
use domain::user::User;
use std::future::Future;
use tonic::{Request, Status};

use super::status::status_from_business;

const INTERNAL_SECRET_METADATA: &str = "x-internal-secret";

/// The raw bearer token of an authenticated call, kept so calls to `workout` made while serving
/// it can forward it (the same job the REST middleware does with the forwarded-token scope).
#[derive(Clone)]
pub struct BearerToken(pub String);

impl std::fmt::Debug for BearerToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("BearerToken(<redacted>)")
    }
}

/// Interceptor for every person-facing service: `authorization: Bearer <token>` is validated by
/// the same function as REST and the caller is stored in the request extensions. A missing,
/// malformed or invalid credential is `UNAUTHENTICATED`; the credential is never logged.
#[allow(clippy::result_large_err)]
pub fn user_interceptor(mut request: Request<()>) -> Result<Request<()>, Status> {
    let denied = || Status::unauthenticated("missing or invalid credential");
    let header = request
        .metadata()
        .get("authorization")
        .and_then(|value| value.to_str().ok())
        .ok_or_else(denied)?;
    let mut parts = header.split_whitespace();
    let token = match (parts.next(), parts.next(), parts.next()) {
        (Some("Bearer"), Some(token), None) => token.to_string(),
        _ => return Err(denied()),
    };
    let user = Authentication::validate_access_token(&token).map_err(|_| denied())?;
    request.extensions_mut().insert(user);
    request.extensions_mut().insert(BearerToken(token));
    Ok(request)
}

/// Interceptor for `InternalService`: only the shared service secret, no user token.
#[allow(clippy::result_large_err)]
pub fn internal_interceptor(request: Request<()>) -> Result<Request<()>, Status> {
    let expected = std::env::var("INTERNAL_SERVICE_SECRET").unwrap_or_default();
    let provided = request
        .metadata()
        .get(INTERNAL_SECRET_METADATA)
        .and_then(|value| value.to_str().ok())
        .unwrap_or_default();
    if !expected.is_empty() && secrets_match(provided, &expected) {
        Ok(request)
    } else {
        log::warn!("Rejected internal gRPC call: missing or invalid {INTERNAL_SECRET_METADATA}");
        Err(Status::unauthenticated("missing or invalid credential"))
    }
}

/// Comparison whose time does not depend on where the strings first differ.
pub fn secrets_match(provided: &str, expected: &str) -> bool {
    let (a, b) = (provided.as_bytes(), expected.as_bytes());
    let mut diff = a.len() ^ b.len();
    for i in 0..a.len().max(b.len()) {
        diff |= usize::from(a.get(i).copied().unwrap_or(0) ^ b.get(i).copied().unwrap_or(0));
    }
    diff == 0
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
        ConsentGateway::require("terms").await.map_err(|e| status_from_business(&e))?;
        ConsentGateway::require("privacy").await.map_err(|e| status_from_business(&e))?;
        work(user).await
    })
    .await
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn secrets_match_only_when_equal() {
        assert!(secrets_match("abc", "abc"));
        assert!(!secrets_match("abc", "abd"));
        assert!(!secrets_match("abc", "abcd"));
        assert!(!secrets_match("", "abc"));
        assert!(secrets_match("", ""));
    }

    #[test]
    fn the_token_never_appears_in_debug_output() {
        assert!(!format!("{:?}", BearerToken("secret-token".into())).contains("secret-token"));
    }
}
