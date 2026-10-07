//! C-008 task 8: the contract harness (TC-013) and the cross-transport negative tests (TC-010,
//! TC-016). Every operation of SYS-C008-001..008 runs once over REST and once over gRPC against
//! the same MongoDB, and the two outcomes are compared by one rule: the REST status mapped by the
//! table in `icd.md` must equal the gRPC code, and the data both transports return, projected to
//! the fields that matter (ids and timestamps are left out), must be equal. The only accepted
//! difference is a dependency outage, which REST answers `500` and gRPC `UNAVAILABLE` (the table
//! maps it). Needs `TEST_MONGO_URL` (database `timeline_test`).
//!
//! Cast: Alice owns content; Bob is her friend; Carol is a stranger; Mod holds the moderator role;
//! Nina has no current consent.
mod standin;

use application::{
    AppState,
    routes::{
        chat_routes::chat_routes,
        content_report_routes::{moderation_routes, report_routes},
        evolution_checkin_routes::evolution_checkin_routes,
        feed_routes::feed_route,
        notification_routes::notification_routes,
        post_routes::post_routes,
        push_device_routes::push_device_routes,
        workout_session_routes::workout_session_routes,
    },
};
use axum::{Router, body::Body, http::Request as HttpRequest};
use business::proto::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use business::proto::proto::friend::friend_service_server::FriendServiceServer;
use business::proto::proto::person::person_service_server::PersonServiceServer;
use integration::proto::timeline::chat_service_client::ChatServiceClient;
use integration::proto::timeline::content_report_service_client::ContentReportServiceClient;
use integration::proto::timeline::evolution_check_in_service_client::EvolutionCheckInServiceClient;
use integration::proto::timeline::feed_service_client::FeedServiceClient;
use integration::proto::timeline::notification_service_client::NotificationServiceClient;
use integration::proto::timeline::post_service_client::PostServiceClient;
use integration::proto::timeline::push_device_service_client::PushDeviceServiceClient;
use integration::proto::timeline::workout_session_service_client::WorkoutSessionServiceClient;
use integration::proto::timeline::*;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::Client;
use serde_json::{Value, json};
use standin::Workout;
use std::collections::HashMap;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Code, Request};
use tower::ServiceExt;

const SECRET: &str = "c008-contract-test-secret";
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

#[derive(Clone, Copy, Debug, PartialEq)]
enum T {
    Rest,
    Grpc,
}

/// What one call returned, in the comparison form: the gRPC code (REST statuses are mapped) and
/// the projected data.
#[derive(Debug, Clone, PartialEq)]
struct Out {
    code: Code,
    data: Value,
}

/// icd.md section 4, REST column to gRPC column. `500` is only a dependency outage here.
fn rest_to_code(status: u16) -> Code {
    match status {
        200 | 201 | 204 => Code::Ok,
        // 422 is axum's rejection of a body that does not match the JSON shape or its types.
        400 | 422 => Code::InvalidArgument,
        401 => Code::Unauthenticated,
        403 => Code::PermissionDenied,
        404 => Code::NotFound,
        409 => Code::AlreadyExists,
        423 => Code::FailedPrecondition,
        429 => Code::ResourceExhausted,
        500 => Code::Unavailable,
        other => panic!("status {other} is not in the contract table"),
    }
}

/// The comparison rule of TC-013: both transports must give one answer.
fn same(label: &str, rest: &Out, grpc: &Out) -> Result<(), String> {
    if rest == grpc {
        Ok(())
    } else {
        Err(format!("{label}: REST gave {rest:?} but gRPC gave {grpc:?}"))
    }
}

struct Actor {
    uuid: String,
    token: String,
}

impl Actor {
    fn auth(&self) -> Option<String> {
        Some(format!("Bearer {}", self.token))
    }
}

fn person(id: i32, name: &str, lifetime: i64, secret: &str) -> Actor {
    let uuid = format!("c008x-{name}-{}", uuid::Uuid::new_v4());
    let claims = Claims {
        sub: format!("{uuid}@c008.test"),
        exp: chrono::Utc::now().timestamp() + lifetime,
        uuid: format!("user-{uuid}"),
        name: name.to_string(),
        person_id: id,
        person_uuid: uuid.clone(),
        person_object_key: String::new(),
        active_business_profile_id: None,
        active_business_profile_uuid: None,
    };
    let token = encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(secret.as_bytes())).unwrap();
    Actor { uuid, token }
}

struct World {
    channel: Channel,
    grpc_url: String,
    rest: Router,
    alice: Actor,
    bob: Actor,
    carol: Actor,
    moderator: Actor,
    nina: Actor,
    rex: Actor,
    workout: tokio::task::JoinHandle<()>,
    _guard: tokio::sync::MutexGuard<'static, ()>,
}

