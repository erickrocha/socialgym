use business::domain::user::User;
use business::use_cases::authentication::{Authentication, ValidateError};
use hyper::http::HeaderValue;
use hyper::{http, Request, Response, StatusCode};
use sea_orm::DatabaseConnection;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};
use tonic::body::Body as TonicBody;
use tonic::server::NamedService;
use tower::{Layer, Service};
use business::domain::business_profile::BusinessProfile;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;

// Type alias for simpler signatures
type BoxFuture<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

#[derive(Clone, Default)]
pub struct GrpcAuthLayer {
    conn: Arc<DatabaseConnection>,
}

impl GrpcAuthLayer {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}



impl<S> Layer<S> for GrpcAuthLayer {
    type Service = GrpcAuthMiddleware<S>;

    fn layer(&self, inner: S) -> Self::Service {
        GrpcAuthMiddleware {
            inner,
            conn: Arc::clone(&self.conn),
        }
    }
}

#[derive(Clone)]
pub struct GrpcAuthMiddleware<S> {
    inner: S,
    conn: Arc<DatabaseConnection>,
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

        Box::pin(async move {
            match authenticate_request(&mut req, conn).await {
                Ok(user) => {
                    req.extensions_mut().insert(user);
                    inner.call(req).await
                }
                Err(response) => Ok(response),
            }
        })
    }
}

async fn authenticate_request(req: &mut Request<TonicBody>,conn: Arc<DatabaseConnection>) -> Result<(User,Option<BusinessProfile>), Response<TonicBody>> {
    let auth_header = req.headers().get(http::header::AUTHORIZATION).ok_or_else(|| grpc_unauthenticated_response("missing authorization header"))?;

    let auth_str = auth_header.to_str().map_err(|_| grpc_unauthenticated_response("invalid authorization header"))?;

    let mut parts = auth_str.split_whitespace();
    let bearer = parts.next();
    let token = parts.next();

    if bearer != Some("Bearer") || token.is_none() {
        return Err(grpc_unauthenticated_response("invalid bearer token format"));
    }

    let token = token.unwrap().to_string();

    let auth_context = Authentication::validate(&conn, token).await.map_err(|e| match e {
        ValidateError::Revoked => grpc_unauthenticated_response("token has been revoked"),
        ValidateError::Invalid => grpc_unauthenticated_response("invalid or expired token"),
    })?;

    if let Some(business_profile_id) = auth_context.active_business_profile_id {
        let business_profile = BusinessProfileUseCase::get_by_id(&conn, business_profile_id)
            .await
            .ok_or_else(|| grpc_unauthenticated_response("active business profile not found"))?;
        Ok((auth_context.user, Some(business_profile)))
    }else {
        Ok((auth_context.user, None))
    }
}

fn grpc_unauthenticated_response(message: &str) -> Response<TonicBody> {
    let mut response = Response::new(empty_grpc_body());

    *response.status_mut() = StatusCode::OK;

    let headers = response.headers_mut();
    headers.insert("content-type", HeaderValue::from_static("application/grpc"));
    headers.insert("grpc-status", HeaderValue::from_static("16")); // UNAUTHENTICATED
    headers.insert(
        "grpc-message",
        HeaderValue::from_str(message).unwrap_or(HeaderValue::from_static("unauthenticated")),
    );

    response
}

fn empty_grpc_body() -> TonicBody {
    TonicBody::empty()
}
