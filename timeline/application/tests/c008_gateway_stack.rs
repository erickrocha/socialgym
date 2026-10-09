//! C-008 checks that need the real `infra/test` stack and its gateway:
//! - TC-011 step 4: a chat stream held idle for 120 s through the gateway still answers a `ping`.
//! - TC-010 step 4 and TC-014 step 8: with the real `workout` container stopped, the timeline
//!   answers `UNAVAILABLE` (and, with it running, an unknown author is `NOT_FOUND`).
//! - TC-009 step 4: the retired REST `/internal/persons/...` routes answer `404` through the gateway.
//!
//! Needs the stack up (`infra/test/start.sh`), the gateway (`infra/test/e2e-gateway.sh up`) and
//! `source infra/test/timeline-test-env.sh` (secret and CA). Person 1 comes from `seed.sql`.
//! Run with `--test-threads=1`: one test stops `workout`.
use domain::access_token::Claims;
use integration::proto::timeline::chat_service_client::ChatServiceClient;
use integration::proto::timeline::client_frame::Frame;
use integration::proto::timeline::server_frame::Event;
use integration::proto::timeline::*;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::Request;
use tonic::metadata::MetadataValue;
use tonic::transport::{Certificate, Channel, ClientTlsConfig};

const IDLE: Duration = Duration::from_secs(120);
use integration::proto::timeline::feed_service_client::FeedServiceClient;
use integration::proto::timeline::post_service_client::PostServiceClient;
use tonic::Code;

/// The gateway channel and a token for person 1 (`workout` also reads `iat` and `jti`).
async fn gateway() -> (Channel, String) {
    let secret =
        std::env::var("ACCESS_TOKEN_SECRET").expect("source infra/test/timeline-test-env.sh");
    let ca =
        std::fs::read(std::env::var("GRPC_CERT_PATH").expect("GRPC_CERT_PATH")).expect("CA file");
    let url = std::env::var("GATEWAY_GRPC_URL").unwrap_or_else(|_| "https://localhost:8443".into());
    let claims = Claims {
        sub: "c006-sender@example.test".into(),
        exp: chrono::Utc::now().timestamp() + 600,
        uuid: "10000000-0000-0000-0000-000000000061".into(),
        name: "Sender Person".into(),
        person_id: 1,
        person_uuid: "00000000-0000-0000-0000-000000000061".into(),
        person_object_key: String::new(),
        active_business_profile_id: None,
        active_business_profile_uuid: None,
    };
    let mut claims = serde_json::to_value(&claims).unwrap();
    claims["iat"] = chrono::Utc::now().timestamp().into();
    claims["jti"] = uuid::Uuid::new_v4().to_string().into();
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    let tls = ClientTlsConfig::new()
        .ca_certificate(Certificate::from_pem(ca))
        .domain_name("localhost");
    let channel = Channel::from_shared(url)
        .unwrap()
        .tls_config(tls)
        .unwrap()
        .connect()
        .await
        .expect("gateway");
    (channel, token)
}

fn authorized<T>(message: T, token: &str) -> Request<T> {
    let mut request = Request::new(message);
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {token}")).unwrap(),
    );
    request
}

const WORKOUT: &str = "test-integration-1";

/// Stops the real `workout` gRPC container and starts it again when dropped, even after a failed assertion.
struct WorkoutDown;

impl WorkoutDown {
    fn stop() -> Self {
        assert!(
            std::process::Command::new("docker")
                .args(["stop", WORKOUT])
                .status()
                .unwrap()
                .success()
        );
        WorkoutDown
    }
}

impl Drop for WorkoutDown {
    fn drop(&mut self) {
        let _ = std::process::Command::new("docker")
            .args(["start", WORKOUT])
            .status();
        // Leave the stack as found: wait until the container answers its health check, so the next test
        // does not connect to a server that is still starting.
        for _ in 0..60 {
            let health = std::process::Command::new("docker")
                .args(["inspect", "--format", "{{.State.Health.Status}}", WORKOUT])
                .output()
                .map(|out| String::from_utf8_lossy(&out.stdout).trim().to_string())
                .unwrap_or_default();
            if health == "healthy" {
                return;
            }
            std::thread::sleep(std::time::Duration::from_secs(2));
        }
    }
}