async fn world() -> World {
    let guard = ENV_LOCK.lock().await;
    let mongo = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(mongo.contains("/timeline_test"), "refusing to run against a non-test database");
    let database = std::sync::Arc::new(Client::with_uri_str(mongo).await.unwrap().database("timeline_test"));

    let actor = |id, name| person(id, name, 3600, SECRET);
    let (alice, bob, carol, moderator, nina, rex) = (actor(8401, "alice"), actor(8402, "bob"), actor(8403, "carol"), actor(8404, "mod"), actor(8405, "nina"), actor(8406, "rex"));
    let friends = HashMap::from([
        (alice.uuid.clone(), vec![bob.uuid.clone()]),
        (bob.uuid.clone(), vec![alice.uuid.clone()]),
    ]);
    let (moderator_token, denied_token, broken_role_token) = (moderator.token.clone(), nina.token.clone(), rex.token.clone());
    let workout_stub = move || Workout {
        friends: friends.clone(),
        moderator_tokens: vec![moderator_token.clone()],
        denied_tokens: vec![denied_token.clone()],
        role_error_tokens: vec![broken_role_token.clone()],
        ..Default::default()
    };
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let workout_port = listener.local_addr().unwrap().port();
    let workout = tokio::spawn(async move {
        let _ = tonic::transport::Server::builder()
            .add_service(PersonServiceServer::new(workout_stub()))
            .add_service(FriendServiceServer::new(workout_stub()))
            .add_service(BusinessProfileServiceServer::new(workout_stub()))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener))
            .await;
    });
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
        integration::router(tonic::transport::Server::builder(), state.database.clone(), state.chat_hub.clone())
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    let grpc_url = format!("http://127.0.0.1:{port}");
    let channel = Channel::from_shared(grpc_url.clone()).unwrap().connect().await.unwrap();
    let rest = Router::new()
        .nest("/posts", post_routes(state.clone()))
        .nest("/feed", feed_route(state.clone()))
        .nest("/notifications", notification_routes(state.clone()))
        .nest("/evolution-checkin", evolution_checkin_routes(state.clone()))
        .nest("/push-devices", push_device_routes(state.clone()))
        .nest("/chat", chat_routes(state.clone()))
        .nest("/reports", report_routes(state.clone()))
        .nest("/moderation", moderation_routes(state.clone()))
        .nest("/workout-sessions", workout_session_routes(state.clone()))
        .with_state(state);
    World { channel, grpc_url, rest, alice, bob, carol, moderator, nina, rex, workout, _guard: guard }
}

fn ip() -> String {
    format!("198.18.{}.{}", (uuid::Uuid::new_v4().as_u128() % 250) as u8, (uuid::Uuid::new_v4().as_u128() % 250) as u8 + 1)
}

/// `Auth` is the full `Authorization` header value, or `None` for no credential.
type Auth = Option<String>;

fn ok(data: Value) -> Out {
    Out { code: Code::Ok, data }
}

fn status<R>(result: Result<tonic::Response<R>, tonic::Status>, data: impl FnOnce(R) -> Value) -> Out {
    match result {
        Ok(response) => ok(data(response.into_inner())),
        Err(status) => {
            if status.code() == Code::Unknown {
                eprintln!("UNKNOWN STATUS: {status:?}");
            }
            Out { code: status.code(), data: Value::Null }
        }
    }
}

impl World {
    /// A new connection. A server that refuses many calls in a row (the interceptor answers
    /// before reading the request) may close the HTTP/2 connection as a flood protection; real
    /// clients reconnect, and so does the negative test.
    async fn reconnect(&mut self) {
        self.channel = Channel::from_shared(self.grpc_url.clone()).unwrap().connect().await.unwrap();
    }

    fn rpc<M>(&self, message: M, auth: &Auth) -> Request<M> {
        let mut request = Request::new(message);
        if let Some(value) = auth {
            if let Ok(value) = MetadataValue::try_from(value.as_str()) {
                request.metadata_mut().insert("authorization", value);
            }
        }
        request.metadata_mut().insert("x-real-ip", MetadataValue::try_from(ip()).unwrap());
        request
    }

    async fn http(&self, method: &str, uri: &str, auth: &Auth, body: Option<Value>) -> (Code, Value) {
        let mut request = HttpRequest::builder().method(method).uri(uri).header("x-real-ip", ip());
        if let Some(value) = auth {
            request = request.header("authorization", value.as_str());
        }
        let body = match body {
            Some(b) => {
                request = request.header("content-type", "application/json");
                Body::from(b.to_string())
            }
            None => Body::empty(),
        };
        let response = self.rest.clone().oneshot(request.body(body).unwrap()).await.unwrap();
        let code = rest_to_code(response.status().as_u16());
        let bytes = axum::body::to_bytes(response.into_body(), usize::MAX).await.unwrap();
        (code, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
    }

    /// A REST answer in comparison form, with `project` applied to successful bodies.
    async fn rest(&self, method: &str, uri: &str, auth: &Auth, body: Option<Value>, project: impl FnOnce(&Value) -> Value) -> Out {
        let (code, data) = self.http(method, uri, auth, body).await;
        Out { code, data: if code == Code::Ok { project(&data) } else { Value::Null } }
    }

    // ---- fixtures made over gRPC for both runs, so only the operation under test differs

    async fn publish(&self, author: &Actor, content: &str) -> String {
        PostServiceClient::new(self.channel.clone())
            .create_post(self.rpc(CreatePostRequest { content: content.into(), ..Default::default() }, &author.auth()))
            .await
            .expect("fixture post")
            .into_inner()
            .uuid
            .unwrap()
    }
}

// ---------------------------------------------------------------- projections

fn post_rest(v: &Value) -> Value {
    json!({
        "author": v["authorUuid"],
        "content": v["content"],
        "comments": v["comments"].as_array().unwrap_or(&vec![]).iter().map(|c| c["content"].clone()).collect::<Vec<_>>(),
        "reactions": v["reactions"].as_array().unwrap_or(&vec![]).iter().map(|r| json!(r["reactionType"].as_str().unwrap_or_default().to_lowercase())).collect::<Vec<_>>(),
    })
}

fn post_grpc(p: Post) -> Value {
    json!({
        "author": p.author_uuid,
        "content": p.content,
        "comments": p.comments.iter().map(|c| c.content.clone()).collect::<Vec<_>>(),
        "reactions": p.reactions.iter().map(|r| r.reaction_type.to_lowercase()).collect::<Vec<_>>(),
    })
}

fn contents_rest(v: &Value) -> Value {
    let mut all: Vec<String> = v.as_array().unwrap_or(&vec![]).iter().filter_map(|p| p["content"].as_str().map(str::to_owned)).collect();
    all.sort();
    json!(all)
}

fn contents_grpc(posts: Vec<Post>) -> Value {
    let mut all: Vec<String> = posts.into_iter().map(|p| p.content).collect();
    all.sort();
    json!(all)
}

// ---------------------------------------------------------------- operations, once per transport

impl World {
    async fn create_post(&self, t: T, auth: &Auth, content: &str) -> Out {
        match t {
            T::Rest => self.rest("POST", "/posts", auth, Some(json!({"authorId": 0, "authorUuid": "", "authorName": "", "content": content})), post_rest).await,
            T::Grpc => status(PostServiceClient::new(self.channel.clone()).create_post(self.rpc(CreatePostRequest { content: content.into(), ..Default::default() }, auth)).await, post_grpc),
        }
    }

