use business::commons::legal_documents::{PRIVACY, TERMS};
use business::commons::rate_limit::RateLimiter;
use business::commons::secret::constant_time_eq;
use business::domain::business_error::BusinessErrorKind;
use business::domain::business_profile::BusinessProfile;
use business::use_cases::authentication::{AuthenticatedContext, Authentication, ValidateError};
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use business::use_cases::consent_use_case::ConsentUseCase;
use hyper::http::HeaderValue;
use hyper::{Request, Response, StatusCode, http};
use sea_orm::DatabaseConnection;
use std::future::Future;
use std::net::IpAddr;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::body::Body as TonicBody;
use tonic::server::NamedService;
use tower::{Layer, Service};

// Type alias for simpler signatures
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// gRPC methods that only the internal services may call: they carry `x-internal-secret` and never a
/// user token, so a user token never opens them and the secret never opens a user method.
const INTERNAL_METHODS: &[&str] = &[
    "/grpc.settings.SettingsService/GetPushPreferenceByOwnerUuid",
    "/grpc.team_member.TeamMemberService/GetTeamRoster",
];

/// Methods (or whole services, when the entry ends with `/`) that stay reachable when the caller has no
/// current Terms and Privacy consent: the consent check itself, consent management, and the account
/// recovery paths (data export, account deletion and cancelling it), as REST allows.
const CONSENT_EXEMPT: &[&str] = &[
    "/grpc.person.PersonService/HasActiveConsent",
    "/grpc.consent.ConsentService/",
    "/grpc.account.AccountService/",
];

/// A method reachable without an access token, limited per IP like REST limits login and signup.
#[derive(Clone)]
struct PublicMethod {
    path: &'static str,
    limiter: RateLimiter,
}

#[derive(Clone, Default)]
pub struct GrpcAuthLayer {
    conn: Arc<DatabaseConnection>,
    public: Arc<Vec<PublicMethod>>,
}

impl GrpcAuthLayer {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self {
            conn,
            public: Arc::new(Vec::new()),
        }
    }

    /// Allows `path` (for example `/grpc.auth.AuthService/Login`) without an access token, limited per IP
    /// by `limiter`. Every other method still needs a valid token.
    pub fn public_method(mut self, path: &'static str, limiter: RateLimiter) -> Self {
        Arc::make_mut(&mut self.public).push(PublicMethod { path, limiter });
        self
    }
}

impl<S> Layer<S> for GrpcAuthLayer {
    type Service = GrpcAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GrpcAuthMiddleware {
            inner,
            conn: Arc::clone(&self.conn),
            public: Arc::clone(&self.public),
        }
    }
}

#[derive(Clone)]
pub struct GrpcAuthMiddleware<S> {
    inner: S,
    conn: Arc<DatabaseConnection>,
    public: Arc<Vec<PublicMethod>>,
}

/// Forward `NamedService` so that `Server::add_service` can identify the
/// wrapped service by its gRPC service name (e.g. "workout.WorkoutService").
impl<S: NamedService> NamedService for GrpcAuthMiddleware<S> {
    const NAME: &'static str = S::NAME;
}

