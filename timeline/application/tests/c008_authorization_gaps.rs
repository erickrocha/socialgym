//! C-008 task 1: one failing-until-fixed test per authorization or integrity gap found while
//! building the authorization matrix (`design.md`). Each test asserts the behavior AFTER the fix
//! (SYS-C008-014..016), so every test is expected to FAIL on the current code and to pass once
//! task 2 lands. Needs a disposable MongoDB (`TEST_MONGO_URL`, database `timeline_test`); the
//! `workout` gRPC calls are answered by an in-process stand-in.
//!
//! Cast: Alice authors posts; Bob is her accepted friend; Carol has no relationship with either.
use application::{
    AppState,
    routes::{
        chat_routes::chat_routes, content_report_routes::report_routes, evolution_checkin_routes::evolution_checkin_routes,
        feed_routes::feed_route,
        post_routes::post_routes,
        workout_session_routes::workout_session_routes,
    },
};
use axum::{Router, body::Body, http::Request};
use business::proto::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use business::proto::proto::friend::friend_service_server::FriendServiceServer;
use business::proto::proto::person::person_service_server::PersonServiceServer;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::Client;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::{Mutex, Once};
use tower::ServiceExt;

const SECRET: &str = "c008-gaps-test-secret";
const INTERNAL_SECRET: &str = "c008-internal-test-secret";

/// The environment (`GRPC_*`, token secret) is process-wide: one test at a time.
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

mod standin;
use standin::Workout;

// ---------------------------------------------------------------- log capture

static LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());
static LOGGER_ONCE: Once = Once::new();

struct Capture;
impl log::Log for Capture {
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }
    fn log(&self, record: &log::Record<'_>) {
        LOGS.lock().unwrap().push(record.args().to_string());
    }
    fn flush(&self) {}
}
static CAPTURE: Capture = Capture;

fn capture_logs() {
    LOGGER_ONCE.call_once(|| {
        let _ = log::set_logger(&CAPTURE);
        log::set_max_level(log::LevelFilter::Trace);
    });
}

// ---------------------------------------------------------------- fixture

struct World {
    app: Router,
    state: AppState,
    alice: Actor,
    bob: Actor,
    carol: Actor,
    _guard: tokio::sync::MutexGuard<'static, ()>,
}

struct Actor {
    uuid: String,
    token: String,
}

fn person(id: i32, name: &str) -> Actor {
    let uuid = format!("c008-{name}-{}", uuid::Uuid::new_v4());
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
    let token = encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(SECRET.as_bytes())).unwrap();
    Actor { uuid, token }
}

async fn world() -> World {
    let guard = ENV_LOCK.lock().await;
    capture_logs();
    let mongo = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(mongo.contains("/timeline_test"), "refusing to run against a non-test database");
    let database = Client::with_uri_str(mongo).await.unwrap().database("timeline_test");

    let (alice, bob, carol) = (person(8001, "alice"), person(8002, "bob"), person(8003, "carol"));
    let friends = HashMap::from([
        (alice.uuid.clone(), vec![bob.uuid.clone()]),
        (bob.uuid.clone(), vec![alice.uuid.clone()]),
    ]);

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let workout = || Workout { friends: friends.clone(), ..Default::default() };
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
        std::env::set_var("GRPC_PORT", port.to_string());
        std::env::set_var("GRPC_USE_TLS", "false");
        std::env::set_var("INTERNAL_SERVICE_SECRET", INTERNAL_SECRET);
    }

    let state = AppState { database: std::sync::Arc::new(database), chat_hub: Default::default() };
    let app = Router::new()
        .nest("/posts", post_routes(state.clone()))
        .nest("/feed", feed_route(state.clone()))
        .nest("/reports", report_routes(state.clone()))
        .nest("/workout-sessions", workout_session_routes(state.clone()))
        .nest("/chat", chat_routes(state.clone()))
        .nest("/check-ins", evolution_checkin_routes(state.clone()))
        .with_state(state.clone());
    World { app, state, alice, bob, carol, _guard: guard }
}