    async fn comment(&self, t: T, auth: &Auth, post: &str, text: &str) -> Out {
        match t {
            T::Rest => self.rest("POST", &format!("/posts/{post}/comments"), auth, Some(json!({"postUuid": post, "authorUuid": "", "authorName": "", "content": text})), post_rest).await,
            T::Grpc => status(PostServiceClient::new(self.channel.clone()).add_comment(self.rpc(AddCommentRequest { post_uuid: post.into(), content: text.into(), ..Default::default() }, auth)).await, post_grpc),
        }
    }

    async fn react(&self, t: T, auth: &Auth, post: &str, kind: &str) -> Out {
        match t {
            T::Rest => self.rest("POST", &format!("/posts/{post}/reactions"), auth, Some(json!({"authorId": "", "authorName": "", "reactionType": kind})), post_rest).await,
            T::Grpc => status(PostServiceClient::new(self.channel.clone()).add_reaction(self.rpc(AddReactionRequest { post_uuid: post.into(), reaction_type: kind.into() }, auth)).await, post_grpc),
        }
    }

    async fn unreact(&self, t: T, auth: &Auth, post: &str) -> Out {
        match t {
            T::Rest => self.rest("DELETE", &format!("/posts/{post}/reactions"), auth, None, post_rest).await,
            T::Grpc => status(PostServiceClient::new(self.channel.clone()).remove_reaction(self.rpc(RemoveReactionRequest { post_uuid: post.into() }, auth)).await, post_grpc),
        }
    }

    async fn delete_post(&self, t: T, auth: &Auth, post: &str) -> Out {
        match t {
            T::Rest => self.rest("DELETE", &format!("/posts/{post}"), auth, None, |_| Value::Null).await,
            T::Grpc => status(PostServiceClient::new(self.channel.clone()).delete_post(self.rpc(DeletePostRequest { post_uuid: post.into() }, auth)).await, |_| Value::Null),
        }
    }

    async fn feed(&self, t: T, auth: &Auth) -> Out {
        match t {
            T::Rest => self.rest("GET", "/feed?page=0", auth, None, contents_rest).await,
            T::Grpc => status(FeedServiceClient::new(self.channel.clone()).get_feed(self.rpc(GetFeedRequest { page: 0 }, auth)).await, |r| contents_grpc(r.posts)),
        }
    }

    async fn feed_by_author(&self, t: T, auth: &Auth, author: &str) -> Out {
        match t {
            T::Rest => self.rest("GET", &format!("/feed/{author}?page=0"), auth, None, contents_rest).await,
            T::Grpc => status(FeedServiceClient::new(self.channel.clone()).get_feed_by_author(self.rpc(GetFeedByAuthorRequest { author_uuid: author.into(), page: 0 }, auth)).await, |r| contents_grpc(r.posts)),
        }
    }

    /// Returns the outcome plus the first notification's key, to feed `mark_read`.
    async fn notifications(&self, t: T, auth: &Auth, owner: &str) -> (Out, Option<String>) {
        let summary = |items: Vec<(String, String)>| {
            let mut kinds: Vec<String> = items.iter().map(|(kind, _)| kind.clone()).collect();
            kinds.sort();
            (json!(kinds), items.first().map(|(_, key)| key.clone()))
        };
        match t {
            T::Rest => {
                let (code, body) = self.http("GET", &format!("/notifications/{owner}?unreadOnly=true"), auth, None).await;
                if code != Code::Ok {
                    return (Out { code, data: Value::Null }, None);
                }
                let items = body.as_array().unwrap().iter().map(|n| (n["notificationType"].as_str().unwrap().to_string(), n["uuid"].as_str().unwrap().to_string())).collect();
                let (data, key) = summary(items);
                (ok(data), key)
            }
            T::Grpc => match NotificationServiceClient::new(self.channel.clone()).list_notifications(self.rpc(ListNotificationsRequest { unread_only: true, limit: 0 }, auth)).await {
                Ok(r) => {
                    let items = r.into_inner().notifications.into_iter().map(|n| (n.notification_type, n.uuid.unwrap())).collect();
                    let (data, key) = summary(items);
                    (ok(data), key)
                }
                Err(s) => (Out { code: s.code(), data: Value::Null }, None),
            },
        }
    }

