//! C-008 task 3: the gRPC foundation (authentication, internal secret, rate limit, failure
//! isolation) exercised through a real in-process server and client. No MongoDB or workout needed.
use application::grpc;
use business::proto::proto::timeline::health_service_client::HealthServiceClient;
use business::proto::proto::timeline::HealthRequest;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use tonic::metadata::MetadataValue;
use tonic::transport::{Channel, Server};
use tonic::{Code, Request};

const SECRET: &str = "c008-foundation-secret";

fn token(secret: &str, expires_in: i64) -> String {
    let claims = Claims {
        sub: "alice@c008.test".into(),
        exp: chrono::Utc::now().timestamp() + expires_in,
        uuid: "user-1".into(),
        name: "Alice".into(),
        person_id: 1,
        person_uuid: "alice-uuid".into(),
        person_object_key: String::new(),
        active_business_profile_id: None,
        active_business_profile_uuid: None,
    };
    encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap()
}

/// A handle that never connects: the foundation tests only use services that touch no data.
async fn idle_state() -> application::AppState {
    let database = mongodb::Client::with_uri_str("mongodb://127.0.0.1:1").await.unwrap().database("idle");
    application::AppState { database: std::sync::Arc::new(database), chat_hub: Default::default() }
}

async fn client() -> HealthServiceClient<Channel> {
    unsafe { std::env::set_var("ACCESS_TOKEN_SECRET", SECRET) };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(
        grpc::router(Server::builder(), idle_state().await)
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    let channel = Channel::from_shared(format!("http://127.0.0.1:{port}")).unwrap().connect().await.unwrap();
    HealthServiceClient::new(channel)
}

async fn check(authorization: Option<&str>) -> Result<String, Code> {
    let mut request = Request::new(HealthRequest {});
    if let Some(value) = authorization {
        request.metadata_mut().insert("authorization", MetadataValue::try_from(value).unwrap());
    }
    client().await.check(request).await.map(|r| r.into_inner().person_uuid).map_err(|s| s.code())
}

#[tokio::test]
async fn a_valid_token_reaches_the_service_as_the_token_owner() {
    let value = format!("Bearer {}", token(SECRET, 3600));
    assert_eq!(check(Some(&value)).await.unwrap(), "alice-uuid");
}

#[tokio::test]
async fn a_missing_malformed_expired_or_tampered_credential_is_unauthenticated() {
    let good = token(SECRET, 3600);
    let cases = [
        None,
        Some("".to_string()),
        Some("Bearer".to_string()),
        Some(format!("Basic {good}")),
        Some(format!("Bearer {good} extra")),
        Some(format!("Bearer {}", token(SECRET, -3600))),
        Some(format!("Bearer {}", token("another-secret", 3600))),
        Some("Bearer not-a-jwt".to_string()),
    ];
    for case in cases {
        assert_eq!(check(case.as_deref()).await, Err(Code::Unauthenticated), "{case:?}");
    }
}

#[test]
fn the_internal_interceptor_accepts_only_the_shared_secret() {
    unsafe { std::env::set_var("INTERNAL_SERVICE_SECRET", "c008-internal") };
    let with = |value: Option<&str>| {
        let mut request = Request::new(());
        if let Some(value) = value {
            request.metadata_mut().insert("x-internal-secret", MetadataValue::try_from(value).unwrap());
        }
        grpc::auth::internal_interceptor(request).map(|_| ()).map_err(|s| s.code())
    };
    assert_eq!(with(None), Err(Code::Unauthenticated));
    assert_eq!(with(Some("wrong")), Err(Code::Unauthenticated));
    assert_eq!(with(Some("")), Err(Code::Unauthenticated));
    assert_eq!(with(Some("c008-internal")), Ok(()));
    // A user token is not a service credential.
    let mut request = Request::new(());
    request.metadata_mut().insert("authorization", MetadataValue::try_from(format!("Bearer {}", token(SECRET, 60))).unwrap());
    assert_eq!(grpc::auth::internal_interceptor(request).map(|_| ()).map_err(|s| s.code()), Err(Code::Unauthenticated));
}

#[test]
fn the_rate_limit_turns_into_resource_exhausted_for_one_address() {
    let limiter = application::authentication::rate_limit::content_limiter();
    let from = |ip: &str| {
        let mut request = Request::new(());
        request.metadata_mut().insert("x-real-ip", MetadataValue::try_from(ip).unwrap());
        grpc::rate_limit::enforce(&limiter, &request).map_err(|s| s.code())
    };
    let outcomes: Vec<_> = (0..61).map(|_| from("203.0.113.7")).collect();
    assert!(outcomes[..60].iter().all(Result::is_ok));
    assert_eq!(outcomes[60], Err(Code::ResourceExhausted));
    assert_eq!(from("203.0.113.8"), Ok(()), "another address has its own budget");
}

#[tokio::test]
async fn a_busy_grpc_port_is_reported_and_does_not_panic() {
    let busy = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    unsafe {
        std::env::set_var("TIMELINE_GRPC_HOST", "127.0.0.1");
        std::env::set_var("TIMELINE_GRPC_PORT", busy.local_addr().unwrap().port().to_string());
        std::env::set_var("TIMELINE_GRPC_TLS", "false");
    }
    assert!(grpc::serve(idle_state().await).await.is_err(), "the caller logs this and keeps the REST server running");
}

/// TC-011 step 3 against the real container: TLS, the published port and token validation.
/// Needs `infra/test` up and `source infra/test/timeline-test-env.sh`.
#[tokio::test]
#[ignore = "requires the infra/test stack"]
async fn the_test_stack_serves_health_over_tls_with_the_same_token_rules() {
    use tonic::transport::{Certificate, ClientTlsConfig};
    let ca = std::fs::read_to_string(std::env::var("GRPC_CERT_PATH").unwrap()).unwrap();
    let channel = Channel::from_static("https://localhost:18092")
        .tls_config(ClientTlsConfig::new().ca_certificate(Certificate::from_pem(ca)).domain_name("localhost"))
        .unwrap()
        .connect()
        .await
        .unwrap();
    // The stack signs with the secret in infra/test/compose.yml (the in-process tests above
    // overwrite ACCESS_TOKEN_SECRET in this process, so it is not read from the environment).
    let secret = "c005-test-secret";
    let call = |authorization: Option<String>| {
        let mut client = HealthServiceClient::new(channel.clone());
        async move {
            let mut request = Request::new(HealthRequest {});
            if let Some(value) = authorization {
                request.metadata_mut().insert("authorization", MetadataValue::try_from(value).unwrap());
            }
            client.check(request).await.map(|r| r.into_inner().person_uuid).map_err(|s| s.code())
        }
    };
    assert_eq!(call(Some(format!("Bearer {}", token(secret, 3600)))).await.unwrap(), "alice-uuid");
    assert_eq!(call(None).await, Err(Code::Unauthenticated));
    assert_eq!(call(Some("Bearer garbage".into())).await, Err(Code::Unauthenticated));
}
