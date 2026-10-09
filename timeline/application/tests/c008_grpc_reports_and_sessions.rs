//! C-008 task 6: reports, moderation and workout sessions over gRPC (TC-007, TC-008), with the
//! REST answer checked next to the gRPC one where the packet asks for it.
//! Needs `TEST_MONGO_URL` (database `timeline_test`).
//!
//! Cast: Alice has a post and is Bob's friend; Carol is a stranger; Mod holds the moderator role.
mod standin;

use application::{AppState, routes::workout_session_routes::workout_session_routes};
use axum::{Router, body::Body, http::Request as HttpRequest};
use business::proto::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use business::proto::proto::friend::friend_service_server::FriendServiceServer;
use business::proto::proto::person::person_service_server::PersonServiceServer;
use domain::access_token::Claims;
use integration::proto::timeline::content_report_service_client::ContentReportServiceClient;
use integration::proto::timeline::post_service_client::PostServiceClient;
use integration::proto::timeline::workout_session_service_client::WorkoutSessionServiceClient;
use integration::proto::timeline::*;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::Client;
use standin::Workout;
use std::collections::HashMap;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Code, Request};
use tower::ServiceExt;

const SECRET: &str = "c008-reports-test-secret";
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

struct Actor {
    uuid: String,
    token: String,
}

fn person(id: i32, name: &str) -> Actor {
    let uuid = format!("c008r-{name}-{}", uuid::Uuid::new_v4());
    let claims = Claims {
        sub: format!("{uuid}@c008.test"),
        exp: chrono::Utc::now().timestamp() + 3600,
        uuid: format!("user-{uuid}"),
        name: name.to_string(),
        person_id: id,
        person_uuid: uuid.clone(),
        person_object_key: String::new(),
        active_business_profile_id: None,
        active_business_profile_uuid: None,
    };
    let token = encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(SECRET.as_bytes()),
    )
    .unwrap();
    Actor { uuid, token }
}

struct World {
    channel: Channel,
    rest: Router,
    alice: Actor,
    bob: Actor,
    carol: Actor,
    moderator: Actor,
    _guard: tokio::sync::MutexGuard<'static, ()>,
}