    async fn mark_read(&self, t: T, auth: &Auth, owner: &str, key: &str) -> Out {
        match t {
            T::Rest => self.rest("PUT", &format!("/notifications/{owner}/read/{key}"), auth, None, |v| v.clone()).await,
            T::Grpc => status(NotificationServiceClient::new(self.channel.clone()).mark_notification_read(self.rpc(MarkNotificationReadRequest { idempotency_key: key.into() }, auth)).await, |r| json!({"read": r.read})),
        }
    }

    async fn add_check_in(&self, t: T, auth: &Auth, created_at: &str, weight: f64) -> Out {
        match t {
            T::Rest => self.rest("POST", "/evolution-checkin", auth, Some(json!({"personUuid": "", "createdAt": created_at, "visibility": "Private", "composition": {"weight": weight}})), |v| json!({"weight": v["composition"]["weight"], "visibility": v["visibility"]})).await,
            T::Grpc => status(
                EvolutionCheckInServiceClient::new(self.channel.clone())
                    .add_evolution_check_in(self.rpc(AddEvolutionCheckInRequest { created_at: created_at.into(), visibility: "Private".into(), composition: Some(BodyComposition { weight, ..Default::default() }), ..Default::default() }, auth))
                    .await,
                |c| json!({"weight": c.composition.map(|b| b.weight), "visibility": c.visibility}),
            ),
        }
    }

    async fn list_check_ins(&self, t: T, auth: &Auth, start: &str, end: &str) -> Out {
        match t {
            T::Rest => self.rest("GET", &format!("/evolution-checkin?startDate={start}&endDate={end}"), auth, None, |v| json!(v.as_array().unwrap().iter().map(|c| c["composition"]["weight"].clone()).collect::<Vec<_>>())).await,
            T::Grpc => status(
                EvolutionCheckInServiceClient::new(self.channel.clone())
                    .list_evolution_check_ins(self.rpc(ListEvolutionCheckInsRequest { start_date: Some(start.into()), end_date: Some(end.into()) }, auth))
                    .await,
                |r| json!(r.check_ins.into_iter().map(|c| c.composition.map(|b| b.weight)).collect::<Vec<_>>()),
            ),
        }
    }

    async fn register_device(&self, t: T, auth: &Auth, device: &str, platform: &str) -> Out {
        match t {
            T::Rest => self.rest("PUT", &format!("/push-devices/{device}"), auth, Some(json!({"platform": platform, "registrationToken": "token-1"})), |_| Value::Null).await,
            T::Grpc => status(PushDeviceServiceClient::new(self.channel.clone()).register_push_device(self.rpc(RegisterPushDeviceRequest { device_uuid: device.into(), platform: platform.into(), registration_token: "token-1".into() }, auth)).await, |_| Value::Null),
        }
    }

    async fn remove_device(&self, t: T, auth: &Auth, device: &str) -> Out {
        match t {
            T::Rest => self.rest("DELETE", &format!("/push-devices/{device}"), auth, None, |_| Value::Null).await,
            T::Grpc => status(PushDeviceServiceClient::new(self.channel.clone()).remove_push_device(self.rpc(RemovePushDeviceRequest { device_uuid: device.into() }, auth)).await, |_| Value::Null),
        }
    }

    async fn direct(&self, t: T, auth: &Auth, target: &str) -> (Out, Option<String>) {
        let participants = |mut p: Vec<String>| {
            p.sort();
            json!(p)
        };
        match t {
            T::Rest => {
                let (code, body) = self.http("POST", "/chat/conversations/direct", auth, Some(json!({"targetPersonUuid": target}))).await;
                if code != Code::Ok {
                    return (Out { code, data: Value::Null }, None);
                }
                let list = body["participantPersonUuids"].as_array().unwrap().iter().map(|u| u.as_str().unwrap().to_string()).collect();
                (ok(participants(list)), body["uuid"].as_str().map(str::to_owned))
            }
            T::Grpc => match ChatServiceClient::new(self.channel.clone()).create_direct_conversation(self.rpc(CreateDirectConversationRequest { target_person_uuid: target.into() }, auth)).await {
                Ok(r) => {
                    let c = r.into_inner();
                    (ok(participants(c.participant_person_uuids)), Some(c.uuid))
                }
                Err(s) => (Out { code: s.code(), data: Value::Null }, None),
            },
        }
    }

    async fn send(&self, t: T, auth: &Auth, conversation: &str, body: &str, client_id: &str) -> (Out, Option<String>) {
        let shape = |sender: String, text: String, id: String| json!({"sender": sender, "body": text, "clientMessageId": id});
        match t {
            T::Rest => {
                let (code, v) = self.http("POST", &format!("/chat/conversations/{conversation}/messages"), auth, Some(json!({"body": body, "media": [], "clientMessageId": client_id}))).await;
                if code != Code::Ok {
                    return (Out { code, data: Value::Null }, None);
                }
                (ok(shape(v["senderPersonUuid"].as_str().unwrap().into(), v["body"].as_str().unwrap().into(), v["clientMessageId"].as_str().unwrap().into())), v["uuid"].as_str().map(str::to_owned))
            }
            T::Grpc => match ChatServiceClient::new(self.channel.clone()).send_message(self.rpc(SendMessageRequest { conversation_uuid: conversation.into(), body: body.into(), media: vec![], client_message_id: client_id.into() }, auth)).await {
                Ok(r) => {
                    let m = r.into_inner();
                    (ok(shape(m.sender_person_uuid, m.body.clone(), m.client_message_id)), Some(m.uuid))
                }
                Err(s) => (Out { code: s.code(), data: Value::Null }, None),
            },
        }
    }

