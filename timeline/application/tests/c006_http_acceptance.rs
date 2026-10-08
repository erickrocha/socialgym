use application::{
    AppState,
    routes::{
        evolution_checkin_routes::evolution_checkin_routes,
        notification_routes::notification_routes, push_device_routes::push_device_routes,
    },
};
use axum::{Router, body::Body, http::Request};
use business::proto::proto::friend::friend_service_server::{FriendService, FriendServiceServer};
use business::proto::proto::friend::*;
use business::proto::proto::person::person_service_server::{PersonService, PersonServiceServer};
use business::proto::proto::person::*;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::{Client, bson::doc};
use serde_json::{Value, json};
use std::net::SocketAddr;
use std::sync::atomic::{AtomicBool, Ordering};
use tonic::{Request as GrpcRequest, Response, Status};
use tower::ServiceExt;

/// Health consent is granted unless a test flips this switch.
static CONSENT_ACTIVE: AtomicBool = AtomicBool::new(true);

/// Stand-in for the integration service: only `has_active_consent` is answered.
struct ConsentStub;

macro_rules! consent_stub {
    ($($name:ident($req:ty) -> $res:ty;)*) => {
        #[tonic::async_trait]
        impl PersonService for ConsentStub {
            async fn has_active_consent(
                &self,
                _: GrpcRequest<ConsentStatusRequest>,
            ) -> Result<Response<ConsentStatusResponse>, Status> {
                Ok(Response::new(ConsentStatusResponse {
                    active: CONSENT_ACTIVE.load(Ordering::SeqCst),
                    version: "test".into(),
                }))
            }
            $(async fn $name(&self, _: GrpcRequest<$req>) -> Result<Response<$res>, Status> {
                Err(Status::unimplemented("not used by the C-006 acceptance"))
            })*
        }
    };
}

consent_stub! {
    get_person(PersonIdRequest) -> PersonResponse;
    get_me(GetMeRequest) -> PersonResponse;
    search_mentionable_friends(SearchMentionableFriendsRequest) -> PeopleResponse;
    update_person(Person) -> PersonResponse;
    search_persons(PersonParams) -> PeopleResponse;
    update_person_info(business::proto::proto::person_info::PersonInfo) -> business::proto::proto::person_info::PersonInfo;
    add_person_address(business::proto::proto::person_address::PersonAddress) -> business::proto::proto::person_address::PersonAddress;
    update_person_address(business::proto::proto::person_address::PersonAddress) -> business::proto::proto::person_address::PersonAddress;
    remove_person_address(RemovePersonAddressRequest) -> RemovePersonAddressResponse;
    get_person_image_upload_url(PersonImageUploadRequest) -> PersonImageUploadResponse;
    delete_person_image(PersonImageRequest) -> DeletePersonImageResponse;
    has_role(RoleStatusRequest) -> RoleStatusResponse;
}