async fn world() -> World {
    let guard = ENV_LOCK.lock().await;
    let mongo = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(
        mongo.contains("/timeline_test"),
        "refusing to run against a non-test database"
    );
    let database = std::sync::Arc::new(
        Client::with_uri_str(mongo)
            .await
            .unwrap()
            .database("timeline_test"),
    );

    let (alice, bob, carol, moderator) = (
        person(8301, "alice"),
        person(8302, "bob"),
        person(8303, "carol"),
        person(8304, "mod"),
    );
    let friends = HashMap::from([
        (alice.uuid.clone(), vec![bob.uuid.clone()]),
        (bob.uuid.clone(), vec![alice.uuid.clone()]),
    ]);
    let workout = || Workout {
        friends: friends.clone(),
        moderator_tokens: vec![moderator.token.clone()],
        ..Default::default()
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let workout_port = listener.local_addr().unwrap().port();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(PersonServiceServer::new(workout()))
            .add_service(FriendServiceServer::new(workout()))
            .add_service(BusinessProfileServiceServer::new(workout()))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("GRPC_PROTOCOL", "http");
        std::env::set_var("GRPC_HOST", "127.0.0.1");
        std::env::set_var("GRPC_PORT", workout_port.to_string());
        std::env::set_var("GRPC_USE_TLS", "false");
    }
    let state = AppState {
        database,
        chat_hub: Default::default(),
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(
        integration::router(
            tonic::transport::Server::builder(),
            state.database.clone(),
            state.chat_hub.clone(),
        )
        .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    let channel = Channel::from_shared(format!("http://127.0.0.1:{port}"))
        .unwrap()
        .connect()
        .await
        .unwrap();
    let rest = Router::new()
        .nest("/workout-sessions", workout_session_routes(state.clone()))
        .with_state(state);
    World {
        channel,
        rest,
        alice,
        bob,
        carol,
        moderator,
        _guard: guard,
    }
}

impl World {
    fn rpc<T>(&self, message: T, actor: &Actor) -> Request<T> {
        let mut request = Request::new(message);
        request.metadata_mut().insert(
            "authorization",
            MetadataValue::try_from(format!("Bearer {}", actor.token)).unwrap(),
        );
        let ip = format!(
            "203.0.113.{}",
            (uuid::Uuid::new_v4().as_u128() % 250) as u8 + 1
        );
        request
            .metadata_mut()
            .insert("x-real-ip", MetadataValue::try_from(ip).unwrap());
        request
    }
    fn reports(&self) -> ContentReportServiceClient<Channel> {
        ContentReportServiceClient::new(self.channel.clone())
    }
    fn sessions(&self) -> WorkoutSessionServiceClient<Channel> {
        WorkoutSessionServiceClient::new(self.channel.clone())
    }

    async fn alice_post(&self) -> String {
        PostServiceClient::new(self.channel.clone())
            .create_post(self.rpc(
                CreatePostRequest {
                    content: "reportable".into(),
                    ..Default::default()
                },
                &self.alice,
            ))
            .await
            .unwrap()
            .into_inner()
            .uuid
            .unwrap()
    }

    async fn rest_status(&self, uri: &str, actor: &Actor) -> u16 {
        let request = HttpRequest::builder()
            .uri(uri)
            .header("authorization", format!("Bearer {}", actor.token))
            .body(Body::empty())
            .unwrap();
        self.rest
            .clone()
            .oneshot(request)
            .await
            .unwrap()
            .status()
            .as_u16()
    }
}

fn code<T>(result: &Result<tonic::Response<T>, tonic::Status>) -> Code {
    result.as_ref().err().map(|s| s.code()).unwrap_or(Code::Ok)
}

fn report_of(post: &str, reason: &str) -> CreateReportRequest {
    CreateReportRequest {
        target_type: "post".into(),
        target_id: post.into(),
        post_id: post.into(),
        reason: reason.into(),
        details: None,
    }
}

// ---------------------------------------------------------------- TC-007

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn reports_need_a_readable_post_and_moderation_needs_the_role() {
    let w = world().await;
    let post = w.alice_post().await;

    let created = w
        .reports()
        .create_report(w.rpc(report_of(&post, "spam"), &w.bob))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(created.reporter_person_uuid, w.bob.uuid);
    assert_eq!(created.status, "open");

    assert_eq!(
        code(
            &w.reports()
                .list_reports(w.rpc(ListReportsRequest::default(), &w.bob))
                .await
        ),
        Code::PermissionDenied
    );
    let denied = DecideReportRequest {
        report_id: created.uuid.clone(),
        decision: "dismissed".into(),
        reason: "no".into(),
    };
    assert_eq!(
        code(
            &w.reports()
                .decide_report(w.rpc(denied.clone(), &w.bob))
                .await
        ),
        Code::PermissionDenied
    );

    let listed = w
        .reports()
        .list_reports(w.rpc(
            ListReportsRequest {
                status: Some("open".into()),
            },
            &w.moderator,
        ))
        .await
        .unwrap()
        .into_inner()
        .reports;
    assert!(listed.iter().any(|r| r.uuid == created.uuid));
    let decided = w
        .reports()
        .decide_report(w.rpc(denied, &w.moderator))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        (
            decided.status.as_str(),
            decided.assigned_moderator_uuid.as_deref()
        ),
        ("resolved", Some(w.moderator.uuid.as_str()))
    );
    let unknown = DecideReportRequest {
        report_id: "no-such-report".into(),
        decision: "dismissed".into(),
        reason: "x".into(),
    };
    assert_eq!(
        code(
            &w.reports()
                .decide_report(w.rpc(unknown, &w.moderator))
                .await
        ),
        Code::NotFound
    );

    // A stranger cannot tell an unreadable post from a missing one, and nothing is stored.
    let before = w
        .reports()
        .list_reports(w.rpc(ListReportsRequest::default(), &w.moderator))
        .await
        .unwrap()
        .into_inner()
        .reports
        .len();
    let unreadable = w
        .reports()
        .create_report(w.rpc(report_of(&post, "spam"), &w.carol))
        .await
        .unwrap_err();
    let missing = w
        .reports()
        .create_report(w.rpc(report_of("no-such-post", "spam"), &w.carol))
        .await
        .unwrap_err();
    assert_eq!(
        (unreadable.code(), missing.code()),
        (Code::NotFound, Code::NotFound)
    );
    assert_eq!(unreadable.message(), missing.message(), "same message");
    let after = w
        .reports()
        .list_reports(w.rpc(ListReportsRequest::default(), &w.moderator))
        .await
        .unwrap()
        .into_inner()
        .reports
        .len();
    assert_eq!(before, after);
}

// ---------------------------------------------------------------- TC-008

fn session(started: &str) -> CreateWorkoutSessionRequest {
    CreateWorkoutSessionRequest {
        session: Some(WorkoutSession {
            uuid: Some("client-chosen-id".into()),
            person_uuid: "someone-else".into(),
            workout_name: Some("Leg day".into()),
            duration: 45,
            started_at: Some(started.into()),
            day_of_week: Some("MONDAY".into()),
            completed_at: Some(started.into()),
            executed_sets: vec![ExecutedSet {
                exercise_name: Some("Squat".into()),
                owner_id: 1,
                owner_name: Some("Alice".into()),
                set_number: 1,
                reps_or_duration: 10,
                weight: 80.0,
                started_at: Some(started.into()),
                completed_at: Some(started.into()),
                ..Default::default()
            }],
            total_volume: 800.0,
            total_sets: 1.0,
        }),
    }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn workout_sessions_belong_to_their_owner_and_hide_from_everyone_else() {
    let w = world().await;
    let now = chrono::Utc::now().naive_utc();
    let stamp = |d: chrono::NaiveDateTime| {
        serde_json::to_value(d)
            .unwrap()
            .as_str()
            .unwrap()
            .to_string()
    };

    let created = w
        .sessions()
        .create_workout_session(w.rpc(session(&stamp(now)), &w.alice))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(
        created.person_uuid, w.alice.uuid,
        "the owner is the caller, not the body"
    );
    let id = created.uuid.clone().unwrap();
    assert_ne!(id, "client-chosen-id", "ids are generated by the server");

    let get = |actor: &Actor, id: &str| {
        w.rpc(
            GetWorkoutSessionRequest {
                session_uuid: id.into(),
            },
            actor,
        )
    };
    assert_eq!(
        w.sessions()
            .get_workout_session(get(&w.alice, &id))
            .await
            .unwrap()
            .into_inner()
            .uuid,
        created.uuid
    );
    let others = w
        .sessions()
        .get_workout_session(get(&w.bob, &id))
        .await
        .unwrap_err();
    let missing = w
        .sessions()
        .get_workout_session(get(&w.bob, "no-such-session"))
        .await
        .unwrap_err();
    assert_eq!(
        (others.code(), missing.code()),
        (Code::NotFound, Code::NotFound)
    );
    assert_eq!(
        others.message(),
        missing.message(),
        "existence is not revealed"
    );
    assert_eq!(
        w.rest_status(&format!("/workout-sessions/{id}"), &w.bob)
            .await,
        404,
        "REST answers the same"
    );

    let window = ListWorkoutSessionsRequest {
        start_date: Some(stamp(now - chrono::Duration::days(1))),
        end_date: Some(stamp(now + chrono::Duration::days(1))),
    };
    let inside = w
        .sessions()
        .list_workout_sessions(w.rpc(window, &w.alice))
        .await
        .unwrap()
        .into_inner()
        .sessions;
    assert!(inside.iter().any(|s| s.uuid == created.uuid));
    let long_ago = ListWorkoutSessionsRequest {
        start_date: Some(stamp(now - chrono::Duration::days(30))),
        end_date: Some(stamp(now - chrono::Duration::days(20))),
    };
    assert!(
        w.sessions()
            .list_workout_sessions(w.rpc(long_ago, &w.alice))
            .await
            .unwrap()
            .into_inner()
            .sessions
            .is_empty()
    );
    let bobs = w
        .sessions()
        .list_workout_sessions(w.rpc(ListWorkoutSessionsRequest::default(), &w.bob))
        .await
        .unwrap()
        .into_inner()
        .sessions;
    assert!(
        bobs.iter().all(|s| s.person_uuid == w.bob.uuid),
        "listing never reaches another person's sessions"
    );

    let mut incomplete = session(&stamp(now));
    incomplete.session.as_mut().unwrap().started_at = None;
    assert_eq!(
        code(
            &w.sessions()
                .create_workout_session(w.rpc(incomplete, &w.alice))
                .await
        ),
        Code::InvalidArgument
    );
}