    async fn messages(&self, t: T, auth: &Auth, conversation: &str) -> Out {
        match t {
            T::Rest => self.rest("GET", &format!("/chat/conversations/{conversation}/messages?page=0"), auth, None, |v| json!(v.as_array().unwrap().iter().map(|m| m["body"].clone()).collect::<Vec<_>>())).await,
            T::Grpc => status(ChatServiceClient::new(self.channel.clone()).list_messages(self.rpc(ListMessagesRequest { conversation_uuid: conversation.into(), ..Default::default() }, auth)).await, |r| json!(r.messages.into_iter().map(|m| m.body).collect::<Vec<_>>())),
        }
    }

    async fn conversations(&self, t: T, auth: &Auth) -> Out {
        match t {
            T::Rest => self.rest("GET", "/chat/conversations?page=0", auth, None, |v| json!(v.as_array().unwrap().len())).await,
            T::Grpc => status(ChatServiceClient::new(self.channel.clone()).list_conversations(self.rpc(ListConversationsRequest::default(), auth)).await, |r| json!(r.conversations.len())),
        }
    }

    async fn mark_conversation_read(&self, t: T, auth: &Auth, conversation: &str, message: &str) -> Out {
        match t {
            T::Rest => self.rest("PUT", &format!("/chat/conversations/{conversation}/read"), auth, Some(json!({"lastReadMessageUuid": message})), |v| v.clone()).await,
            T::Grpc => status(ChatServiceClient::new(self.channel.clone()).mark_conversation_read(self.rpc(MarkConversationReadRequest { conversation_uuid: conversation.into(), last_read_message_uuid: message.into() }, auth)).await, |r| json!({"read": r.read})),
        }
    }

    async fn presence(&self, t: T, auth: &Auth, asked: &[&str]) -> Out {
        match t {
            T::Rest => self.rest("GET", &format!("/chat/presence?uuids={}", asked.join(",")), auth, None, |v| json!(v["online"])).await,
            T::Grpc => status(ChatServiceClient::new(self.channel.clone()).get_presence(self.rpc(GetPresenceRequest { person_uuids: asked.iter().map(|s| s.to_string()).collect() }, auth)).await, |r| json!(r.online)),
        }
    }

    async fn report(&self, t: T, auth: &Auth, post: &str, reason: &str) -> (Out, Option<String>) {
        let shape = |reporter: String, status: String| json!({"reporter": reporter, "status": status});
        match t {
            T::Rest => {
                let (code, v) = self.http("POST", "/reports", auth, Some(json!({"targetType": "post", "targetId": post, "postId": post, "reason": reason}))).await;
                if code != Code::Ok {
                    return (Out { code, data: Value::Null }, None);
                }
                // The report is serialized as stored, so its id is `_id`.
                (ok(shape(v["reporterPersonUuid"].as_str().unwrap().into(), v["status"].as_str().unwrap().into())), v["_id"].as_str().map(str::to_owned))
            }
            T::Grpc => match ContentReportServiceClient::new(self.channel.clone()).create_report(self.rpc(CreateReportRequest { target_type: "post".into(), target_id: post.into(), post_id: post.into(), reason: reason.into(), details: None }, auth)).await {
                Ok(r) => {
                    let r = r.into_inner();
                    (ok(shape(r.reporter_person_uuid, r.status)), Some(r.uuid))
                }
                Err(s) => (Out { code: s.code(), data: Value::Null }, None),
            },
        }
    }

    async fn list_reports(&self, t: T, auth: &Auth, reason: &str) -> Out {
        match t {
            T::Rest => self.rest("GET", "/moderation/reports", auth, None, |v| json!(v.as_array().unwrap().iter().filter(|r| r["reason"] == reason).count())).await,
            T::Grpc => status(ContentReportServiceClient::new(self.channel.clone()).list_reports(self.rpc(ListReportsRequest::default(), auth)).await, |r| json!(r.reports.iter().filter(|x| x.reason == reason).count())),
        }
    }

    async fn decide(&self, t: T, auth: &Auth, report: &str) -> Out {
        match t {
            T::Rest => self.rest("POST", &format!("/moderation/reports/{report}/decision"), auth, Some(json!({"decision": "dismissed", "reason": "checked"})), |v| json!({"status": v["status"], "decision": v["decision"]})).await,
            T::Grpc => status(ContentReportServiceClient::new(self.channel.clone()).decide_report(self.rpc(DecideReportRequest { report_id: report.into(), decision: "dismissed".into(), reason: "checked".into() }, auth)).await, |r| json!({"status": r.status, "decision": r.decision})),
        }
    }

