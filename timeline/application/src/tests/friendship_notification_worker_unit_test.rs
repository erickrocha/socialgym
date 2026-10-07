use super::{consume_batch, notification_for_event};
use aws_config::BehaviorVersion;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use domain::friendship_notification_event::FriendshipNotificationEvent;
use mongodb::{Client as MongoClient, bson::doc};

#[test]
fn friendship_event_creates_generic_recipient_notification() {
    let notification = notification_for_event(FriendshipNotificationEvent {
        event_uuid: "event-1".to_string(),
        event_type: "friend_request_created".to_string(),
        friendship_uuid: "relationship-1".to_string(),
        actor_person_uuid: "actor-1".to_string(),
        recipient_person_uuid: "recipient-1".to_string(),
        occurred_at: "2026-10-02T00:00:00Z".to_string(),
    })
    .unwrap();

    assert_eq!(notification.uuid, "event-1");
    assert_eq!(notification.recipient_person_uuid, "recipient-1");
    assert_eq!(
        notification.target_type.as_deref(),
        Some("friendship_request")
    );
    assert_eq!(notification.target_uuid.as_deref(), Some("relationship-1"));
    assert!(!notification.snippet.contains("health"));
}

#[tokio::test]
#[ignore = "requires TEST_MONGO_URL and a disposable LocalStack FIFO queue"]
async fn c006_friendship_consumer_materializes_and_acknowledges_idempotently() {
    let mongo_url = std::env::var("TEST_MONGO_URL")
        .expect("TEST_MONGO_URL must point to the disposable timeline_test database");
    let queue_url = std::env::var("AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL")
        .expect("AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL must point to the test FIFO queue");
    let endpoint = std::env::var("AWS_ENDPOINT_URL")
        .expect("AWS_ENDPOINT_URL must point to LocalStack");
    let mongo_client = MongoClient::with_uri_str(mongo_url).await.unwrap();
    let database = mongo_client.database("timeline_test");
    let notification_ids = vec![
        "c006-tc008-created".to_string(),
        "c006-tc008-accepted".to_string(),
    ];
    database
        .collection::<domain::in_app_notification::InAppNotification>("in_app_notifications")
        .delete_many(doc! { "_id": { "$in": &notification_ids } })
        .await
        .unwrap();

    let shared_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let sqs_config = aws_sdk_sqs::config::Builder::from(&shared_config)
        .endpoint_url(endpoint)
        .build();
    let client = aws_sdk_sqs::Client::from_conf(sqs_config);
    let events = [
        FriendshipNotificationEvent {
            event_uuid: notification_ids[0].clone(),
            event_type: "friend_request_created".to_string(),
            friendship_uuid: "c006-tc008-friendship".to_string(),
            actor_person_uuid: "c006-tc008-actor".to_string(),
            recipient_person_uuid: "c006-tc008-recipient".to_string(),
            occurred_at: "2026-10-04T00:00:00Z".to_string(),
        },
        FriendshipNotificationEvent {
            event_uuid: notification_ids[0].clone(),
            event_type: "friend_request_created".to_string(),
            friendship_uuid: "c006-tc008-friendship".to_string(),
            actor_person_uuid: "c006-tc008-actor".to_string(),
            recipient_person_uuid: "c006-tc008-recipient".to_string(),
            occurred_at: "2026-10-04T00:00:00Z".to_string(),
        },
        FriendshipNotificationEvent {
            event_uuid: notification_ids[1].clone(),
            event_type: "friend_request_accepted".to_string(),
            friendship_uuid: "c006-tc008-friendship".to_string(),
            actor_person_uuid: "c006-tc008-recipient".to_string(),
            recipient_person_uuid: "c006-tc008-actor".to_string(),
            occurred_at: "2026-10-04T00:00:01Z".to_string(),
        },
    ];
    // FIFO queues drop repeated deduplication ids for 5 minutes, even after the message was
    // consumed; make them unique per run so the test can be repeated. Idempotency under test
    // comes from the duplicate `event_uuid`, not from the queue's deduplication.
    let run_id = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_nanos();
    for (index, event) in events.iter().enumerate() {
        let body = serde_json::json!({
            "eventUuid": &event.event_uuid,
            "eventType": &event.event_type,
            "friendshipUuid": &event.friendship_uuid,
            "actorPersonUuid": &event.actor_person_uuid,
            "recipientPersonUuid": &event.recipient_person_uuid,
            "occurredAt": &event.occurred_at,
        });
        client
            .send_message()
            .queue_url(&queue_url)
            .message_body(body.to_string())
            // Unique per run: an in-flight message from a prior run would otherwise block the group.
            .message_group_id(format!("c006-tc008-friendship-{run_id}"))
            .message_deduplication_id(format!("c006-tc008-delivery-{run_id}-{index}"))
            .send()
            .await
            .unwrap();
    }

    assert_eq!(consume_batch(&database, &client, &queue_url).await.unwrap(), 3);
    let notifications = MentionNotificationUseCase::list_notifications(
        &database,
        "c006-tc008-recipient",
        true,
        50,
    )
    .await
    .unwrap();
    let matching: Vec<_> = notifications
        .into_iter()
        .filter(|notification| notification.uuid == notification_ids[0])
        .collect();
    assert_eq!(matching.len(), 1);
    assert_eq!(matching[0].notification_type, "FriendRequestCreated");

    let accepted = MentionNotificationUseCase::list_notifications(
        &database,
        "c006-tc008-actor",
        true,
        50,
    )
    .await
    .unwrap();
    assert!(accepted.iter().any(|notification| {
        notification.uuid == notification_ids[1]
            && notification.notification_type == "FriendRequestAccepted"
    }));

    let remaining = client
        .receive_message()
        .queue_url(&queue_url)
        .max_number_of_messages(10)
        .wait_time_seconds(1)
        .send()
        .await
        .unwrap()
        .messages
        .unwrap_or_default();
    assert!(remaining.is_empty());

    database
        .collection::<domain::in_app_notification::InAppNotification>("in_app_notifications")
        .delete_many(doc! { "_id": { "$in": &notification_ids } })
        .await
        .unwrap();
}
