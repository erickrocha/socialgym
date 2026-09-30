use business::gateway::post_gateway::PostGateway;
use business::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use business::gateway::mention_notification_gateway::MentionNotificationGateway;
use business::repositories::repository::Repository;
use business::use_cases::post_use_case::PostUseCase;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use domain::business_error::BusinessErrorKind;
use domain::comment::Comment;
use domain::enums::ReactionType;
use domain::enums::Visibility;
use domain::evolution_check_in::EvolutionCheckIn;
use domain::in_app_notification::InAppNotification;
use domain::mention_notification_event::MentionNotificationEvent;
use domain::post::Post;
use domain::reaction::Reaction;
use mongodb::{Client, Database, bson::doc};

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
    gateway.mark_processed("c006-event:recipient").await.unwrap();

    let unread = MentionNotificationUseCase::list_notifications(
        &database,
        "recipient-c006",
        true,
        50,
    )
    .await
    .unwrap();
    assert_eq!(unread.len(), 1);
    assert!(!unread[0].read);

    assert!(
        MentionNotificationUseCase::mark_as_read(
            &database,
            "other-recipient",
            "c006-event:recipient",
        )
        .await
        .unwrap()
        == false
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
        MentionNotificationUseCase::list_notifications(
            &database,
            "recipient-c006",
            true,
            50,
        )
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
async fn c005_post_comment_reaction_feed_acceptance() {
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

    let forbidden = PostUseCase::delete_owned(&database, "c005-post".to_string(), "other-actor")
        .await
        .unwrap_err();
    assert_eq!(forbidden.kind, BusinessErrorKind::Forbidden);
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

    PostUseCase::delete_owned(&database, "c005-post".to_string(), "actor-uuid")
        .await
        .unwrap();
    assert!(posts.find_by_id("c005-post".to_string()).await.is_none());
    let missing = PostUseCase::delete_owned(&database, "c005-post".to_string(), "actor-uuid")
        .await
        .unwrap_err();
    assert_eq!(missing.kind, BusinessErrorKind::NotFound);
}