#[tokio::test]
#[ignore = "requires the infra/test stack and its gateway; holds a stream open for 120 s"]
async fn a_stream_idle_for_two_minutes_survives_the_gateway() {
    let (channel, token) = gateway().await;

    let (send, frames) = mpsc::channel(4);
    let mut request = Request::new(ReceiverStream::new(frames));
    request.metadata_mut().insert(
        "authorization",
        MetadataValue::try_from(format!("Bearer {token}")).unwrap(),
    );
    let mut events = ChatServiceClient::new(channel)
        .open_stream(request)
        .await
        .expect("open stream")
        .into_inner();

    // Idle: no frame goes either way. A gateway that cuts idle streams ends it here.
    tokio::time::sleep(IDLE).await;

    send.send(ClientFrame {
        frame: Some(Frame::Ping(PingFrame {})),
    })
    .await
    .expect("stream still writable");
    let event = tokio::time::timeout(Duration::from_secs(5), events.message())
        .await
        .expect("no answer within 5 s")
        .expect("stream failed")
        .expect("stream ended")
        .event;
    assert!(
        matches!(event, Some(Event::Pong(_))),
        "expected pong, got {event:?}"
    );
}

#[tokio::test]
#[ignore = "requires the infra/test stack and its gateway; stops and restarts the workout container"]
async fn a_stopped_workout_is_unavailable_and_never_a_missing_business_profile() {
    let (channel, token) = gateway().await;
    let by_author = |uuid: &str| {
        authorized(
            integration::proto::timeline::GetFeedByAuthorRequest {
                author_uuid: uuid.into(),
                page: 0,
            },
            &token,
        )
    };

    // Running: a uuid that is no Business Profile is NOT_FOUND (the real workout's miss status).
    let unknown = uuid::Uuid::new_v4().to_string();
    let status = FeedServiceClient::new(channel.clone())
        .get_feed_by_author(by_author(&unknown))
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::NotFound, "{status:?}");

    let _down = WorkoutDown::stop();
    // Stopped: the same question is UNAVAILABLE, for a fresh uuid (no cached answer) ...
    let fresh = uuid::Uuid::new_v4().to_string();
    let status = FeedServiceClient::new(channel.clone())
        .get_feed_by_author(by_author(&fresh))
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::Unavailable, "{status:?}");
    // ... and so is an operation that needs the consent check.
    let create = authorized(
        integration::proto::timeline::CreatePostRequest {
            content: "x".into(),
            ..Default::default()
        },
        &token,
    );
    let status = PostServiceClient::new(channel)
        .create_post(create)
        .await
        .unwrap_err();
    assert_eq!(status.code(), Code::Unavailable, "{status:?}");
}

#[tokio::test]
#[ignore = "requires the infra/test stack and its gateway"]
async fn the_retired_internal_rest_routes_answer_404() {
    let ca =
        std::fs::read(std::env::var("GRPC_CERT_PATH").expect("GRPC_CERT_PATH")).expect("CA file");
    let secret =
        std::env::var("INTERNAL_SERVICE_SECRET").expect("source infra/test/timeline-test-env.sh");
    let client = reqwest::Client::builder()
        .add_root_certificate(reqwest::Certificate::from_pem(&ca).unwrap())
        .build()
        .unwrap();
    let base =
        std::env::var("GATEWAY_REST_URL").unwrap_or_else(|_| "https://localhost:8443".into());
    let person = "00000000-0000-0000-0000-000000000061";

    // if the host is wrong, so first prove the gateway reaches the timeline with a live REST route.
    let live = client
        .get(format!("{base}/timeline/api/feed"))
        .send()
        .await
        .expect("gateway");
    assert_eq!(
        live.status().as_u16(),
        401,
        "the live feed route must answer 401 without a token"
    );
    for (method, path) in [
        (
            reqwest::Method::DELETE,
            format!("/timeline/api/internal/persons/{person}"),
        ),
        (
            reqwest::Method::GET,
            format!("/timeline/api/internal/persons/{person}/export"),
        ),
    ] {
        let status = client
            .request(method.clone(), format!("{base}{path}"))
            .header("x-internal-secret", &secret)
            .send()
            .await
            .expect("gateway")
            .status();
        assert_eq!(status.as_u16(), 404, "{method} {path} must no longer exist");
    }
}