/// (status, parsed body, raw body)
async fn call(app: &Router, method: &str, uri: &str, token: Option<&str>, body: Option<Value>) -> (u16, Value, String) {
    let mut req = Request::builder().method(method).uri(uri);
    if let Some(token) = token {
        req = req.header("authorization", format!("Bearer {token}"));
    }
    let body = match body {
        Some(b) => {
            req = req.header("content-type", "application/json");
            Body::from(b.to_string())
        }
        None => Body::empty(),
    };
    let res = app.clone().oneshot(req.body(body).unwrap()).await.unwrap();
    let status = res.status().as_u16();
    let bytes = axum::body::to_bytes(res.into_body(), usize::MAX).await.unwrap();
    let raw = String::from_utf8_lossy(&bytes).to_string();
    (status, serde_json::from_str(&raw).unwrap_or(Value::Null), raw)
}

/// Alice publishes a post with unmistakable content; returns (post uuid, content).
async fn alice_post(w: &World) -> (String, String) {
    let content = format!("private words of alice {}", uuid::Uuid::new_v4());
    let (status, post, raw) = call(&w.app, "POST", "/posts", Some(&w.alice.token), Some(json!({
        "authorId": 8001, "authorUuid": "ignored", "authorName": "ignored", "content": content
    }))).await;
    assert_eq!(status, 201, "setup: Alice must be able to post: {raw}");
    (post["uuid"].as_str().unwrap().to_string(), content)
}

macro_rules! gap_test {
    ($name:ident, $doc:literal, |$w:ident| $body:block) => {
        #[tokio::test]
        #[ignore = "requires a disposable TEST_MONGO_URL; expected to FAIL until C-008 task 2"]
        async fn $name() {
            let $w = world().await;
            $body
        }
    };
}

// ---------------------------------------------------------------- the gaps

gap_test!(gap_comment_on_a_post_the_caller_cannot_read_is_not_found, "SYS-C008-014", |w| {
    let (post, content) = alice_post(&w).await;
    let (status, _, raw) = call(&w.app, "POST", &format!("/posts/{post}/comments"), Some(&w.carol.token), Some(json!({
        "postUuid": post, "authorUuid": "x", "authorName": "x", "content": "hello"
    }))).await;
    assert_eq!(status, 404, "a stranger must not be able to comment: {raw}");
    assert!(!raw.contains(&content), "the response must not disclose the post content");
});

gap_test!(gap_reaction_on_a_post_the_caller_cannot_read_is_not_found, "SYS-C008-014", |w| {
    let (post, content) = alice_post(&w).await;
    let (status, _, raw) = call(&w.app, "POST", &format!("/posts/{post}/reactions"), Some(&w.carol.token), Some(json!({
        "authorId": "x", "authorName": "x", "reactionType": "like"
    }))).await;
    assert_eq!(status, 404, "a stranger must not be able to react: {raw}");
    assert!(!raw.contains(&content), "the response must not disclose the post content");
});

gap_test!(gap_removing_a_reaction_does_not_return_an_unreadable_post, "SYS-C008-014", |w| {
    let (post, content) = alice_post(&w).await;
    let (status, _, raw) = call(&w.app, "DELETE", &format!("/posts/{post}/reactions"), Some(&w.carol.token), None).await;
    assert_eq!(status, 404, "{raw}");
    assert!(!raw.contains(&content), "DELETE reactions must not be a way to read any post by id");
});

gap_test!(gap_deleting_an_unreadable_post_reveals_nothing, "SYS-C008-014", |w| {
    let (post, _) = alice_post(&w).await;
    let (stranger, _, _) = call(&w.app, "DELETE", &format!("/posts/{post}"), Some(&w.carol.token), None).await;
    assert_eq!(stranger, 404, "a stranger gets not found, not forbidden");
    let (friend, _, _) = call(&w.app, "DELETE", &format!("/posts/{post}"), Some(&w.bob.token), None).await;
    assert_eq!(friend, 403, "a reader who is not the author is forbidden");
});

gap_test!(gap_feed_by_a_person_uuid_is_not_found, "SYS-C008-014", |w| {
    let (_, content) = alice_post(&w).await;
    let (status, _, raw) = call(&w.app, "GET", &format!("/feed/{}", w.alice.uuid), Some(&w.carol.token), None).await;
    assert_eq!(status, 404, "the feed by uuid serves only Business Profiles: {raw}");
    assert!(!raw.contains(&content), "a stranger must not read Alice's posts through the author feed");
});

