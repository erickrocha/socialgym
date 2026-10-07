//! C-008 task 4: the gRPC twins of the post, feed, notification, check-in and push-device
//! surfaces, exercised through a real in-process server against MongoDB. The `workout` calls are
//! answered by the shared stand-in. Needs `TEST_MONGO_URL` (database `timeline_test`).
//!
//! Cast: Alice authors; Bob is her accepted friend; Carol has no relationship with either.
mod standin;

use application::{AppState, grpc, routes::feed_routes::feed_route};
use axum::{Router, body::Body, http::Request};
use business::proto::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use business::proto::proto::friend::friend_service_server::FriendServiceServer;
use business::proto::proto::person::person_service_server::PersonServiceServer;
use business::proto::proto::timeline::evolution_check_in_service_client::EvolutionCheckInServiceClient;
use business::proto::proto::timeline::feed_service_client::FeedServiceClient;
use business::proto::proto::timeline::notification_service_client::NotificationServiceClient;
use business::proto::proto::timeline::post_service_client::PostServiceClient;
use business::proto::proto::timeline::push_device_service_client::PushDeviceServiceClient;
use business::proto::proto::timeline::*;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::Client;
use serde_json::Value;
use standin::Workout;
use std::collections::HashMap;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Code, Request as Rpc};
use tower::ServiceExt;

const SECRET: &str = "c008-services-test-secret";

/// The environment (`GRPC_*`, token secret) is process-wide: one test at a time.
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

struct Actor {
    uuid: String,
    token: String,
}

