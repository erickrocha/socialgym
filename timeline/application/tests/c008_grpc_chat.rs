//! C-008 task 5: the gRPC chat operations (TC-005) and the real-time stream (TC-006), through a
//! real in-process server against MongoDB, with the shared `workout` stand-in.
//! Needs `TEST_MONGO_URL` (database `timeline_test`).
//!
//! Cast: Alice and Bob are friends with one direct conversation; Carol is a stranger.
mod standin;

use application::{AppState, grpc};
use business::proto::proto::business_profile::business_profile_service_server::BusinessProfileServiceServer;
use business::proto::proto::friend::friend_service_server::FriendServiceServer;
use business::proto::proto::person::person_service_server::PersonServiceServer;
use business::proto::proto::timeline::chat_service_client::ChatServiceClient;
use business::proto::proto::timeline::client_frame::Frame;
use business::proto::proto::timeline::server_frame::Event;
use business::proto::proto::timeline::*;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::Client;
use standin::Workout;
use std::collections::HashMap;
use std::time::Duration;
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;
use tonic::metadata::MetadataValue;
use tonic::transport::Channel;
use tonic::{Code, Request, Streaming};

const SECRET: &str = "c008-chat-test-secret";
static ENV_LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());

struct Actor {
    uuid: String,
    token: String,
}