impl<S> Service<Request<TonicBody>> for GrpcAuthMiddleware<S>
where
    S: Service<Request<TonicBody>, Response = Response<TonicBody>> + Clone + Send + 'static,
    S::Future: Send + 'static,
    S::Error: Send + Sync + 'static,
{
    type Response = Response<TonicBody>;
    type Error = S::Error;
    type Future = BoxFuture<Result<Self::Response, Self::Error>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.inner.poll_ready(cx)
    }

    fn call(&mut self, mut req: Request<TonicBody>) -> Self::Future {
        let mut inner = self.inner.clone();

        let conn = Arc::clone(&self.conn);
        let public = Arc::clone(&self.public);

        Box::pin(async move {
            if let Some(method) = public.iter().find(|method| method.path == req.uri().path()) {
                if !method.limiter.check(client_ip(&req)) {
                    return Ok(grpc_error_response(
                        GRPC_RESOURCE_EXHAUSTED,
                        "too many requests",
                    ));
                }
                return inner.call(req).await;
            }

            if is_internal_request(&req) {
                let expected_secret = std::env::var("INTERNAL_SERVICE_SECRET").ok();
                if is_authorized_internal_request(&req, expected_secret.as_deref()) {
                    return inner.call(req).await;
                }
                return Ok(grpc_unauthenticated_response(
                    "missing or invalid internal service secret",
                ));
            }

            match authenticate_request(&mut req, Arc::clone(&conn)).await {
                // Insert each value under its own type: services look these up as
                // `User` / `BusinessProfile`, which a tuple extension never matches.
                Ok((auth_context, business_profile)) => {
                    if !is_consent_exempt(req.uri().path())
                        && let Err(response) =
                            require_current_consent(&conn, auth_context.user.person_id).await
                    {
                        return Ok(response);
                    }
                    // `logout` and the profile switch revoke this exact token, so they need its jti and exp.
                    req.extensions_mut().insert(auth_context.user.clone());
                    req.extensions_mut().insert(auth_context);
                    if let Some(business_profile) = business_profile {
                        req.extensions_mut().insert(business_profile);
                    }
                    inner.call(req).await
                }
                Err(response) => Ok(response),
            }
        })
    }
}

/// The address the gateway saw, from `X-Real-IP` (the nginx gateway always sets it); calls that reach the
/// server without it share one bucket.
fn client_ip(req: &Request<TonicBody>) -> IpAddr {
    req.headers()
        .get("x-real-ip")
        .and_then(|value| value.to_str().ok())
        .and_then(|value| value.parse::<IpAddr>().ok())
        .unwrap_or(IpAddr::from([0, 0, 0, 0]))
}

fn is_consent_exempt(path: &str) -> bool {
    CONSENT_EXEMPT.iter().any(|exempt| {
        if exempt.ends_with('/') {
            path.starts_with(exempt)
        } else {
            path == *exempt
        }
    })
}

/// Terms and Privacy must be current for an ordinary call, as the REST middleware requires.
async fn require_current_consent(
    conn: &DatabaseConnection,
    person_id: i32,
) -> Result<(), Response<TonicBody>> {
    for document in [TERMS, PRIVACY] {
        if let Err(error) = ConsentUseCase::require_current(conn, person_id, document).await {
            return Err(if error.kind == BusinessErrorKind::Infrastructure {
                grpc_error_response(GRPC_UNAVAILABLE, "consent check temporarily unavailable")
            } else {
                grpc_error_response(
                    GRPC_PERMISSION_DENIED,
                    &format!("{document} consent is required"),
                )
            });
        }
    }
    Ok(())
}

fn is_internal_request(req: &Request<TonicBody>) -> bool {
    INTERNAL_METHODS.contains(&req.uri().path())
}

fn is_authorized_internal_request(req: &Request<TonicBody>, expected_secret: Option<&str>) -> bool {
    let Some(expected_secret) = expected_secret.filter(|value| !value.is_empty()) else {
        return false;
    };
    req.headers()
        .get("x-internal-secret")
        .and_then(|value| value.to_str().ok())
        .map(|value| constant_time_eq(value, expected_secret))
        .unwrap_or(false)
}