fn person(id: i32, name: &str) -> Actor {
    let uuid = format!("c008s-{name}-{}", uuid::Uuid::new_v4());
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

struct World {
    channel: Channel,
    rest: Router,
    alice: Actor,
    bob: Actor,
    carol: Actor,
    /// Address the rate limiter sees; unique per test so budgets do not mix.
    ip: String,
    _guard: tokio::sync::MutexGuard<'static, ()>,
}

async fn world() -> World {
    let guard = ENV_LOCK.lock().await;
    let mongo = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(mongo.contains("/timeline_test"), "refusing to run against a non-test database");
    let database = std::sync::Arc::new(Client::with_uri_str(mongo).await.unwrap().database("timeline_test"));

    let (alice, bob, carol) = (person(8101, "alice"), person(8102, "bob"), person(8103, "carol"));
    let friends = HashMap::from([
        (alice.uuid.clone(), vec![bob.uuid.clone()]),
        (bob.uuid.clone(), vec![alice.uuid.clone()]),
    ]);
    let workout = || Workout { friends: friends.clone(), ..Default::default() };
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

    let state = AppState { database, chat_hub: Default::default() };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(
        grpc::router(tonic::transport::Server::builder(), state.clone())
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    let channel = Channel::from_shared(format!("http://127.0.0.1:{port}")).unwrap().connect().await.unwrap();

    let rest = Router::new().nest("/feed", feed_route(state.clone())).with_state(state);
    let ip = format!("198.51.100.{}", rand_octet());
    World { channel, rest, alice, bob, carol, ip, _guard: guard }
}

fn rand_octet() -> u8 {
    (uuid::Uuid::new_v4().as_u128() % 250) as u8 + 1
}

impl World {
    /// A request as `actor` (None: no credential) from this test's address.
    fn rpc<T>(&self, message: T, actor: Option<&Actor>) -> Rpc<T> {
        let mut request = Rpc::new(message);
        if let Some(actor) = actor {
            request.metadata_mut().insert("authorization", MetadataValue::try_from(format!("Bearer {}", actor.token)).unwrap());
        }
        request.metadata_mut().insert("x-real-ip", MetadataValue::try_from(self.ip.as_str()).unwrap());
        request
    }
    fn posts(&self) -> PostServiceClient<Channel> { PostServiceClient::new(self.channel.clone()) }
    fn feed(&self) -> FeedServiceClient<Channel> { FeedServiceClient::new(self.channel.clone()) }
    fn notifications(&self) -> NotificationServiceClient<Channel> { NotificationServiceClient::new(self.channel.clone()) }
    fn check_ins(&self) -> EvolutionCheckInServiceClient<Channel> { EvolutionCheckInServiceClient::new(self.channel.clone()) }
    fn devices(&self) -> PushDeviceServiceClient<Channel> { PushDeviceServiceClient::new(self.channel.clone()) }

    async fn publish(&self, author: &Actor, content: &str) -> Post {
        self.posts()
            .create_post(self.rpc(CreatePostRequest { content: content.into(), ..Default::default() }, Some(author)))
            .await
            .expect("setup: create post")
            .into_inner()
    }

    async fn rest_get(&self, uri: &str, actor: &Actor) -> (u16, Value) {
        let request = Request::builder().uri(uri).header("authorization", format!("Bearer {}", actor.token)).body(Body::empty()).unwrap();
        let response = self.rest.clone().oneshot(request).await.unwrap();
        let status = response.status().as_u16();
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (status, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
    }
}

fn code<T>(result: Result<tonic::Response<T>, tonic::Status>) -> Code {
    result.err().map(|s| s.code()).unwrap_or(Code::Ok)
}

// ---------------------------------------------------------------- TC-001 posts

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn posts_follow_the_audience_rules_over_grpc() {
    let w = world().await;
    let text = format!("alice says {}", uuid::Uuid::new_v4());
    let post = w
        .posts()
        .create_post(w.rpc(CreatePostRequest { content: text.clone(), ..Default::default() }, Some(&w.alice)))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(post.author_uuid, w.alice.uuid, "the author is the person in the token");
    let id = post.uuid.clone().unwrap();

    // Media needs the third-party consent flag.
    let media = vec![Media { uuid: None, url: "u".into(), media_type: "IMAGE".into(), object_key: "k".into() }];
    let refused = w.posts().create_post(w.rpc(CreatePostRequest { content: "m".into(), media: media.clone(), ..Default::default() }, Some(&w.alice))).await;
    assert_eq!(code(refused), Code::InvalidArgument);

    // A friend can comment and react (any case); an unknown type is rejected.
    let commented = w
        .posts()
        .add_comment(w.rpc(AddCommentRequest { post_uuid: id.clone(), content: "nice".into(), ..Default::default() }, Some(&w.bob)))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(commented.comments.len(), 1);
    assert_eq!(commented.comments[0].author_uuid, w.bob.uuid);
    let reacted = w.posts().add_reaction(w.rpc(AddReactionRequest { post_uuid: id.clone(), reaction_type: "LOVE".into() }, Some(&w.bob))).await.unwrap().into_inner();
    assert_eq!(reacted.reactions[0].reaction_type.to_lowercase(), "love");
    let unknown = w.posts().add_reaction(w.rpc(AddReactionRequest { post_uuid: id.clone(), reaction_type: "dislike".into() }, Some(&w.bob))).await;
    assert_eq!(code(unknown), Code::InvalidArgument);

    // A stranger learns nothing: not found, and nothing of the post in the message.
    let stranger = w.posts().add_comment(w.rpc(AddCommentRequest { post_uuid: id.clone(), content: "hi".into(), ..Default::default() }, Some(&w.carol))).await.unwrap_err();
    assert_eq!(stranger.code(), Code::NotFound);
    assert!(!stranger.message().contains(&text));
    assert_eq!(code(w.posts().remove_reaction(w.rpc(RemoveReactionRequest { post_uuid: id.clone() }, Some(&w.carol))).await), Code::NotFound);
    assert_eq!(code(w.posts().delete_post(w.rpc(DeletePostRequest { post_uuid: id.clone() }, Some(&w.carol))).await), Code::NotFound);

    // A reader who is not the author may not delete; the author may, once.
    assert_eq!(code(w.posts().delete_post(w.rpc(DeletePostRequest { post_uuid: id.clone() }, Some(&w.bob))).await), Code::PermissionDenied);
    assert_eq!(code(w.posts().remove_reaction(w.rpc(RemoveReactionRequest { post_uuid: id.clone() }, Some(&w.bob))).await), Code::Ok);
    assert_eq!(code(w.posts().delete_post(w.rpc(DeletePostRequest { post_uuid: id.clone() }, Some(&w.alice))).await), Code::Ok);
    assert_eq!(code(w.posts().delete_post(w.rpc(DeletePostRequest { post_uuid: id }, Some(&w.alice))).await), Code::NotFound);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn the_post_rate_limit_is_resource_exhausted_after_the_rest_budget() {
    let w = world().await;
    let mut last = Code::Ok;
    for _ in 0..61 {
        last = code(w.posts().create_post(w.rpc(CreatePostRequest { content: "spam".into(), ..Default::default() }, Some(&w.alice))).await);
    }
    assert_eq!(last, Code::ResourceExhausted, "the 61st post in a minute from one address is refused");
}

// ---------------------------------------------------------------- TC-001 feed

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn the_feed_matches_rest_and_only_business_profiles_are_listable_by_author() {
    let w = world().await;
    let post = w.publish(&w.alice, &format!("feed words {}", uuid::Uuid::new_v4())).await;
    let id = post.uuid.unwrap();

    let ids = |posts: Vec<Post>| posts.into_iter().filter_map(|p| p.uuid).collect::<Vec<_>>();
    let bob_feed = ids(w.feed().get_feed(w.rpc(GetFeedRequest { page: 0 }, Some(&w.bob))).await.unwrap().into_inner().posts);
    assert!(bob_feed.contains(&id), "a friend sees the post");
    let carol_feed = ids(w.feed().get_feed(w.rpc(GetFeedRequest { page: 0 }, Some(&w.carol))).await.unwrap().into_inner().posts);
    assert!(!carol_feed.contains(&id), "a stranger does not");

    let (status, rest) = w.rest_get("/feed?page=0", &w.bob).await;
    assert_eq!(status, 200);
    let rest_ids: Vec<String> = rest.as_array().unwrap().iter().map(|p| p["uuid"].as_str().unwrap().to_string()).collect();
    assert_eq!(bob_feed, rest_ids, "gRPC and REST return the same page");

    let by_person = w.feed().get_feed_by_author(w.rpc(GetFeedByAuthorRequest { author_uuid: w.alice.uuid.clone(), page: 0 }, Some(&w.bob))).await;
    assert_eq!(code(by_person), Code::NotFound, "a person's posts are not listable by uuid");
}

// ---------------------------------------------------------------- TC-002 notifications

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn notifications_belong_to_the_person_in_the_token() {
    let w = world().await;
    let post = w.publish(&w.alice, "notify me").await;
    let id = post.uuid.unwrap();
    w.posts().add_comment(w.rpc(AddCommentRequest { post_uuid: id, content: "hello".into(), ..Default::default() }, Some(&w.bob))).await.unwrap();

    let mine = w.notifications().list_notifications(w.rpc(ListNotificationsRequest { unread_only: true, limit: 0 }, Some(&w.alice))).await.unwrap().into_inner().notifications;
    assert_eq!(mine.len(), 1);
    assert_eq!(mine[0].recipient_person_uuid, w.alice.uuid);
    let key = mine[0].uuid.clone().unwrap();

    let others = w.notifications().list_notifications(w.rpc(ListNotificationsRequest { unread_only: false, limit: 100 }, Some(&w.carol))).await.unwrap().into_inner().notifications;
    assert!(others.is_empty(), "another person sees none of Alice's notifications");
    let stolen = w.notifications().mark_notification_read(w.rpc(MarkNotificationReadRequest { idempotency_key: key.clone() }, Some(&w.carol))).await;
    assert_eq!(code(stolen), Code::InvalidArgument, "a key that is not the caller's is refused, as REST's 400");

    assert!(w.notifications().mark_notification_read(w.rpc(MarkNotificationReadRequest { idempotency_key: key }, Some(&w.alice))).await.unwrap().into_inner().read);
    let unread = w.notifications().list_notifications(w.rpc(ListNotificationsRequest { unread_only: true, limit: 0 }, Some(&w.alice))).await.unwrap().into_inner().notifications;
    assert!(unread.is_empty());
}

// ---------------------------------------------------------------- TC-003 check-ins

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn check_ins_are_created_for_the_caller_and_listed_by_range() {
    let w = world().await;
    let now = chrono::Utc::now().naive_utc();
    let stamp = |d: chrono::NaiveDateTime| serde_json::to_value(d).unwrap().as_str().unwrap().to_string();
    let request = AddEvolutionCheckInRequest {
        created_at: stamp(now),
        note: Some("weekly".into()),
        visibility: "Private".into(),
        composition: Some(BodyComposition { uuid: None, weight: 80.5, body_fat_pct: 14.0, muscle_mass_pct: 45.0, visceral_fat: 7.0 }),
        circumferences: None,
    };
    let created = w.check_ins().add_evolution_check_in(w.rpc(request, Some(&w.alice))).await.unwrap().into_inner();
    assert_eq!(created.person_uuid, w.alice.uuid);
    assert!(created.uuid.is_some());
    assert_eq!(created.composition.unwrap().weight, 80.5);

    let range = ListEvolutionCheckInsRequest { start_date: Some(stamp(now - chrono::Duration::days(1))), end_date: Some(stamp(now + chrono::Duration::days(1))) };
    let mine = w.check_ins().list_evolution_check_ins(w.rpc(range.clone(), Some(&w.alice))).await.unwrap().into_inner().check_ins;
    assert!(mine.iter().any(|c| c.uuid == created.uuid));
    let theirs = w.check_ins().list_evolution_check_ins(w.rpc(range, Some(&w.bob))).await.unwrap().into_inner().check_ins;
    assert!(theirs.iter().all(|c| c.person_uuid == w.bob.uuid), "listing never reaches another person's check-ins");

    let bad = AddEvolutionCheckInRequest { created_at: "not a date".into(), visibility: "Private".into(), ..Default::default() };
    assert_eq!(code(w.check_ins().add_evolution_check_in(w.rpc(bad, Some(&w.alice))).await), Code::InvalidArgument);
}

// ---------------------------------------------------------------- TC-004 push devices

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn push_devices_are_validated_owned_and_never_echo_the_token() {
    let w = world().await;
    let device = uuid::Uuid::new_v4().to_string();
    let register = |platform: &str, token: &str| RegisterPushDeviceRequest { device_uuid: device.clone(), platform: platform.into(), registration_token: token.into() };

    assert_eq!(code(w.devices().register_push_device(w.rpc(register("windows", "tok"), Some(&w.alice))).await), Code::InvalidArgument);
    assert_eq!(code(w.devices().register_push_device(w.rpc(register("android", "  "), Some(&w.alice))).await), Code::InvalidArgument);
    let bad_id = RegisterPushDeviceRequest { device_uuid: "not-a-uuid".into(), platform: "android".into(), registration_token: "t".into() };
    assert_eq!(code(w.devices().register_push_device(w.rpc(bad_id, Some(&w.alice))).await), Code::InvalidArgument);

    let unique_token = format!("fcm-{}", uuid::Uuid::new_v4());
    let response = w.devices().register_push_device(w.rpc(register("android", &unique_token), Some(&w.alice))).await.unwrap();
    assert!(!format!("{:?}", response.get_ref()).contains(&unique_token), "the token is never returned");

    let other = w.devices().remove_push_device(w.rpc(RemovePushDeviceRequest { device_uuid: device.clone() }, Some(&w.bob))).await;
    assert_eq!(code(other), Code::NotFound, "only the owner can remove a device");
    assert_eq!(code(w.devices().remove_push_device(w.rpc(RemovePushDeviceRequest { device_uuid: device.clone() }, Some(&w.alice))).await), Code::Ok);
    assert_eq!(code(w.devices().remove_push_device(w.rpc(RemovePushDeviceRequest { device_uuid: device }, Some(&w.alice))).await), Code::NotFound);
}

// ---------------------------------------------------------------- authentication on every service

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn every_service_refuses_a_call_without_a_credential() {
    let w = world().await;
    let none = None;
    assert_eq!(code(w.posts().create_post(w.rpc(CreatePostRequest::default(), none)).await), Code::Unauthenticated);
    assert_eq!(code(w.feed().get_feed(w.rpc(GetFeedRequest::default(), none)).await), Code::Unauthenticated);
    assert_eq!(code(w.notifications().list_notifications(w.rpc(ListNotificationsRequest::default(), none)).await), Code::Unauthenticated);
    assert_eq!(code(w.check_ins().list_evolution_check_ins(w.rpc(ListEvolutionCheckInsRequest::default(), none)).await), Code::Unauthenticated);
    assert_eq!(code(w.devices().remove_push_device(w.rpc(RemovePushDeviceRequest::default(), none)).await), Code::Unauthenticated);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn an_oversized_message_is_out_of_range_before_any_use_case_runs() {
    let w = world().await;
    let huge = CreatePostRequest { content: "x".repeat(grpc::MAX_MESSAGE_BYTES + 1024), ..Default::default() };
    let status = w.posts().max_encoding_message_size(usize::MAX).create_post(w.rpc(huge, Some(&w.alice))).await.unwrap_err();
    assert_eq!(status.code(), Code::OutOfRange, "{status:?}");
}

// ---------------------------------------------------------------- TC-009 internal service

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn the_internal_service_accepts_only_the_shared_secret_and_exports_only_own_content() {
    use business::proto::proto::timeline::internal_service_client::InternalServiceClient;
    let w = world().await;
    unsafe { std::env::set_var("INTERNAL_SERVICE_SECRET", "c008-internal-secret") };
    let mut internal = InternalServiceClient::new(w.channel.clone());
    let ask = |secret: Option<&str>, token: Option<&Actor>, who: &str| {
        let mut request = Rpc::new(PersonDataRequest { person_uuid: who.into() });
        if let Some(secret) = secret {
            request.metadata_mut().insert("x-internal-secret", MetadataValue::try_from(secret).unwrap());
        }
        if let Some(actor) = token {
            request.metadata_mut().insert("authorization", MetadataValue::try_from(format!("Bearer {}", actor.token)).unwrap());
        }
        request
    };

    // The interceptor on the channel requires the secret; a user token is not enough, and the
    // user-token interceptor of the other services is not what guards this one.
    for (secret, token) in [(None, None), (Some("wrong"), None), (None, Some(&w.alice))] {
        let denied = internal.export_person_data(ask(secret, token, &w.bob.uuid)).await;
        assert_eq!(code(denied), Code::Unauthenticated, "{secret:?}");
    }

    // Bob comments on Alice's post and reacts; Alice's own text must not be in Bob's export.
    let alice_words = format!("alice private text {}", uuid::Uuid::new_v4());
    let post = w.publish(&w.alice, &alice_words).await.uuid.unwrap();
    w.posts().add_comment(w.rpc(AddCommentRequest { post_uuid: post.clone(), content: "bob's own words".into(), ..Default::default() }, Some(&w.bob))).await.unwrap();
    w.posts().add_reaction(w.rpc(AddReactionRequest { post_uuid: post, reaction_type: "like".into() }, Some(&w.bob))).await.unwrap();

    let bobs = internal.export_person_data(ask(Some("c008-internal-secret"), None, &w.bob.uuid)).await.unwrap().into_inner().export_json;
    assert!(bobs.contains("bob's own words"));
    assert!(!bobs.contains(&alice_words), "another person's post text is not exported");
    let alices = internal.export_person_data(ask(Some("c008-internal-secret"), None, &w.alice.uuid)).await.unwrap().into_inner().export_json;
    assert!(alices.contains(&alice_words), "a person's own post is exported whole");

    // Deleting a person removes their data and is repeatable.
    for _ in 0..2 {
        internal.delete_person_data(ask(Some("c008-internal-secret"), None, &w.alice.uuid)).await.unwrap();
    }
    let after = internal.export_person_data(ask(Some("c008-internal-secret"), None, &w.alice.uuid)).await.unwrap().into_inner().export_json;
    assert!(!after.contains(&alice_words));
}
