//! C-006 mention enqueueing against a disposable MongoDB and a local stand-in for the
//! Workout friend service (no Workout stack needed).
use business::commons::token_context::with_forwarded_token;
use business::proto::proto::friend::friend_service_server::{FriendService, FriendServiceServer};
use business::proto::proto::friend::*;
use business::use_cases::mention_use_case::MentionUseCase;
use domain::comment::Comment;
use domain::mention::Mention;
use domain::mention_notification_event::MentionNotificationEvent;
use domain::post::Post;
use futures::TryStreamExt;
use mongodb::{Client, bson::doc};
use tonic::{Request, Response, Status};

/// "author-1" is friends with "friend-1" only.
struct Friends;

#[tonic::async_trait]
impl FriendService for Friends {
    async fn get_friends(
        &self,
        _: Request<FriendsRequest>,
    ) -> Result<Response<FriendsResponse>, Status> {
        Ok(Response::new(FriendsResponse {
            friends: vec![Friend {
                person_uuid: "author-1".into(),
                friend_uuid: "friend-1".into(),
                ..Default::default()
            }],
        }))
    }
    async fn get_friend_page(
        &self,
        _: Request<FriendPageRequest>,
    ) -> Result<Response<FriendPageResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn search_friends(
        &self,
        _: Request<SearchFriendsRequest>,
    ) -> Result<Response<SearchFriendsResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn send_friend_request(
        &self,
        _: Request<FriendRequestRequest>,
    ) -> Result<Response<Friend>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn accept_friend_request(
        &self,
        _: Request<FriendRequestRequest>,
    ) -> Result<Response<Friend>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn deny_friend_request(
        &self,
        _: Request<FriendRequestRequest>,
    ) -> Result<Response<Friend>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn cancel_friend_request(
        &self,
        _: Request<FriendRequestRequest>,
    ) -> Result<Response<Friend>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn remove_friend(
        &self,
        _: Request<FriendRequestRequest>,
    ) -> Result<Response<RemoveFriendResponse>, Status> {
        Err(Status::unimplemented(""))
    }
    async fn get_friend_profile(
        &self,
        _: Request<FriendProfileRequest>,
    ) -> Result<Response<FriendProfileResponse>, Status> {
        Err(Status::unimplemented(""))
    }
}

fn mention(uuid: &str) -> Mention {
    Mention {
        name: uuid.into(),
        mentioned_uuid: uuid.into(),
    }
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_mentions_in_posts_and_comments_enqueue_one_event_per_eligible_person() {
    let url = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
    let database = Client::with_uri_str(url)
        .await
        .unwrap()
        .database("timeline_test");
    let events = database.collection::<MentionNotificationEvent>("mention_notification_events");
    let ids = [
        "c006-mu-post:friend-1",
        "c006-mu-post:stranger",
        "c006-mu-comment:friend-1",
        "c006-mu-comment:stranger",
    ];
    events
        .delete_many(doc! { "_id": { "$in": ids.to_vec() } })
        .await
        .unwrap();

    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    tokio::spawn(
        tonic::transport::Server::builder()
            .add_service(FriendServiceServer::new(Friends))
            .serve_with_incoming(tokio_stream::wrappers::TcpListenerStream::new(listener)),
    );
    unsafe {
        std::env::set_var("GRPC_PROTOCOL", "http");
        std::env::set_var("GRPC_HOST", "127.0.0.1");
        std::env::set_var("GRPC_PORT", port.to_string());
        std::env::set_var("GRPC_USE_TLS", "false");
    }

    // Posts: every mention is queued (the post author's audience is validated at creation).
    let long_text = "x".repeat(200);
    let post = Post::updated(
        "c006-mu-post".into(),
        1,
        "author-1".into(),
        "Author".into(),
        None,
        None,
        long_text,
        vec![],
        vec![],
        vec![],
        vec![mention("friend-1"), mention("stranger")],
    );
    MentionUseCase::enqueue_mentions_from_post(&database, 1, &post)
        .await
        .unwrap();
    let queued = events
        .find(doc! { "postUuid": "c006-mu-post" })
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(queued.len(), 2);
    assert!(
        queued.iter().all(|e| e.entity_type == "post"
            && e.status == "Pending"
            && e.snippet.ends_with("..."))
    );

    // Re-enqueueing the same post is idempotent (event id = post:person).
    MentionUseCase::enqueue_mentions_from_post(&database, 1, &post)
        .await
        .unwrap();
    assert_eq!(
        events
            .count_documents(doc! { "postUuid": "c006-mu-post" })
            .await
            .unwrap(),
        2
    );

    // Comments: only friends of the comment author are eligible; duplicates collapse.
    let comment = Comment::new(
        "c006-mu-comment".into(),
        "c006-mu-post".into(),
        "author-1".into(),
        "Author".into(),
        None,
        None,
        "hello".into(),
        None,
        vec![
            mention("friend-1"),
            mention("friend-1"),
            mention("stranger"),
        ],
    );
    with_forwarded_token(Some("test-token".into()), async {
        MentionUseCase::enqueue_mentions_from_comment(&database, 1, &post, &comment)
            .await
            .unwrap();
        MentionUseCase::enqueue_mentions_from_comment(&database, 1, &post, &comment)
            .await
            .unwrap();
    })
    .await;
    let queued = events
        .find(doc! { "commentUuid": "c006-mu-comment" })
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(queued.len(), 1, "{queued:?}");
    assert_eq!(queued[0].mentioned_person_uuid, "friend-1");
    assert_eq!(queued[0].entity_type, "comment");
    assert_eq!(queued[0].post_uuid.as_deref(), Some("c006-mu-post"));

    // No mentions -> nothing queued and no friend lookup needed to fail the call.
    let empty = Comment::new(
        "c006-mu-empty".into(),
        "c006-mu-post".into(),
        "author-1".into(),
        "Author".into(),
        None,
        None,
        "hi".into(),
        None,
        vec![],
    );
    with_forwarded_token(
        Some("test-token".into()),
        MentionUseCase::enqueue_mentions_from_comment(&database, 1, &post, &empty),
    )
    .await
    .unwrap();

    events
        .delete_many(doc! { "_id": { "$in": ids.to_vec() } })
        .await
        .unwrap();
}
