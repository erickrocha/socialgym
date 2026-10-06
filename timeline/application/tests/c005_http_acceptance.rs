use application::{AppState, routes::feed_routes::feed_route, routes::post_routes::post_routes};
use axum::{Router, body::Body, http::Request};
use business::proto::proto::person::person_service_server::{PersonService, PersonServiceServer};
use business::proto::proto::friend::friend_service_server::{FriendService, FriendServiceServer};
use business::proto::proto::friend::*;
use business::proto::proto::person::*;
use domain::access_token::Claims;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::{Client, bson::doc};
use serde_json::{Value, json};
use std::net::SocketAddr;
use tonic::{Request as GrpcRequest, Response, Status};
use tower::ServiceExt;

/// Stand-in for the integration service: every consent document is active and nobody has friends.
struct ConsentStub;

macro_rules! consent_stub {
    ($($name:ident($req:ty) -> $res:ty;)*) => {
        #[tonic::async_trait]
        impl PersonService for ConsentStub {
            async fn has_active_consent(
                &self,
                _: GrpcRequest<ConsentStatusRequest>,
            ) -> Result<Response<ConsentStatusResponse>, Status> {
                Ok(Response::new(ConsentStatusResponse { active: true, version: "test".into() }))
            }
            $(async fn $name(&self, _: GrpcRequest<$req>) -> Result<Response<$res>, Status> {
                Err(Status::unimplemented("not used by the C-005 acceptance"))
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

/// The owner and the other person are friends, so each can read the other's posts.
#[tonic::async_trait]
impl FriendService for ConsentStub {
    async fn get_friends(&self, request: GrpcRequest<FriendsRequest>) -> Result<Response<FriendsResponse>, Status> {
        let me = request.into_inner().uuid;
        let other = if me == "c005-owner" { "c005-other" } else { "c005-owner" };
        Ok(Response::new(FriendsResponse {
            friends: vec![Friend { person_uuid: me, friend_uuid: other.into(), ..Default::default() }],
        }))
    }
    async fn get_friend_page(&self, _: GrpcRequest<FriendPageRequest>) -> Result<Response<FriendPageResponse>, Status> { Err(Status::unimplemented("")) }
    async fn search_friends(&self, _: GrpcRequest<SearchFriendsRequest>) -> Result<Response<SearchFriendsResponse>, Status> { Err(Status::unimplemented("")) }
    async fn send_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn accept_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn deny_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn cancel_friend_request(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<Friend>, Status> { Err(Status::unimplemented("")) }
    async fn remove_friend(&self, _: GrpcRequest<FriendRequestRequest>) -> Result<Response<RemoveFriendResponse>, Status> { Err(Status::unimplemented("")) }
}

fn token(email: &str, person_id: i32, person_uuid: &str) -> String {
    let claims = Claims {
        sub: email.into(),
        exp: chrono::Utc::now().timestamp() + 3600,
        uuid: format!("user-{person_id}"),
        name: format!("Person {person_id}"),
        person_id,
        person_uuid: person_uuid.into(),
        person_object_key: String::new(),
        active_business_profile_id: None,
        active_business_profile_uuid: None,
    };
    encode(
        &Header::new(Algorithm::HS512),
        &claims,
        &EncodingKey::from_secret(b"c005-http-test-secret"),
    )
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

#[tokio::test]
#[ignore = "requires a disposable TEST_MONGO_URL targeting the timeline_test database"]
async fn c005_timeline_routes_enforce_identity_and_return_stable_outcomes() {
    let mongo_url = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    assert!(mongo_url.contains("/timeline_test"), "refusing to run against a non-test database");
    let database = Client::with_uri_str(mongo_url).await.unwrap().database("timeline_test");
    database.collection::<Value>("posts").delete_many(doc! {}).await.unwrap();

    // Plaintext gRPC consent stub on an ephemeral port.
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let addr: SocketAddr = listener.local_addr().unwrap();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(PersonServiceServer::new(ConsentStub))
            .add_service(FriendServiceServer::new(ConsentStub))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    unsafe {
        std::env::set_var("ACCESS_TOKEN_SECRET", "c005-http-test-secret");
        std::env::set_var("GRPC_PROTOCOL", "http");
        std::env::set_var("GRPC_HOST", "127.0.0.1");
        std::env::set_var("GRPC_PORT", addr.port().to_string());
        std::env::set_var("GRPC_USE_TLS", "false");
    }

    let state = AppState {
        database: std::sync::Arc::new(database),
        chat_hub: Default::default(),
    };
    let app = Router::new()
        .nest("/posts", post_routes(state.clone()))
        .nest("/feed", feed_route(state.clone()))
        .with_state(state);

    let owner = token("owner@c005.test", 5001, "c005-owner");
    let other = token("other@c005.test", 5002, "c005-other");

    // Invalid token -> 401.
    assert_eq!(call(&app, "GET", "/feed", "not-a-jwt", None).await.0, 401);

    // Create -> 201, attribution comes from the token, not the body.
    let (status, post) = call(&app, "POST", "/posts", &owner, Some(json!({
        "authorId": 999, "authorUuid": "spoofed", "authorName": "Spoofed", "content": "hello c005"
    }))).await;
    assert_eq!(status, 201, "{post}");
    assert_eq!(post["authorUuid"], "c005-owner");
    let id = post["uuid"].as_str().unwrap().to_string();

    // Validation -> 400 (media without third-party consent).
    let (status, _) = call(&app, "POST", "/posts", &owner, Some(json!({
        "authorId": 5001, "authorUuid": "c005-owner", "authorName": "x", "content": "m",
        "media": [{"url": "u", "mediaType": "IMAGE", "objectKey": "k"}]
    }))).await;
    assert_eq!(status, 400);

    // Feed for the owner includes the post.
    let (status, feed) = call(&app, "GET", "/feed?page=0", &owner, None).await;
    assert_eq!(status, 200);
    assert!(feed.as_array().unwrap().iter().any(|p| p["uuid"] == id.as_str()));

    // Comment and reaction -> 201; missing post -> 404.
    let (status, p) = call(&app, "POST", &format!("/posts/{id}/comments"), &other, Some(json!({
        "postUuid": "ignored", "authorUuid": "x", "authorName": "x", "content": "nice"
    }))).await;
    assert_eq!(status, 201, "{p}");
    assert_eq!(p["comments"][0]["postUuid"], id.as_str());
    let (status, _) = call(&app, "POST", &format!("/posts/{id}/reactions"), &other, Some(json!({
        "authorId": "x", "authorName": "x", "reactionType": "LIKE"
    }))).await;
    assert_eq!(status, 201);
    assert_eq!(call(&app, "POST", "/posts/missing-post/comments", &other, Some(json!({
        "postUuid": "m", "authorUuid": "x", "authorName": "x", "content": "c"
    }))).await.0, 404);

    // Non-owner delete -> 403; missing -> 404; owner delete -> 204.
    assert_eq!(call(&app, "DELETE", &format!("/posts/{id}"), &other, None).await.0, 403);
    assert_eq!(call(&app, "DELETE", "/posts/missing-post", &owner, None).await.0, 404);
    assert_eq!(call(&app, "DELETE", &format!("/posts/{id}"), &owner, None).await.0, 204);
}