async fn authenticate_request(
    req: &mut Request<TonicBody>,
    conn: Arc<DatabaseConnection>,
) -> Result<(AuthenticatedContext, Option<BusinessProfile>), Response<TonicBody>> {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .ok_or_else(|| grpc_unauthenticated_response("missing authorization header"))?;

    let auth_str = auth_header
        .to_str()
        .map_err(|_| grpc_unauthenticated_response("invalid authorization header"))?;

    let mut parts = auth_str.split_whitespace();
    let bearer = parts.next();
    let token = parts.next();

    if bearer != Some("Bearer") || token.is_none() {
        return Err(grpc_unauthenticated_response("invalid bearer token format"));
    }

    let token = token.unwrap().to_string();

    let auth_context = Authentication::validate(&conn, token)
        .await
        .map_err(|e| match e {
            ValidateError::Revoked => grpc_unauthenticated_response("token has been revoked"),
            ValidateError::Invalid => grpc_unauthenticated_response("invalid or expired token"),
            ValidateError::Unavailable => {
                grpc_error_response(GRPC_UNAVAILABLE, "authentication temporarily unavailable")
            }
        })?;

    if let Some(business_profile_id) = auth_context.active_business_profile_id {
        let business_profile = BusinessProfileUseCase::get_by_id(&conn, business_profile_id)
            .await
            .ok_or_else(|| grpc_unauthenticated_response("active business profile not found"))?;
        Ok((auth_context, Some(business_profile)))
    } else {
        Ok((auth_context, None))
    }
}

const GRPC_PERMISSION_DENIED: &str = "7";
const GRPC_RESOURCE_EXHAUSTED: &str = "8";
const GRPC_UNAVAILABLE: &str = "14";
const GRPC_UNAUTHENTICATED: &str = "16";

fn grpc_unauthenticated_response(message: &str) -> Response<TonicBody> {
    grpc_error_response(GRPC_UNAUTHENTICATED, message)
}

fn grpc_error_response(code: &'static str, message: &str) -> Response<TonicBody> {
    let mut response = Response::new(empty_grpc_body());

    *response.status_mut() = StatusCode::OK;

    let headers = response.headers_mut();
    headers.insert("content-type", HeaderValue::from_static("application/grpc"));
    headers.insert("grpc-status", HeaderValue::from_static(code));
    headers.insert(
        "grpc-message",
        HeaderValue::from_str(message).unwrap_or(HeaderValue::from_static("error")),
    );

    response
}

fn empty_grpc_body() -> TonicBody {
    TonicBody::empty()
}

#[cfg(test)]
mod tests {
    use super::{
        is_authorized_internal_request as is_authorized_internal_push_preference_request,
        is_internal_request as is_internal_push_preference_request,
    };
    use hyper::Request;
    use tonic::body::Body;

    #[test]
    fn internal_preference_route_is_exact() {
        let request = Request::builder()
            .uri("/grpc.settings.SettingsService/GetPushPreferenceByOwnerUuid")
            .body(Body::empty())
            .unwrap();
        assert!(is_internal_push_preference_request(&request));

        let public_method = Request::builder()
            .uri("/grpc.settings.SettingsService/GetByOwnerIds")
            .body(Body::empty())
            .unwrap();
        assert!(!is_internal_push_preference_request(&public_method));
    }

    #[test]
    fn internal_preference_requires_matching_secret_header() {
        let request = Request::builder()
            .uri("/grpc.settings.SettingsService/GetPushPreferenceByOwnerUuid")
            .header("x-internal-secret", "wrong-secret")
            .body(Body::empty())
            .unwrap();
        assert!(!is_authorized_internal_push_preference_request(
            &request,
            Some("test-secret")
        ));

        let missing_secret = Request::builder()
            .uri("/grpc.settings.SettingsService/GetPushPreferenceByOwnerUuid")
            .header("x-internal-secret", "test-secret")
            .body(Body::empty())
            .unwrap();
        assert!(!is_authorized_internal_push_preference_request(
            &missing_secret,
            None
        ));

        let request = Request::builder()
            .uri("/grpc.settings.SettingsService/GetPushPreferenceByOwnerUuid")
            .header("x-internal-secret", "test-secret")
            .body(Body::empty())
            .unwrap();
        assert!(is_authorized_internal_push_preference_request(
            &request,
            Some("test-secret")
        ));
    }
}

#[cfg(test)]
#[path = "../tests/grpc_auth_layer_unit_test.rs"]
mod layer_tests;