    async fn create_session(&self, t: T, auth: &Auth, started: &str) -> (Out, Option<String>) {
        let shape = |owner: String, name: Option<String>, sets: usize| json!({"owner": owner, "name": name, "sets": sets});
        let set = json!({"exerciseName": "Squat", "ownerId": 1, "ownerName": "Alice", "setNumber": 1, "repsOrDuration": 10, "weight": 80.0, "startedAt": started, "completedAt": started});
        match t {
            T::Rest => {
                let body = json!({"personUuid": "ignored", "workoutName": "Legs", "duration": 30, "startedAt": started, "completedAt": started, "executedSets": [set], "totalVolume": 800.0, "totalSets": 1.0});
                let (code, v) = self.http("POST", "/workout-sessions", auth, Some(body)).await;
                if code != Code::Ok {
                    return (Out { code, data: Value::Null }, None);
                }
                (ok(shape(v["personUuid"].as_str().unwrap().into(), v["workoutName"].as_str().map(str::to_owned), v["executedSets"].as_array().unwrap().len())), v["uuid"].as_str().map(str::to_owned))
            }
            T::Grpc => {
                let session = WorkoutSession {
                    person_uuid: "ignored".into(),
                    workout_name: Some("Legs".into()),
                    duration: 30,
                    started_at: Some(started.into()),
                    completed_at: Some(started.into()),
                    executed_sets: vec![ExecutedSet { exercise_name: Some("Squat".into()), owner_id: 1, owner_name: Some("Alice".into()), set_number: 1, reps_or_duration: 10, weight: 80.0, started_at: Some(started.into()), completed_at: Some(started.into()), ..Default::default() }],
                    total_volume: 800.0,
                    total_sets: 1.0,
                    ..Default::default()
                };
                match WorkoutSessionServiceClient::new(self.channel.clone()).create_workout_session(self.rpc(CreateWorkoutSessionRequest { session: Some(session) }, auth)).await {
                    Ok(r) => {
                        let s = r.into_inner();
                        (ok(shape(s.person_uuid, s.workout_name, s.executed_sets.len())), s.uuid)
                    }
                    Err(s) => (Out { code: s.code(), data: Value::Null }, None),
                }
            }
        }
    }

    async fn get_session(&self, t: T, auth: &Auth, id: &str) -> Out {
        match t {
            T::Rest => self.rest("GET", &format!("/workout-sessions/{id}"), auth, None, |v| json!({"owner": v["personUuid"], "name": v["workoutName"]})).await,
            T::Grpc => status(WorkoutSessionServiceClient::new(self.channel.clone()).get_workout_session(self.rpc(GetWorkoutSessionRequest { session_uuid: id.into() }, auth)).await, |s| json!({"owner": s.person_uuid, "name": s.workout_name})),
        }
    }

