use business::use_cases::authentication::Authentication;
use tonic::{Request, Status};

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

#[cfg(test)]
#[path = "../tests/grpc_auth_unit_test.rs"]
mod tests;