gap_test!(gap_a_client_supplied_post_id_is_ignored, "SYS-C008-015", |w| {
    let (existing, _) = alice_post(&w).await;
    let (status, post, raw) = call(&w.app, "POST", "/posts", Some(&w.alice.token), Some(json!({
        "uuid": existing, "authorId": 8001, "authorUuid": "x", "authorName": "x", "content": "second post"
    }))).await;
    assert_eq!(status, 201, "a duplicate client id must not fail the create: {raw}");
    assert_ne!(post["uuid"].as_str().unwrap(), existing, "the server generates the id");
});

gap_test!(gap_reaction_type_is_case_insensitive_and_unknown_names_are_rejected, "SYS-C008-015", |w| {
    let (post, _) = alice_post(&w).await;
    let (status, body, raw) = call(&w.app, "POST", &format!("/posts/{post}/reactions"), Some(&w.bob.token), Some(json!({
        "authorId": "x", "authorName": "x", "reactionType": "LOVE"
    }))).await;
    assert_eq!(status, 201, "the web sends capitalized names, so case differences are accepted: {raw}");
    assert_eq!(body["reactions"][0]["reactionType"].as_str().map(str::to_lowercase).as_deref(), Some("love"), "{raw}");
    let (status, _, raw) = call(&w.app, "POST", &format!("/posts/{post}/reactions"), Some(&w.bob.token), Some(json!({
        "authorId": "x", "authorName": "x", "reactionType": "dislike"
    }))).await;
    assert_eq!(status, 400, "an unknown reaction type is rejected, not stored as like: {raw}");
});

gap_test!(gap_a_missing_authorization_header_is_401, "SYS-C008-010", |w| {
    let (status, _, raw) = call(&w.app, "GET", "/feed", None, None).await;
    assert_eq!(status, 401, "missing credentials are 401, not 403: {raw}");
});

gap_test!(gap_reporting_an_unreadable_post_is_not_found, "SYS-C008-007", |w| {
    let (post, _) = alice_post(&w).await;
    let (status, _, raw) = call(&w.app, "POST", "/reports", Some(&w.carol.token), Some(json!({
        "targetType": "post", "targetId": post, "postId": post, "reason": "spam"
    }))).await;
    assert_eq!(status, 404, "a report must not confirm that a hidden post exists: {raw}");
});

gap_test!(gap_another_persons_workout_session_is_not_found, "SYS-C008-008", |w| {
    let (status, session, raw) = call(&w.app, "POST", "/workout-sessions", Some(&w.alice.token), Some(json!({
        "personUuid": "ignored", "duration": 10, "executedSets": [], "totalVolume": 0.0, "totalSets": 0.0,
        "startedAt": "2026-10-06T10:00:00", "completedAt": "2026-10-06T10:10:00"
    }))).await;
    assert_eq!(status, 201, "setup: {raw}");
    let id = session["uuid"].as_str().unwrap();
    let (other, _, _) = call(&w.app, "GET", &format!("/workout-sessions/{id}"), Some(&w.bob.token), None).await;
    let (missing, _, _) = call(&w.app, "GET", "/workout-sessions/does-not-exist", Some(&w.bob.token), None).await;
    assert_eq!(missing, 404);
    assert_eq!(other, 404, "someone else's session answers like a missing one, so existence is not revealed");
});

gap_test!(gap_presence_does_not_report_people_the_caller_cannot_see, "SYS-C008-005", |w| {
    let (_conn, _rx) = w.state.chat_hub.register(&w.carol.uuid);
    let (status, body, raw) = call(&w.app, "GET", &format!("/chat/presence?uuids={}", w.carol.uuid), Some(&w.alice.token), None).await;
    assert_eq!(status, 200, "{raw}");
    assert!(
        body["online"].as_array().is_none_or(|online| online.is_empty()),
        "Alice has no relationship with Carol and must not learn she is online: {raw}"
    );
});

gap_test!(gap_an_invalid_token_is_not_written_to_the_logs, "SYS-C008-015", |w| {
    let marker = format!("leaky-token-{}", uuid::Uuid::new_v4());
    let (status, _, _) = call(&w.app, "GET", "/feed", Some(&marker), None).await;
    assert_eq!(status, 401);
    let leaked = LOGS.lock().unwrap().iter().any(|line| line.contains(&marker));
    assert!(!leaked, "a credential must never reach the logs");
});

gap_test!(gap_a_workout_session_without_dates_is_rejected_not_a_panic, "SYS-C008-015", |w| {
    let (status, _, raw) = call(&w.app, "POST", "/workout-sessions", Some(&w.alice.token), Some(json!({
        "personUuid": "ignored", "duration": 10, "executedSets": [], "totalVolume": 0.0, "totalSets": 0.0
    }))).await;
    assert_eq!(status, 400, "missing startedAt/completedAt must be a validation error, never a panic: {raw}");
});

