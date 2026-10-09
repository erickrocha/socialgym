mod support;

use business::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use business::gateway::mention_notification_gateway::MentionNotificationGateway;
use business::gateway::post_gateway::PostGateway;
use business::gateway::push_device_gateway::PushDeviceGateway;
use business::repositories::repository::Repository;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use business::use_cases::post_use_case::PostUseCase;
use domain::business_error::BusinessErrorKind;
use domain::comment::Comment;
use domain::enums::ReactionType;
use domain::enums::Visibility;
use domain::evolution_check_in::EvolutionCheckIn;
use domain::in_app_notification::InAppNotification;
use domain::mention::Mention;
use domain::mention_notification_event::MentionNotificationEvent;
use domain::post::Post;
use domain::reaction::Reaction;
use domain::user::User;
use futures::TryStreamExt;
use mongodb::{Client, Database, bson::doc};

/// Points the gRPC client at the in-process workout stand-in for the length of one test, then
/// restores the real fixture's settings so the tests that need the live Workout are unaffected.
struct StubGrpc(Vec<(&'static str, Option<String>)>);

impl StubGrpc {
    async fn start(friends: &[&str]) -> Self {
        let keys = ["GRPC_PROTOCOL", "GRPC_HOST", "GRPC_PORT", "GRPC_USE_TLS"];
        let saved = keys.iter().map(|k| (*k, std::env::var(k).ok())).collect();
        let stub = support::start_stub().await;
        *stub.friends.lock().unwrap() = friends.iter().map(|f| f.to_string()).collect();
        Self(saved)
    }
}

impl Drop for StubGrpc {
    fn drop(&mut self) {
        for (key, value) in &self.0 {
            unsafe {
                match value {
                    Some(v) => std::env::set_var(key, v),
                    None => std::env::remove_var(key),
                }
            }
        }
    }
}

async fn test_database() -> Database {
    let url = std::env::var("TEST_MONGO_URL")
        .expect("TEST_MONGO_URL must point to a disposable MongoDB database");
    let client = Client::with_uri_str(url).await.unwrap();
    client.database("timeline_test")
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_evolution_checkin_gateway_acceptance() {
    let database = test_database().await;
    let collection = database.collection::<EvolutionCheckIn>("evolutions");
    collection.delete_many(doc! {}).await.unwrap();

    let start = mongodb::bson::DateTime::from_millis(1_000);
    let in_range = EvolutionCheckIn::new(
        "c006-evolution-in-range".to_string(),
        "person-c006".to_string(),
        start,
        Some("Weekly progress".to_string()),
        Visibility::Private,
        None,
        None,
    );
    let out_of_range = EvolutionCheckIn::new(
        "c006-evolution-out-of-range".to_string(),
        "person-c006".to_string(),
        mongodb::bson::DateTime::from_millis(10_000),
        None,
        Visibility::Private,
        None,
        None,
    );

    let gateway = EvolutionCheckInGateway::new(&database);
    gateway.persist(in_range).await.unwrap();
    gateway.persist(out_of_range).await.unwrap();

    let results = gateway
        .find_all_by_person_uuid(
            "person-c006".to_string(),
            mongodb::bson::DateTime::from_millis(500),
            mongodb::bson::DateTime::from_millis(2_000),
        )
        .await;
    assert_eq!(results.len(), 1);
    assert_eq!(results[0].uuid, "c006-evolution-in-range");
    assert_eq!(results[0].person_uuid, "person-c006");
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_notification_pipeline_acceptance() {
    let database = test_database().await;
    database
        .collection::<MentionNotificationEvent>("mention_notification_events")
        .delete_many(doc! {})
        .await
        .unwrap();
    database
        .collection::<InAppNotification>("in_app_notifications")
        .delete_many(doc! {})
        .await
        .unwrap();

    let gateway = MentionNotificationGateway::new(&database);
    let event = MentionNotificationEvent::new(
        "c006-event:recipient".to_string(),
        "post".to_string(),
        "c006-post".to_string(),
        Some("c006-post".to_string()),
        None,
        42,
        "author-c006".to_string(),
        "Author".to_string(),
        "recipient-c006".to_string(),
        "mentioned you".to_string(),
    );

    MentionNotificationUseCase::enqueue(&database, vec![event.clone()])
        .await
        .unwrap();
    MentionNotificationUseCase::enqueue(&database, vec![event])
        .await
        .unwrap();

    let pending = gateway.list_pending(20).await.unwrap();
    assert_eq!(pending.len(), 1);
    assert!(gateway.claim_pending("c006-event:recipient").await.unwrap());
    assert!(!gateway.claim_pending("c006-event:recipient").await.unwrap());

    let notification = InAppNotification::from_mention_event(
        "c006-event:recipient".to_string(),
        "recipient-c006".to_string(),
        "author-c006".to_string(),
        "Author".to_string(),
        Some("c006-post".to_string()),
        None,
        "post".to_string(),
        "c006-post".to_string(),
        "mentioned you".to_string(),
    );
    gateway
        .persist_in_app_notification(notification.clone())
        .await
        .unwrap();
    gateway
        .persist_in_app_notification(notification)
        .await
        .unwrap();
    gateway
        .mark_processed("c006-event:recipient")
        .await
        .unwrap();

    let unread =
        MentionNotificationUseCase::list_notifications(&database, "recipient-c006", true, 50)
            .await
            .unwrap();
    assert_eq!(unread.len(), 1);
    assert!(!unread[0].read);

    assert!(
        !MentionNotificationUseCase::mark_as_read(
            &database,
            "other-recipient",
            "c006-event:recipient",
        )
        .await
        .unwrap()
    );
    assert!(
        MentionNotificationUseCase::mark_as_read(
            &database,
            "recipient-c006",
            "c006-event:recipient",
        )
        .await
        .unwrap()
    );
    assert!(
        MentionNotificationUseCase::list_notifications(&database, "recipient-c006", true, 50,)
            .await
            .unwrap()
            .is_empty()
    );

    gateway
        .mark_failed("c006-event:recipient", 5, "terminal test failure")
        .await
        .unwrap();
    let failed = database
        .collection::<MentionNotificationEvent>("mention_notification_events")
        .find_one(doc! { "_id": "c006-event:recipient" })
        .await
        .unwrap()
        .unwrap();
    assert_eq!(failed.status, "Failed");
    assert_eq!(failed.retry_count, 5);
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_push_device_transfer_rotation_acceptance() {
    let database = test_database().await;
    let collection = database.collection::<domain::push_device::PushDevice>("push_devices");
    collection
        .delete_many(doc! {
            "$or": [
                { "_id": { "$in": ["c006-device-a", "c006-device-b"] } },
                { "registrationToken": { "$in": ["c006-token-1", "c006-token-2"] } },
            ]
        })
        .await
        .unwrap();
    collection
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "registrationToken": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
        )
        .await
        .unwrap();
    collection
        .create_index(
            mongodb::IndexModel::builder()
                .keys(doc! { "deviceUuid": 1 })
                .options(
                    mongodb::options::IndexOptions::builder()
                        .unique(true)
                        .build(),
                )
                .build(),
        )
        .await
        .unwrap();

    PushDeviceGateway::register(
        &database,
        "c006-device-a",
        "c006-owner-a",
        "android",
        "c006-token-1",
    )
    .await
    .unwrap();
    PushDeviceGateway::register(
        &database,
        "c006-device-a",
        "c006-owner-a",
        "android",
        "c006-token-2",
    )
    .await
    .unwrap();

    let rotated = PushDeviceGateway::find_all_for_person(&database, "c006-owner-a")
        .await
        .unwrap();
    assert_eq!(rotated.len(), 1);
    assert_eq!(rotated[0].registration_token, "c006-token-2");

    PushDeviceGateway::register(
        &database,
        "c006-device-b",
        "c006-owner-b",
        "ios",
        "c006-token-2",
    )
    .await
    .unwrap();
    assert!(
        PushDeviceGateway::find_all_for_person(&database, "c006-owner-a")
            .await
            .unwrap()
            .is_empty()
    );
    let transferred = PushDeviceGateway::find_all_for_person(&database, "c006-owner-b")
        .await
        .unwrap();
    assert_eq!(transferred.len(), 1);
    assert_eq!(transferred[0].platform, "ios");

    assert!(
        !PushDeviceGateway::remove_owned(&database, "c006-device-b", "c006-owner-a")
            .await
            .unwrap()
    );
    assert!(
        PushDeviceGateway::remove_owned(&database, "c006-device-b", "c006-owner-b")
            .await
            .unwrap()
    );
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_friendship_notification_persistence_is_idempotent() {
    let database = test_database().await;
    let collection = database.collection::<InAppNotification>("in_app_notifications");
    collection
        .delete_one(doc! { "_id": "c006-friendship-event" })
        .await
        .unwrap();
    let gateway = MentionNotificationGateway::new(&database);
    let notification = InAppNotification::from_friendship_event(
        "c006-friendship-event".to_string(),
        "FriendRequestCreated".to_string(),
        "c006-recipient".to_string(),
        "c006-actor".to_string(),
        "c006-friendship".to_string(),
        "Someone sent you a friend request.".to_string(),
    );

    gateway
        .persist_friendship_notification(notification.clone())
        .await
        .unwrap();
    gateway
        .persist_friendship_notification(notification)
        .await
        .unwrap();

    let unread =
        MentionNotificationUseCase::list_notifications(&database, "c006-recipient", true, 50)
            .await
            .unwrap();
    let matching: Vec<_> = unread
        .into_iter()
        .filter(|notification| notification.uuid == "c006-friendship-event")
        .collect();
    assert_eq!(matching.len(), 1);
    assert_eq!(
        matching[0].target_type.as_deref(),
        Some("friendship_request")
    );
    assert_eq!(matching[0].target_uuid.as_deref(), Some("c006-friendship"));
    assert_eq!(matching[0].push_status.as_deref(), Some("Pending"));
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_social_interaction_producers_acceptance() {
    let _grpc = StubGrpc::start(&["c006-tc007-owner"]).await;
    let database = test_database().await;
    let post_ids = vec!["c006-tc007-post", "c006-tc007-self-post"];
    let notification_ids = vec![
        "c006-tc007-comment:comment:c006-tc007-owner",
        "c006-tc007-reaction:reaction:c006-tc007-owner",
    ];
    database
        .collection::<Post>("posts")
        .delete_many(doc! { "_id": { "$in": &post_ids } })
        .await
        .unwrap();
    database
        .collection::<InAppNotification>("in_app_notifications")
        .delete_many(doc! { "_id": { "$in": &notification_ids } })
        .await
        .unwrap();

    let owner_post = Post::updated(
        post_ids[0].to_string(),
        1,
        "c006-tc007-owner".to_string(),
        "Owner".to_string(),
        None,
        None,
        "Owner post".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    PostGateway::new(&database)
        .persist(owner_post)
        .await
        .unwrap();

    let actor = User::new(
        "Actor".to_string(),
        "c006-tc007-actor@example.com".to_string(),
        "c006-tc007-user".to_string(),
        2,
        "c006-tc007-actor".to_string(),
        String::new(),
        None,
    );
    let comment = Comment::new(
        "c006-tc007-comment".to_string(),
        post_ids[0].to_string(),
        "untrusted-author".to_string(),
        "Untrusted".to_string(),
        None,
        None,
        "A comment".to_string(),
        None,
        Vec::new(),
    );
    PostUseCase::add_comment(&database, &actor, post_ids[0].to_string(), comment)
        .await
        .unwrap();

    let owner_notifications =
        MentionNotificationUseCase::list_notifications(&database, "c006-tc007-owner", false, 50)
            .await
            .unwrap();
    assert_eq!(owner_notifications.len(), 1);
    assert_eq!(owner_notifications[0].notification_type, "Comment");
    assert_eq!(owner_notifications[0].actor_person_uuid, "c006-tc007-actor");

    for (reaction_uuid, reaction_type) in [
        ("c006-tc007-reaction", ReactionType::Like),
        ("c006-tc007-reaction-update", ReactionType::Love),
    ] {
        PostUseCase::add_reaction(
            &database,
            &actor,
            post_ids[0].to_string(),
            Reaction::new(
                reaction_uuid.to_string(),
                String::new(),
                String::new(),
                reaction_type,
            ),
        )
        .await
        .unwrap();
    }
    PostUseCase::remove_reaction(
        &database,
        post_ids[0].to_string(),
        actor.person_id,
        actor.person_uuid.clone(),
    )
    .await
    .unwrap();

    let owner_notifications =
        MentionNotificationUseCase::list_notifications(&database, "c006-tc007-owner", false, 50)
            .await
            .unwrap();
    assert_eq!(owner_notifications.len(), 2);
    assert_eq!(
        owner_notifications
            .iter()
            .filter(|notification| notification.notification_type == "Reaction")
            .count(),
        1
    );

    let self_post = Post::updated(
        post_ids[1].to_string(),
        2,
        actor.person_uuid.clone(),
        actor.name.clone(),
        None,
        None,
        "Self post".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    PostGateway::new(&database)
        .persist(self_post)
        .await
        .unwrap();
    let self_comment = Comment::new(
        "c006-tc007-self-comment".to_string(),
        post_ids[1].to_string(),
        actor.person_uuid.clone(),
        actor.name.clone(),
        None,
        None,
        "Self comment".to_string(),
        None,
        Vec::new(),
    );
    PostUseCase::add_comment(&database, &actor, post_ids[1].to_string(), self_comment)
        .await
        .unwrap();
    PostUseCase::add_reaction(
        &database,
        &actor,
        post_ids[1].to_string(),
        Reaction::new(
            "c006-tc007-self-reaction".to_string(),
            String::new(),
            String::new(),
            ReactionType::Like,
        ),
    )
    .await
    .unwrap();
    assert!(
        MentionNotificationUseCase::list_notifications(&database, &actor.person_uuid, false, 50,)
            .await
            .unwrap()
            .is_empty()
    );

    database
        .collection::<Post>("posts")
        .delete_many(doc! { "_id": { "$in": &post_ids } })
        .await
        .unwrap();
    database
        .collection::<InAppNotification>("in_app_notifications")
        .delete_many(doc! { "_id": { "$in": &notification_ids } })
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL, an authenticated Workout gRPC fixture, and a disposable MongoDB database"]
async fn c006_eligible_comment_mentions_prefer_mention_for_the_post_owner() {
    let _ = rustls::crypto::ring::default_provider().install_default();
    let database = test_database().await;
    let post_uuid = "c006-tc007-mention-post";
    let comment_uuid = "c006-tc007-mention-comment";
    let event_ids = vec![
        "c006-tc007-mention-comment:00000000-0000-0000-0000-000000000062".to_string(),
        "c006-tc007-mention-comment:00000000-0000-0000-0000-000000000099".to_string(),
    ];
    database
        .collection::<Post>("posts")
        .delete_one(doc! { "_id": post_uuid })
        .await
        .unwrap();
    database
        .collection::<MentionNotificationEvent>("mention_notification_events")
        .delete_many(doc! { "_id": { "$in": &event_ids } })
        .await
        .unwrap();
    database
        .collection::<InAppNotification>("in_app_notifications")
        .delete_many(doc! { "_id": { "$in": &event_ids } })
        .await
        .unwrap();

    PostGateway::new(&database)
        .persist(Post::updated(
            post_uuid.to_string(),
            2,
            "00000000-0000-0000-0000-000000000062".to_string(),
            "Receiver Person".to_string(),
            None,
            None,
            "Owner post".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        ))
        .await
        .unwrap();

    let actor = User::new(
        "Sender Person".to_string(),
        "c006-sender@example.test".to_string(),
        "10000000-0000-0000-0000-000000000061".to_string(),
        1,
        "00000000-0000-0000-0000-000000000061".to_string(),
        "default".to_string(),
        None,
    );
    let now = chrono::Utc::now().timestamp();
    let claims = serde_json::json!({
        "sub": actor.email,
        "exp": now + 3600,
        "iat": now,
        "jti": "c006-tc007-mention-token",
        "uuid": actor.uuid,
        "name": actor.name,
        "person_id": actor.person_id,
        "person_uuid": actor.person_uuid,
        "person_object_key": actor.person_object_key,
        "active_business_profile_id": null,
        "active_business_profile_uuid": null
    });
    let secret = std::env::var("ACCESS_TOKEN_SECRET")
        .expect("ACCESS_TOKEN_SECRET must match the disposable Workout gRPC fixture");
    let token = jsonwebtoken::encode(
        &jsonwebtoken::Header::new(jsonwebtoken::Algorithm::HS512),
        &claims,
        &jsonwebtoken::EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap();
    let accepted_friends =
        business::commons::token_context::with_forwarded_token(Some(token.clone()), async {
            business::gateway::friend_gateway::FriendGateway::new(
                business::commons::grpc_config::GrpcConfig::build_endpoint(),
            )
            .find_friend_uuids(actor.person_id, &actor.person_uuid)
            .await
            .unwrap()
        })
        .await;
    assert_eq!(
        accepted_friends,
        vec!["00000000-0000-0000-0000-000000000062".to_string()]
    );
    let comment = Comment::new(
        comment_uuid.to_string(),
        post_uuid.to_string(),
        String::new(),
        String::new(),
        None,
        None,
        "Owner and non-friend mention".to_string(),
        None,
        vec![
            Mention {
                name: "Receiver Person".to_string(),
                mentioned_uuid: "00000000-0000-0000-0000-000000000062".to_string(),
            },
            Mention {
                name: "Receiver Person".to_string(),
                mentioned_uuid: "00000000-0000-0000-0000-000000000062".to_string(),
            },
            Mention {
                name: "Unrelated Person".to_string(),
                mentioned_uuid: "00000000-0000-0000-0000-000000000099".to_string(),
            },
        ],
    );
    business::commons::token_context::with_forwarded_token(Some(token), async {
        PostUseCase::add_comment(&database, &actor, post_uuid.to_string(), comment)
            .await
            .unwrap();
    })
    .await;

    let events = database
        .collection::<MentionNotificationEvent>("mention_notification_events")
        .find(doc! { "postUuid": post_uuid, "commentUuid": comment_uuid })
        .await
        .unwrap()
        .try_collect::<Vec<_>>()
        .await
        .unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(
        events[0].mentioned_person_uuid,
        "00000000-0000-0000-0000-000000000062"
    );
    assert_eq!(events[0].uuid, event_ids[0]);
    MentionNotificationUseCase::process_pending(&database, 20)
        .await
        .unwrap();

    let owner_notifications = MentionNotificationUseCase::list_notifications(
        &database,
        "00000000-0000-0000-0000-000000000062",
        false,
        50,
    )
    .await
    .unwrap();
    let owner_mentions: Vec<_> = owner_notifications
        .into_iter()
        .filter(|notification| notification.uuid == event_ids[0])
        .collect();
    assert_eq!(owner_mentions.len(), 1);
    assert_eq!(owner_mentions[0].notification_type, "Mention");
    assert_eq!(
        owner_mentions[0].comment_uuid.as_deref(),
        Some(comment_uuid)
    );

    database
        .collection::<Post>("posts")
        .delete_one(doc! { "_id": post_uuid })
        .await
        .unwrap();
    database
        .collection::<MentionNotificationEvent>("mention_notification_events")
        .delete_many(doc! { "_id": { "$in": &event_ids } })
        .await
        .unwrap();
    database
        .collection::<InAppNotification>("in_app_notifications")
        .delete_many(doc! { "_id": { "$in": &event_ids } })
        .await
        .unwrap();
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_invalid_push_token_removal_is_scoped_to_that_token() {
    let database = test_database().await;
    let collection = database.collection::<domain::push_device::PushDevice>("push_devices");
    collection
        .delete_many(doc! { "_id": { "$in": ["c006-invalid-device", "c006-valid-device"] } })
        .await
        .unwrap();
    PushDeviceGateway::register(
        &database,
        "c006-invalid-device",
        "c006-token-owner",
        "android",
        "c006-invalid-token",
    )
    .await
    .unwrap();
    PushDeviceGateway::register(
        &database,
        "c006-valid-device",
        "c006-token-owner",
        "ios",
        "c006-valid-token",
    )
    .await
    .unwrap();

    PushDeviceGateway::remove_invalid_token(&database, "c006-invalid-token")
        .await
        .unwrap();
    let remaining = PushDeviceGateway::find_all_for_person(&database, "c006-token-owner")
        .await
        .unwrap();
    assert_eq!(remaining.len(), 1);
    assert_eq!(remaining[0].registration_token, "c006-valid-token");
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_push_claim_is_exclusive_and_recovers_expired_lease() {
    let database = test_database().await;
    let collection = database.collection::<InAppNotification>("in_app_notifications");
    collection
        .delete_one(doc! { "_id": "c006-push-lease" })
        .await
        .unwrap();
    let gateway = MentionNotificationGateway::new(&database);
    let notification = InAppNotification::from_friendship_event(
        "c006-push-lease".to_string(),
        "FriendRequestCreated".to_string(),
        "c006-lease-recipient".to_string(),
        "c006-lease-actor".to_string(),
        "c006-lease-friendship".to_string(),
        "Someone sent you a friend request.".to_string(),
    );
    gateway
        .persist_friendship_notification(notification)
        .await
        .unwrap();

    assert!(
        gateway
            .claim_push_notification("c006-push-lease")
            .await
            .unwrap()
    );
    assert!(
        !gateway
            .claim_push_notification("c006-push-lease")
            .await
            .unwrap()
    );
    collection
        .update_one(
            doc! { "_id": "c006-push-lease" },
            doc! { "$set": {
                "pushClaimedAt": mongodb::bson::DateTime::from_millis(0),
            } },
        )
        .await
        .unwrap();
    assert!(
        gateway
            .claim_push_notification("c006-push-lease")
            .await
            .unwrap()
    );
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c005_post_comment_reaction_feed_acceptance() {
    let _grpc = StubGrpc::start(&[]).await;
    let database = test_database().await;
    let posts = PostGateway::new(&database);
    database
        .collection::<Post>("posts")
        .delete_many(doc! {})
        .await
        .unwrap();

    let mut post = Post::updated(
        "c005-post".to_string(),
        42,
        "actor-uuid".to_string(),
        "Actor".to_string(),
        None,
        None,
        "hello timeline".to_string(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    );
    post.created_at = mongodb::bson::DateTime::from_millis(42);
    let persisted = posts.persist(post).await.unwrap();
    assert_eq!(persisted.uuid, "c005-post");

    for (uuid, author_uuid) in [
        ("c005-friend-post", "friend-uuid"),
        ("c005-outsider-post", "outsider-uuid"),
    ] {
        let mut extra_post = Post::updated(
            uuid.to_string(),
            43,
            author_uuid.to_string(),
            "Other".to_string(),
            None,
            None,
            "another post".to_string(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
            Vec::new(),
        );
        extra_post.created_at = mongodb::bson::DateTime::from_millis(42);
        posts.persist(extra_post).await.unwrap();
    }

    let comment = Comment::new(
        "c005-comment".to_string(),
        "c005-post".to_string(),
        "commenter-uuid".to_string(),
        "Commenter".to_string(),
        None,
        None,
        "nice post".to_string(),
        None,
        Vec::new(),
    );
    let with_comment = posts.add_comment("c005-post", comment).await.unwrap();
    assert_eq!(with_comment.comments.len(), 1);

    let reaction = Reaction::new(
        "c005-reaction".to_string(),
        "reactor-uuid".to_string(),
        "Reactor".to_string(),
        ReactionType::Like,
    );
    let with_reaction = posts.add_reaction("c005-post", reaction).await.unwrap();
    assert_eq!(with_reaction.reactions.len(), 1);
    let replacement_reaction = Reaction::new(
        "c005-replacement-reaction".to_string(),
        "reactor-uuid".to_string(),
        "Reactor".to_string(),
        ReactionType::Love,
    );
    let with_replacement = posts
        .add_reaction("c005-post", replacement_reaction)
        .await
        .unwrap();
    assert_eq!(with_replacement.reactions.len(), 1);
    assert!(matches!(
        with_replacement.reactions[0].reaction_type,
        ReactionType::Love
    ));
    let other_reaction = Reaction::new(
        "c005-other-reaction".to_string(),
        "other-reactor-uuid".to_string(),
        "Other Reactor".to_string(),
        ReactionType::Like,
    );
    let with_other_reaction = posts
        .add_reaction("c005-post", other_reaction)
        .await
        .unwrap();
    assert_eq!(with_other_reaction.reactions.len(), 2);

    let feed = posts
        .find_feed(
            vec!["actor-uuid".to_string(), "friend-uuid".to_string()],
            0,
            1,
        )
        .await
        .unwrap();
    assert_eq!(feed.len(), 1);
    assert_eq!(feed[0].uuid, "c005-friend-post");
    let second_feed_page = posts
        .find_feed(
            vec!["actor-uuid".to_string(), "friend-uuid".to_string()],
            1,
            1,
        )
        .await
        .unwrap();
    assert_eq!(second_feed_page.len(), 1);
    assert_eq!(second_feed_page[0].uuid, "c005-post");

    let forbidden = PostUseCase::delete_owned(&database, "c005-post".to_string(), 0, "other-actor")
        .await
        .unwrap_err();
    assert_eq!(
        forbidden.kind,
        BusinessErrorKind::NotFound,
        "a stranger cannot tell the post exists"
    );
    assert!(posts.find_by_id("c005-post".to_string()).await.is_some());

    let without_reaction = posts
        .remove_reaction("c005-post", "reactor-uuid")
        .await
        .unwrap();
    assert_eq!(without_reaction.reactions.len(), 1);
    assert_eq!(
        without_reaction.reactions[0].author_id,
        "other-reactor-uuid"
    );

    PostUseCase::delete_owned(&database, "c005-post".to_string(), 0, "actor-uuid")
        .await
        .unwrap();
    assert!(posts.find_by_id("c005-post".to_string()).await.is_none());
    let missing = PostUseCase::delete_owned(&database, "c005-post".to_string(), 0, "actor-uuid")
        .await
        .unwrap_err();
    assert_eq!(missing.kind, BusinessErrorKind::NotFound);
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
async fn c006_concurrent_push_registrations_leave_one_document_per_token_and_device() {
    let database = test_database().await;
    let collection = database.collection::<domain::push_device::PushDevice>("push_devices");
    collection.delete_many(doc! {}).await.unwrap();
    for field in ["registrationToken", "deviceUuid"] {
        collection
            .create_index(
                mongodb::IndexModel::builder()
                    .keys(doc! { field: 1 })
                    .options(
                        mongodb::options::IndexOptions::builder()
                            .unique(true)
                            .build(),
                    )
                    .build(),
            )
            .await
            .unwrap();
    }

    // Regression: a device that already holds a token registers one owned by another device.
    // The stale token must be dropped before the document moves (unique deviceUuid).
    PushDeviceGateway::register(
        &database,
        "c006-move-a",
        "c006-move-owner-a",
        "android",
        "c006-move-token-a",
    )
    .await
    .unwrap();
    PushDeviceGateway::register(
        &database,
        "c006-move-b",
        "c006-move-owner-b",
        "android",
        "c006-move-token-b",
    )
    .await
    .unwrap();
    PushDeviceGateway::register(
        &database,
        "c006-move-b",
        "c006-move-owner-b",
        "android",
        "c006-move-token-a",
    )
    .await
    .unwrap();
    let moved = PushDeviceGateway::find_all_for_person(&database, "c006-move-owner-b")
        .await
        .unwrap();
    assert_eq!(moved.len(), 1);
    assert_eq!(moved[0].registration_token, "c006-move-token-a");
    assert!(
        PushDeviceGateway::find_all_for_person(&database, "c006-move-owner-a")
            .await
            .unwrap()
            .is_empty()
    );

    // Many installations racing to claim one token, repeated so the duplicate-key
    // recovery paths run regardless of scheduling.
    for round in 0..5 {
        let token = format!("c006-race-token-{round}");
        let results = futures::future::join_all((0..12).map(|i| {
            let database = database.clone();
            let token = token.clone();
            async move {
                PushDeviceGateway::register(
                    &database,
                    &format!("c006-race-device-{round}-{i}"),
                    &format!("c006-race-owner-{i}"),
                    "android",
                    &token,
                )
                .await
            }
        }))
        .await;
        assert!(
            results.iter().all(|r| r.is_ok()),
            "round {round}: {results:?}"
        );
        assert_eq!(
            collection
                .count_documents(doc! { "registrationToken": &token })
                .await
                .unwrap(),
            1
        );
    }

    // Installations that already hold a token all rotate to the same new one: the update
    // collides on the unique token index and must fall back to transferring it.
    for i in 0..12 {
        PushDeviceGateway::register(
            &database,
            &format!("c006-rotate-device-{i}"),
            &format!("c006-rotate-owner-{i}"),
            "ios",
            &format!("c006-old-token-{i}"),
        )
        .await
        .unwrap();
    }
    let results = futures::future::join_all((0..12).map(|i| {
        let database = database.clone();
        async move {
            PushDeviceGateway::register(
                &database,
                &format!("c006-rotate-device-{i}"),
                &format!("c006-rotate-owner-{i}"),
                "ios",
                "c006-shared-new-token",
            )
            .await
        }
    }))
    .await;
    assert!(results.iter().all(|r| r.is_ok()), "{results:?}");
    assert_eq!(
        collection
            .count_documents(doc! { "registrationToken": "c006-shared-new-token" })
            .await
            .unwrap(),
        1
    );
    assert!(
        collection
            .count_documents(doc! { "registrationToken": { "$regex": "^c006-old-token" } })
            .await
            .unwrap()
            <= 11
    );

    collection.delete_many(doc! {}).await.unwrap();
    collection.drop_indexes().await.unwrap();
}