#[tonic::async_trait]
impl FriendService for ConsentStub {
    async fn get_friends(&self, _: GrpcRequest<FriendsRequest>) -> Result<Response<FriendsResponse>, Status> {
        Ok(Response::new(FriendsResponse { friends: vec![] }))
    }
    async fn get_friend_page(&self, _: GrpcRequest<FriendPageRequest>) -> Result<Response<FriendPageResponse>, Status> { Err(Status::unimplemented("")) }
    async fn search_friends(&self, _: GrpcRequest<SearchFriendsRequest>) -> Result<Response<SearchFriendsResponse>, Status> { Err(Status::unimplemented("")) }
    async fn send_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn accept_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn deny_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn cancel_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn remove_friend(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<RemoveFriendResponse>, Status> { Err(Status::unimplemented("")) }
    async fn get_friend_profile(&self, _: GrpcRequest<FriendProfileRequest>) -> Result<Response<FriendProfileResponse>, Status> { Err(Status::unimplemented("")) }
}

const SECRET: &str = "c006-http-test-secret";

fn token(person_id: i32, person_uuid: &str) -> String {
    let claims = Claims {
        sub: format!("{person_uuid}@c006.test"),
        exp: chrono::Utc::now().timestamp() + 3600,
        uuid: format!("user-{person_id}"),
        name: format!("Person {person_id}"),
        person_id,
        person_uuid: person_uuid.into(),
        person_object_key: String::new(),
        active_business_profile_id: None,
        active_business_profile_uuid: None,
    };
    encode(&Header::new(Algorithm::HS512), &claims, &EncodingKey::from_secret(SECRET.as_bytes()))
        .unwrap()
}

async fn call(app: &Router, method: &str, uri: &str, tok: &str, body: Option<Value>) -> (u16, Value) {
    let mut req = Request::builder()
        .method(method)
        .uri(uri)
        .header("authorization", format!("Bearer {tok}"));
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
    (status, serde_json::from_slice(&bytes).unwrap_or(Value::Null))
}

fn notification(key: &str, recipient: &str) -> Value {
    let now = mongodb::bson::DateTime::now();
    json!({
        "_id": key, "notificationType": "Mention", "recipientPersonUuid": recipient,
        "actorPersonUuid": "c006-actor", "actorName": "Actor", "postUuid": "post-1",
        "commentUuid": null, "entityType": "post", "entityUuid": "post-1", "snippet": "hi @you",
        "read": false, "createdAt": now, "updatedAt": now,
        "targetType": "post", "targetUuid": "post-1", "pushAttemptCount": 0,
        "pushCompletedDeviceUuids": []
    })
}

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL targeting the timeline_test database"]
async fn c006_notification_evolution_and_push_routes_enforce_identity_and_return_stable_outcomes() {
    let mongo_url = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(mongo_url.contains("/timeline_test"), "refusing to run against a non-test database");
    let database = Client::with_uri_str(mongo_url).await.unwrap().database("timeline_test");
    let notifications = database.collection::<Value>("in_app_notifications");
    let keys = ["c006-http-n1", "c006-http-n2", "c006-http-other"];
    notifications.delete_many(doc! { "_id": { "$in": keys.to_vec() } }).await.unwrap();
    database
        .collection::<Value>("evolutions")
        .delete_many(doc! { "personUuid": "c006-http-owner" })
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(PersonServiceServer::new(ConsentStub))
            .add_service(FriendServiceServer::new(ConsentStub))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", SECRET);
        std::env::set_var("GRPC_PROTOCOL", "http");
        std::env::set_var("GRPC_HOST", "127.0.0.1");
        std::env::set_var("GRPC_PORT", addr.port().to_string());
        std::env::set_var("GRPC_USE_TLS", "false");
    }

    let state = AppState {
        database: std::sync::Arc::new(database.clone()),
        chat_hub: Default::default(),
    };
    let app = Router::new()
        .nest("/notifications", notification_routes(state.clone()))
        .nest("/evolution-checkin", evolution_checkin_routes(state.clone()))
        .nest("/push-devices", push_device_routes(state.clone()))
        .with_state(state);

    let owner = token(6001, "c006-http-owner");
    let other = token(6002, "c006-http-other");

    // ---- Notifications: list / read, owner scoping and validation.
    notifications
        .insert_many([
            notification("c006-http-n1", "c006-http-owner"),
            notification("c006-http-n2", "c006-http-owner"),
            notification("c006-http-other", "c006-http-other"),
        ])
        .await
        .unwrap();

    assert_eq!(call(&app, "GET", "/notifications/c006-http-owner", "not-a-jwt", None).await.0, 401);
    // Reading another person's notifications is rejected before touching the database.
    assert_eq!(call(&app, "GET", "/notifications/c006-http-owner", &other, None).await.0, 403);
    assert_eq!(
        call(&app, "PUT", "/notifications/c006-http-owner/read/c006-http-n1", &other, None).await.0,
        403
    );

    let (status, list) = call(&app, "GET", "/notifications/c006-http-owner", &owner, None).await;
    assert_eq!(status, 200, "{list}");
    let list = list.as_array().unwrap();
    assert_eq!(list.len(), 2);
    assert!(list.iter().all(|n| n["recipientPersonUuid"] == "c006-http-owner"));