fn person_with_lifetime(id: i32, name: &str, seconds: i64) -> Actor {
    let uuid = format!("c008c-{name}-{}", uuid::Uuid::new_v4());
    let claims = Claims {
        sub: format!("{uuid}@c008.test"),
        exp: chrono::Utc::now().timestamp() + seconds,
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
    alice: Actor,
    bob: Actor,
    carol: Actor,
    state: AppState,
    conversation: String,
    _guard: tokio::sync::MutexGuard<'static, ()>,
}

struct Open {
    send: mpsc::Sender<ClientFrame>,
    events: Streaming<ServerFrame>,
}

impl Open {
    async fn send(&self, frame: Frame) {
        self.send.send(ClientFrame { frame: Some(frame) }).await.unwrap();
    }

    /// The next event matching `wanted`, skipping others, within two seconds.
    async fn next<T>(&mut self, wanted: impl Fn(Event) -> Option<T>) -> T {
        tokio::time::timeout(Duration::from_secs(2), async {
            loop {
                let event = self.events.message().await.unwrap().expect("stream ended").event.expect("event");
                if let Some(found) = wanted(event) {
                    return found;
                }
            }
        })
        .await
        .expect("expected event did not arrive")
    }

    /// True when no event arrives for a short while.
    async fn stays_quiet(&mut self) -> bool {
        tokio::time::timeout(Duration::from_millis(400), self.events.message()).await.is_err()
    }
}

async fn world_with(denied_for_carol: bool) -> World {
    let guard = ENV_LOCK.lock().await;
    let mongo = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(mongo.contains("/timeline_test"), "refusing to run against a non-test database");
    let database = std::sync::Arc::new(Client::with_uri_str(mongo).await.unwrap().database("timeline_test"));

    let (alice, bob, carol) = (person_with_lifetime(8201, "alice", 3600), person_with_lifetime(8202, "bob", 3600), person_with_lifetime(8203, "carol", 3600));
    let friends = HashMap::from([
        (alice.uuid.clone(), vec![bob.uuid.clone()]),
        (bob.uuid.clone(), vec![alice.uuid.clone()]),
    ]);
    let denied = if denied_for_carol { vec![carol.token.clone()] } else { vec![] };
    let workout = || Workout { friends: friends.clone(), denied_tokens: denied.clone(), ..Default::default() };
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
    let mut world = World { channel, alice, bob, carol, state, conversation: String::new(), _guard: guard };
    let created = world
        .chat()
        .create_direct_conversation(world.rpc(CreateDirectConversationRequest { target_person_uuid: world.bob.uuid.clone() }, Some(&world.alice)))
        .await
        .expect("setup: Alice opens a conversation with Bob")
        .into_inner();
    world.conversation = created.uuid;
    world
}

async fn world() -> World {
    world_with(false).await
}

impl World {
    fn rpc<T>(&self, message: T, actor: Option<&Actor>) -> Request<T> {
        let mut request = Request::new(message);
        if let Some(actor) = actor {
            request.metadata_mut().insert("authorization", MetadataValue::try_from(format!("Bearer {}", actor.token)).unwrap());
        }
        // A distinct address per call keeps the chat rate limit out of the way.
        let ip = format!("192.0.2.{}", (uuid::Uuid::new_v4().as_u128() % 250) as u8 + 1);
        request.metadata_mut().insert("x-real-ip", MetadataValue::try_from(ip).unwrap());
        request
    }

    fn chat(&self) -> ChatServiceClient<Channel> {
        ChatServiceClient::new(self.channel.clone())
    }

    async fn open(&self, actor: Option<&Actor>) -> Result<Open, tonic::Status> {
        let (send, frames) = mpsc::channel(16);
        let events = self.chat().open_stream(self.rpc(ReceiverStream::new(frames), actor)).await?.into_inner();
        Ok(Open { send, events })
    }

    async fn messages(&self, actor: &Actor) -> Vec<Message> {
        self.chat()
            .list_messages(self.rpc(ListMessagesRequest { conversation_uuid: self.conversation.clone(), ..Default::default() }, Some(actor)))
            .await
            .unwrap()
            .into_inner()
            .messages
    }
}

fn send_frame(conversation: &str, body: &str, id: &str) -> Frame {
    Frame::Send(SendFrame { conversation_uuid: conversation.into(), body: body.into(), media: vec![], client_message_id: id.into() })
}

fn code<T>(result: Result<tonic::Response<T>, tonic::Status>) -> Code {
    result.err().map(|s| s.code()).unwrap_or(Code::Ok)
}

// ---------------------------------------------------------------- TC-005

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn unary_chat_operations_enforce_participation_and_presence() {
    let w = world().await;
    let sent = w
        .chat()
        .send_message(w.rpc(SendMessageRequest { conversation_uuid: w.conversation.clone(), body: "oi".into(), media: vec![], client_message_id: "u1".into() }, Some(&w.alice)))
        .await
        .unwrap()
        .into_inner();
    assert_eq!(sent.sender_person_uuid, w.alice.uuid);

    let listed = w.chat().list_conversations(w.rpc(ListConversationsRequest::default(), Some(&w.bob))).await.unwrap().into_inner().conversations;
    assert!(listed.iter().any(|c| c.uuid == w.conversation));
    assert_eq!(w.messages(&w.bob).await.len(), 1);

    let intruder = w
        .chat()
        .send_message(w.rpc(SendMessageRequest { conversation_uuid: w.conversation.clone(), body: "hello".into(), media: vec![], client_message_id: "c1".into() }, Some(&w.carol)))
        .await;
    assert_eq!(code(intruder), Code::PermissionDenied);
    assert_eq!(w.messages(&w.alice).await.len(), 1, "nothing from Carol is stored");

    let read = w.chat().mark_conversation_read(w.rpc(MarkConversationReadRequest { conversation_uuid: w.conversation.clone(), last_read_message_uuid: sent.uuid.clone() }, Some(&w.bob))).await;
    assert_eq!(code(read), Code::Ok);

    // Presence: Alice is online and Bob's friend; Carol is online but unrelated to Bob.
    let (_a, _rx_a) = w.state.chat_hub.register(&w.alice.uuid);
    let (_c, _rx_c) = w.state.chat_hub.register(&w.carol.uuid);
    let asked = vec![w.alice.uuid.clone(), w.carol.uuid.clone(), "nobody-he-knows".to_string()];
    let online = w.chat().get_presence(w.rpc(GetPresenceRequest { person_uuids: asked }, Some(&w.bob))).await.unwrap().into_inner().online;
    assert_eq!(online, vec![w.alice.uuid.clone()], "Carol and the unknown uuid are not reported");
}

// ---------------------------------------------------------------- TC-006

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn the_stream_delivers_message_typing_read_and_pong_events_between_participants() {
    let w = world().await;
    let mut alice = w.open(Some(&w.alice)).await.unwrap();
    let mut bob = w.open(Some(&w.bob)).await.unwrap();

    alice.send(send_frame(&w.conversation, "oi", "m1")).await;
    let message = bob.next(|e| match e { Event::MessageNew(m) => Some(m), _ => None }).await;
    assert_eq!(message.conversation_uuid, w.conversation);
    let stored = message.message.unwrap();
    assert_eq!(stored.body, "oi");
    let messages = w.messages(&w.bob).await;
    assert_eq!(messages.len(), 1);
    assert_eq!(messages[0].client_message_id, "m1");

    alice.send(Frame::Typing(TypingFrame { conversation_uuid: w.conversation.clone() })).await;
    assert_eq!(bob.next(|e| match e { Event::Typing(t) => Some(t), _ => None }).await.person_uuid, w.alice.uuid);

    bob.send(Frame::Read(ReadFrame { conversation_uuid: w.conversation.clone(), last_read_message_uuid: stored.uuid.clone() })).await;
    let read = alice.next(|e| match e { Event::MessageRead(r) => Some(r), _ => None }).await;
    assert_eq!((read.person_uuid, read.last_read_message_uuid), (w.bob.uuid.clone(), stored.uuid));

    alice.send(Frame::Ping(PingFrame {})).await;
    alice.next(|e| match e { Event::Pong(_) => Some(()), _ => None }).await;
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn an_outsider_gets_an_error_event_and_nobody_else_gets_anything() {
    let w = world().await;
    let mut alice = w.open(Some(&w.alice)).await.unwrap();
    let mut bob = w.open(Some(&w.bob)).await.unwrap();
    let mut carol = w.open(Some(&w.carol)).await.unwrap();

    carol.send(send_frame(&w.conversation, "let me in", "x1")).await;
    carol.next(|e| match e { Event::Error(error) => Some(error), _ => None }).await;
    assert!(alice.stays_quiet().await && bob.stays_quiet().await, "the participants see no event for the frame");
    assert!(w.messages(&w.alice).await.is_empty(), "nothing from Carol is stored");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn a_reopened_stream_misses_nothing_stored() {
    let w = world().await;
    let mut alice = w.open(Some(&w.alice)).await.unwrap();
    let bob = w.open(Some(&w.bob)).await.unwrap();
    drop(bob);
    tokio::time::sleep(Duration::from_millis(200)).await;

    alice.send(send_frame(&w.conversation, "while you were away", "m2")).await;
    alice.next(|e| match e { Event::MessageNew(m) => Some(m), _ => None }).await;
    let _bob_again = w.open(Some(&w.bob)).await.unwrap();
    let bodies: Vec<String> = w.messages(&w.bob).await.into_iter().map(|m| m.body).collect();
    assert_eq!(bodies, vec!["while you were away".to_string()]);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn the_stream_refuses_missing_credentials_and_missing_consent() {
    let w = world_with(true).await;
    assert_eq!(w.open(None).await.err().map(|s| s.code()), Some(Code::Unauthenticated));
    assert_eq!(w.open(Some(&w.carol)).await.err().map(|s| s.code()), Some(Code::PermissionDenied), "consent is checked at open");
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL"]
async fn the_stream_ends_when_the_token_expires_and_a_fresh_token_reopens_it() {
    let w = world().await;
    let short = person_with_lifetime(8204, "dave", 2);
    let mut stream = w.open(Some(&short)).await.unwrap();
    let ended = tokio::time::timeout(Duration::from_secs(6), stream.events.message()).await.expect("closed within seconds of the expiry");
    assert_eq!(ended.err().map(|s| s.code()), Some(Code::Unauthenticated));
    assert!(w.open(Some(&w.alice)).await.is_ok(), "a fresh token opens a new stream");
}