gap_test!(gap_the_person_data_export_holds_only_the_persons_own_content, "SYS-C008-009", |w| {
    // Bob (a friend) comments on Alice's post; his export must not carry Alice's text.
    let (post, alice_text) = alice_post(&w).await;
    let (status, _, raw) = call(&w.app, "POST", &format!("/posts/{post}/comments"), Some(&w.bob.token), Some(json!({
        "postUuid": post, "authorUuid": "x", "authorName": "x", "content": "bob's own words"
    }))).await;
    assert_eq!(status, 201, "setup: {raw}");

    // The export is served by the internal gRPC service (the REST internal routes are retired);
    // the interceptor that checks the secret has its own tests, so call the handler directly.
    use integration::proto::timeline::{PersonDataRequest, internal_service_server::InternalService};
    let services = integration::service::internal_service::GrpcInternalService::new(w.state.database.clone());
    let export = services
        .export_person_data(tonic::Request::new(PersonDataRequest { person_uuid: w.bob.uuid.clone() }))
        .await
        .expect("export")
        .into_inner()
        .export_json;
    assert!(export.contains("bob's own words"), "the export must hold Bob's own comment");
    assert!(!export.contains(&alice_text), "the export must not hold another person's post text");
});

gap_test!(gap_a_client_supplied_check_in_id_is_ignored, "SYS-C008-015", |w| {
    let body = |uuid: Option<&str>| json!({
        "uuid": uuid, "personUuid": "ignored", "createdAt": "2026-10-06T10:00:00", "note": "weekly", "visibility": "Private"
    });
    let (status, first, raw) = call(&w.app, "POST", "/check-ins", Some(&w.alice.token), Some(body(None))).await;
    assert_eq!(status, 201, "setup: {raw}");
    let existing = first["uuid"].as_str().unwrap().to_string();
    let (status, second, raw) = call(&w.app, "POST", "/check-ins", Some(&w.alice.token), Some(body(Some(&existing)))).await;
    assert_eq!(status, 201, "a duplicate client id must not fail the create: {raw}");
    assert_ne!(second["uuid"].as_str().unwrap(), existing, "the server generates the id");
});

gap_test!(gap_a_client_supplied_workout_session_id_is_ignored, "SYS-C008-015", |w| {
    let body = |uuid: Option<&str>, name: &str| json!({
        "uuid": uuid, "personUuid": "ignored", "workoutName": name, "duration": 10,
        "startedAt": "2026-10-06T10:00:00", "completedAt": "2026-10-06T10:10:00",
        "executedSets": [], "totalVolume": 0.0, "totalSets": 0.0
    });
    let (status, first, raw) = call(&w.app, "POST", "/workout-sessions", Some(&w.alice.token), Some(body(None, "first"))).await;
    assert_eq!(status, 201, "setup: {raw}");
    let existing = first["uuid"].as_str().unwrap().to_string();
    let (status, second, raw) = call(&w.app, "POST", "/workout-sessions", Some(&w.alice.token), Some(body(Some(&existing), "second"))).await;
    assert_eq!(status, 201, "a duplicate client id must not fail the create: {raw}");
    assert_ne!(second["uuid"].as_str().unwrap(), existing, "the server generates the id");
    let (status, kept, raw) = call(&w.app, "GET", &format!("/workout-sessions/{existing}"), Some(&w.alice.token), None).await;
    assert_eq!(status, 200, "{raw}");
    assert_eq!(kept["workoutName"], "first", "the existing session must be unchanged");
});

gap_test!(gap_post_content_is_limited_to_5000_characters_over_rest, "SYS-C008-015", |w| {
    let post = |len: usize| json!({ "authorId": 1, "authorUuid": "x", "authorName": "x", "content": "a".repeat(len) });
    let (status, _, raw) = call(&w.app, "POST", "/posts", Some(&w.alice.token), Some(post(5001))).await;
    assert_eq!(status, 400, "5001 characters must be rejected: {raw}");
    let (status, _, raw) = call(&w.app, "POST", "/posts", Some(&w.alice.token), Some(post(5000))).await;
    assert_eq!(status, 201, "5000 characters must be accepted: {raw}");
});