    async fn list_sessions(&self, t: T, auth: &Auth, start: &str, end: &str) -> Out {
        match t {
            T::Rest => self.rest("GET", &format!("/workout-sessions?startDate={start}&endDate={end}"), auth, None, |v| json!(v.as_array().unwrap().len())).await,
            T::Grpc => status(WorkoutSessionServiceClient::new(self.channel.clone()).list_workout_sessions(self.rpc(ListWorkoutSessionsRequest { start_date: Some(start.into()), end_date: Some(end.into()) }, auth)).await, |r| json!(r.sessions.len())),
        }
    }
}

// ---------------------------------------------------------------- scenarios

/// Runs `scenario` over REST and over gRPC and compares every labelled step.
async fn compare<F, Fut>(w: &World, scenario: F)
where
    F: Fn(T) -> Fut,
    Fut: std::future::Future<Output = Vec<(String, Out)>>,
{
    let (rest, grpc) = (scenario(T::Rest).await, scenario(T::Grpc).await);
    assert_eq!(rest.len(), grpc.len());
    let _ = w;
    let failures: Vec<String> = rest.iter().zip(&grpc).filter_map(|((label, r), (_, g))| same(label, r, g).err()).collect();
    assert!(failures.is_empty(), "REST and gRPC differ:\n{}", failures.join("\n"));
    // A contract test that compared nothing would pass forever.
    assert!(rest.iter().any(|(_, o)| o.code == Code::Ok), "the scenario never succeeded once");
}

fn step(label: &str, out: Out) -> (String, Out) {
    (label.to_string(), out)
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn posts_and_feeds_give_equal_outcomes_on_both_transports() {
    let world = world().await;
    let w = &world;
    compare(w, |t| async move {
        let (a, b, c) = (w.alice.auth(), w.bob.auth(), w.carol.auth());
        let marker = format!("post {}", uuid::Uuid::new_v4());
        let created = w.create_post(t, &a, &marker).await;
        let post = w.publish(&w.alice, "fixture post").await;
        let mut steps = vec![
            step("create", Out { code: created.code, data: json!({"author": created.data["author"], "content": created.data["content"].as_str().map(|c| c.starts_with("post "))}) }),
            step("friend comments", w.comment(t, &b, &post, "nice").await),
            step("friend reacts LOVE", w.react(t, &b, &post, "LOVE").await),
            step("friend reacts dislike", w.react(t, &b, &post, "dislike").await),
            step("stranger comments", w.comment(t, &c, &post, "hi").await),
            step("stranger reacts", w.react(t, &c, &post, "like").await),
            step("stranger unreacts", w.unreact(t, &c, &post).await),
            step("stranger deletes", w.delete_post(t, &c, &post).await),
            step("friend deletes", w.delete_post(t, &b, &post).await),
            step("friend unreacts", w.unreact(t, &b, &post).await),
            step("feed of a person by uuid", w.feed_by_author(t, &b, &w.alice.uuid).await),
        ];
        // Only the marker posts are compared, so posts of other tests cannot interfere.
        let feed = w.feed(t, &b).await;
        let mine = feed.data.as_array().map(|all| all.iter().filter(|c| c.as_str().is_some_and(|s| s.contains(&marker))).count());
        steps.push(step("friend feed holds the marker posts", Out { code: feed.code, data: json!(mine) }));
        steps.push(step("author deletes", w.delete_post(t, &a, &post).await));
        steps.push(step("author deletes again", w.delete_post(t, &a, &post).await));
        steps
    })
    .await;
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn notifications_check_ins_and_devices_give_equal_outcomes() {
    let world = world().await;
    let w = &world;
    compare(w, |t| async move {
        let (a, b, c) = (w.alice.auth(), w.bob.auth(), w.carol.auth());
        let post = w.publish(&w.alice, "to be commented").await;
        w.comment(T::Grpc, &b, &post, "ping").await;
        let (alices, key) = w.notifications(t, &a, &w.alice.uuid).await;
        let key = key.unwrap_or_default();
        let now = chrono::Utc::now().naive_utc();
        let stamp = |d: chrono::NaiveDateTime| serde_json::to_value(d).unwrap().as_str().unwrap().to_string();
        let (from, to, created) = (stamp(now - chrono::Duration::days(1)), stamp(now + chrono::Duration::days(1)), stamp(now));
        let device = uuid::Uuid::new_v4().to_string();
        vec![
            step("recipient lists", Out { code: alices.code, data: json!(alices.data.as_array().map(|v| v.contains(&json!("Comment")))) }),
            // REST has an owner in the path (`/notifications/{owner}`, 403 for anyone else); gRPC has
            // no owner field and always answers for the caller, so that case has no gRPC twin.
            step("stranger lists their own", Out { code: Code::Ok, data: json!(w.notifications(t, &c, &w.carol.uuid).await.0.data.as_array().map(|v| v.is_empty())) }),
            step("stranger marks Alice's key", w.mark_read(t, &c, &w.carol.uuid, &key).await),
            step("recipient marks", w.mark_read(t, &a, &w.alice.uuid, &key).await),
            step("check-in", w.add_check_in(t, &a, &created, 81.5).await),
            step("check-in with a bad date", w.add_check_in(t, &a, "not a date", 1.0).await),
            step("own check-ins", Out { code: Code::Ok, data: json!(w.list_check_ins(t, &a, &from, &to).await.data.as_array().map(|v| v.contains(&json!(81.5)))) }),
            step("another person's check-ins", Out { code: Code::Ok, data: json!(w.list_check_ins(t, &b, &from, &to).await.data.as_array().map(|v| v.contains(&json!(81.5)))) }),
            step("register device", w.register_device(t, &a, &device, "android").await),
            step("register bad platform", w.register_device(t, &a, &device, "windows").await),
            step("stranger removes it", w.remove_device(t, &c, &device).await),
            step("owner removes it", w.remove_device(t, &a, &device).await),
            step("owner removes it again", w.remove_device(t, &a, &device).await),
        ]
    })
    .await;
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn chat_gives_equal_outcomes_on_both_transports() {
    let world = world().await;
    let w = &world;
    compare(w, |t| async move {
        let (a, b, c) = (w.alice.auth(), w.bob.auth(), w.carol.auth());
        let (created, conversation) = w.direct(t, &a, &w.bob.uuid).await;
        let conversation = conversation.unwrap();
        let (sent, message) = w.send(t, &a, &conversation, &format!("oi {t:?}"), &uuid::Uuid::new_v4().to_string()).await;
        let (_stranger_direct, _) = w.direct(t, &c, &w.alice.uuid).await;
        vec![
            step("create direct", created),
            step("send", Out { code: sent.code, data: json!({"sender": sent.data["sender"], "body": sent.data["body"].as_str().map(|b| b.starts_with("oi"))}) }),
            step("stranger sends", w.send(t, &c, &conversation, "let me in", "x1").await.0),
            step("friend lists conversations", { let o = w.conversations(t, &b).await; Out { code: o.code, data: json!(o.data.as_u64().map(|n| n >= 1)) } }),
            step("stranger lists messages", w.messages(t, &c, &conversation).await),
            step("friend marks read", w.mark_conversation_read(t, &b, &conversation, &message.unwrap_or_default()).await),
            step("stranger marks read", w.mark_conversation_read(t, &c, &conversation, "none").await),
            step("presence of the unrelated", w.presence(t, &b, &[&w.carol.uuid, "nobody-he-knows"]).await),
        ]
    })
    .await;
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn reports_and_sessions_give_equal_outcomes_on_both_transports() {
    let world = world().await;
    let w = &world;
    compare(w, |t| async move {
        let (a, b, c, m) = (w.alice.auth(), w.bob.auth(), w.carol.auth(), w.moderator.auth());
        let post = w.publish(&w.alice, "reportable").await;
        let reason = format!("spam {t:?} {}", uuid::Uuid::new_v4());
        let (reported, report) = w.report(t, &b, &post, &reason).await;
        let report = report.unwrap_or_else(|| panic!("report not created over {t:?}: {:?}", reported.code));
        let now = chrono::Utc::now().naive_utc();
        let stamp = |d: chrono::NaiveDateTime| serde_json::to_value(d).unwrap().as_str().unwrap().to_string();
        let (from, to, started) = (stamp(now - chrono::Duration::days(1)), stamp(now + chrono::Duration::days(1)), stamp(now));
        let (created, id) = w.create_session(t, &a, &started).await;
        let id = id.unwrap();
        // The report listed by the moderator is counted by its own reason, which differs per run.
        let listed = w.list_reports(t, &m, &reason).await;
        vec![
            step("report", reported),
            step("report of an unreadable post", w.report(t, &c, &post, "x").await.0),
            step("report of a missing post", w.report(t, &c, "no-such-post", "x").await.0),
            step("regular person lists", w.list_reports(t, &b, &reason).await),
            step("moderator lists", listed),
            step("regular person decides", w.decide(t, &b, &report).await),
            step("moderator decides", w.decide(t, &m, &report).await),
            step("moderator decides unknown", w.decide(t, &m, "no-such-report").await),
            step("create session", created),
            step("owner gets it", w.get_session(t, &a, &id).await),
            step("another person gets it", w.get_session(t, &b, &id).await),
            step("missing session", w.get_session(t, &b, "no-such-session").await),
            step("owner lists", Out { code: Code::Ok, data: json!(w.list_sessions(t, &a, &from, &to).await.data.as_u64().map(|n| n >= 1)) }),
            step("another person lists", w.list_sessions(t, &c, &from, &to).await),
        ]
    })
    .await;
}

// ---------------------------------------------------------------- TC-010 / TC-016 negatives

/// One operation of every service, as a function of the credential.
async fn one_of_each(w: &World, t: T, auth: &Auth) -> Vec<(String, Out)> {
    let post = w.publish(&w.alice, "target").await;
    vec![
        step("posts", w.create_post(t, auth, "x").await),
        step("feed", w.feed(t, auth).await),
        step("notifications", w.notifications(t, auth, &w.alice.uuid).await.0),
        step("check-ins", w.list_check_ins(t, auth, "2020-01-01T00:00:00", "2030-01-01T00:00:00").await),
        step("push devices", w.remove_device(t, auth, &uuid::Uuid::new_v4().to_string()).await),
        step("chat", w.conversations(t, auth).await),
        step("reports", w.report(t, auth, &post, "x").await.0),
        step("moderation", w.list_reports(t, auth, "x").await),
        step("sessions", w.list_sessions(t, auth, "2020-01-01T00:00:00", "2030-01-01T00:00:00").await),
    ]
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn every_service_answers_unauthenticated_for_a_missing_malformed_expired_or_tampered_credential() {
    let mut w = world().await;
    let good = w.alice.token.clone();
    let expired = person(1, "old", -3600, SECRET).token;
    let tampered = person(1, "forged", 3600, "another-secret").token;
    let cases: Vec<(&str, Auth)> = vec![
        ("no header", None),
        ("bare scheme", Some("Bearer".into())),
        ("wrong scheme", Some(format!("Basic {good}"))),
        ("extra part", Some(format!("Bearer {good} extra"))),
        ("expired", Some(format!("Bearer {expired}"))),
        ("tampered", Some(format!("Bearer {tampered}"))),
        ("garbage", Some("Bearer not-a-jwt".into())),
    ];
    for (name, auth) in cases {
        for t in [T::Rest, T::Grpc] {
            w.reconnect().await;
            for (service, out) in one_of_each(&w, t, &auth).await {
                assert_eq!(out.code, Code::Unauthenticated, "{service} over {t:?} with {name}");
            }
        }
    }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn a_person_without_current_consent_is_denied_on_both_transports() {
    let w = world().await;
    let nina = w.nina.auth();
    for t in [T::Rest, T::Grpc] {
        for (service, out) in one_of_each(&w, t, &nina).await {
            assert_eq!(out.code, Code::PermissionDenied, "{service} over {t:?}");
        }
    }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn a_dependency_outage_is_the_only_difference_and_maps_to_unavailable() {
    let w = world().await;
    w.workout.abort();
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    let a = w.alice.auth();
    let (rest, grpc) = (w.create_post(T::Rest, &a, "while workout is down").await, w.create_post(T::Grpc, &a, "while workout is down").await);
    assert_eq!(grpc.code, Code::Unavailable);
    assert_eq!(rest.code, Code::Unavailable, "REST answers 500, which the contract table maps to UNAVAILABLE");
}

/// TC-013 step 3: the harness must notice a pair that differs.
#[test]
fn the_comparison_reports_a_pair_that_differs() {
    let a = Out { code: Code::Ok, data: json!({"content": "x"}) };
    let b = Out { code: Code::Ok, data: json!({"content": "y"}) };
    assert!(same("pair", &a, &a).is_ok());
    assert!(same("pair", &a, &b).is_err());
    assert!(same("pair", &a, &Out { code: Code::NotFound, data: Value::Null }).is_err());
}

/// TC-010 step 5: when the role lookup itself fails the moderation operations deny, on both
/// transports, instead of failing open or answering as an outage.
#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn a_failing_role_lookup_denies_moderation_on_both_transports() {
    let w = world().await;
    let rex = w.rex.auth();
    for t in [T::Rest, T::Grpc] {
        assert_eq!(w.list_reports(t, &rex, "x").await.code, Code::PermissionDenied, "{t:?}");
    }
}