    // limit is clamped to [1, 100].
    let (_, limited) = call(&app, "GET", "/notifications/c006-http-owner?limit=0", &owner, None).await;
    assert_eq!(limited.as_array().unwrap().len(), 1);

    let (status, read) =
        call(&app, "PUT", "/notifications/c006-http-owner/read/c006-http-n1", &owner, None).await;
    assert_eq!((status, &read["read"]), (200, &json!(true)));
    let (_, unread) =
        call(&app, "GET", "/notifications/c006-http-owner?unread_only=true", &owner, None).await;
    let unread = unread.as_array().unwrap();
    assert_eq!(unread.len(), 1);
    assert_eq!(unread[0]["uuid"], "c006-http-n2");

    // A key that does not belong to the recipient is a 400, not a state change.
    assert_eq!(
        call(&app, "PUT", "/notifications/c006-http-owner/read/c006-http-other", &owner, None).await.0,
        400
    );
    assert_eq!(
        call(&app, "PUT", "/notifications/c006-http-owner/read/does-not-exist", &owner, None).await.0,
        400
    );

    // ---- Evolution check-ins: consent gate, ownership from the token, history window.
    let checkin = json!({
        "personUuid": "spoofed", "createdAt": "2026-05-23T16:47:28", "note": "weekly",
        "visibility": "Private",
        "composition": { "weight": 82.5, "bodyFatPct": 14.2, "muscleMassPct": 45.0, "visceralFat": 7 }
    });
    assert_eq!(call(&app, "POST", "/evolution-checkin", "not-a-jwt", Some(checkin.clone())).await.0, 401);

    CONSENT_ACTIVE.store(false, Ordering::SeqCst);
    assert_eq!(call(&app, "POST", "/evolution-checkin", &owner, Some(checkin.clone())).await.0, 403);
    CONSENT_ACTIVE.store(true, Ordering::SeqCst);

    let (status, saved) = call(&app, "POST", "/evolution-checkin", &owner, Some(checkin)).await;
    assert_eq!(status, 201, "{saved}");
    assert_eq!(saved["personUuid"], "c006-http-owner");

    let (status, history) = call(
        &app,
        "GET",
        "/evolution-checkin?startDate=2026-05-01T00:00:00&endDate=2026-06-01T00:00:00",
        &owner,
        None,
    )
    .await;
    assert_eq!(status, 200, "{history}");
    assert_eq!(history.as_array().unwrap().len(), 1);
    // Another person's history is empty: the owner always comes from the token.
    let (_, theirs) = call(
        &app,
        "GET",
        "/evolution-checkin?startDate=2026-05-01T00:00:00&endDate=2026-06-01T00:00:00",
        &other,
        None,
    )
    .await;
    assert!(theirs.as_array().unwrap().is_empty());
    // Default window (last 7 days) does not include the 2026-05 entry.
    let (status, recent) = call(&app, "GET", "/evolution-checkin", &owner, None).await;
    assert_eq!(status, 200);
    assert!(recent.as_array().unwrap().is_empty());

    // ---- Push devices: removal is owner-scoped.
    let device = "d2a7c810-2a10-4cab-8a9e-3df935d20c06";
    let register = json!({ "platform": "android", "registrationToken": "c006-http-token" });
    assert_eq!(call(&app, "PUT", &format!("/push-devices/{device}"), &owner, Some(register.clone())).await.0, 204);
    assert_eq!(call(&app, "PUT", "/push-devices/not-a-uuid", &owner, Some(register)).await.0, 400);
    assert_eq!(call(&app, "DELETE", &format!("/push-devices/{device}"), &other, None).await.0, 404);
    assert_eq!(call(&app, "DELETE", &format!("/push-devices/{device}"), &owner, None).await.0, 204);
    assert_eq!(call(&app, "DELETE", &format!("/push-devices/{device}"), &owner, None).await.0, 404);

    notifications.delete_many(doc! { "_id": { "$in": keys.to_vec() } }).await.unwrap();
    database
        .collection::<Value>("evolutions")
        .delete_many(doc! { "personUuid": "c006-http-owner" })
        .await
        .unwrap();
}
