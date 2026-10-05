use aws_config::BehaviorVersion;
use aws_sdk_sqs::Client;
use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use domain::friendship_notification_event::FriendshipNotificationEvent;
use domain::in_app_notification::InAppNotification;
use mongodb::Database;
use std::env;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

const QUEUE_URL_ENV: &str = "AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL";

pub fn start(db: Arc<Database>) {
    tokio::spawn(async move {
        let queue_url = match env::var(QUEUE_URL_ENV) {
            Ok(value) if !value.is_empty() => value,
            _ => {
                log::info!("{QUEUE_URL_ENV} is unset; friendship consumer is disabled");
                return;
            }
        };
        let config = aws_config::load_defaults(BehaviorVersion::latest()).await;
        let client = match env::var("AWS_ENDPOINT_URL") {
            Ok(endpoint) if !endpoint.is_empty() => {
                let config = aws_sdk_sqs::config::Builder::from(&config)
                    .endpoint_url(endpoint)
                    .build();
                Client::from_conf(config)
            }
            _ => Client::new(&config),
        };

        log::info!("Friendship notification SQS consumer started");
        loop {
            match consume_batch(&db, &client, &queue_url).await {
                Ok(0) => {}
                Ok(count) => log::info!("Materialized {count} friendship notification(s)"),
                Err(error) => {
                    log::error!("Friendship notification consumer failed: {error}");
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}

async fn consume_batch(db: &Database, client: &Client, queue_url: &str) -> Result<usize, String> {
    let output = client
        .receive_message()
        .queue_url(queue_url)
        .max_number_of_messages(10)
        .wait_time_seconds(20)
        .send()
        .await
        .map_err(|error| format!("SQS receive failed: {error}"))?;

    let mut processed = 0;
    for message in output.messages() {
        let (Some(body), Some(receipt_handle)) = (message.body(), message.receipt_handle()) else {
            log::error!("Friendship SQS message is missing body or receipt handle");
            continue;
        };
        let event: FriendshipNotificationEvent = serde_json::from_str(body)
            .map_err(|error| format!("invalid friendship event body: {error}"))?;
        let notification = notification_for_event(event)?;
        MentionNotificationUseCase::persist_friendship_notification(db, notification)
            .await
            .map_err(|error| format!("Mongo notification persistence failed: {error}"))?;
        client
            .delete_message()
            .queue_url(queue_url)
            .receipt_handle(receipt_handle)
            .send()
            .await
            .map_err(|error| format!("SQS acknowledgement failed: {error}"))?;
        processed += 1;
    }
    Ok(processed)
}

fn notification_for_event(event: FriendshipNotificationEvent) -> Result<InAppNotification, String> {
    let (notification_type, snippet) = match event.event_type.as_str() {
        "friend_request_created" => ("FriendRequestCreated", "Someone sent you a friend request."),
        "friend_request_accepted" => (
            "FriendRequestAccepted",
            "Someone accepted your friend request.",
        ),
        other => return Err(format!("unsupported friendship event type: {other}")),
    };

    Ok(InAppNotification::from_friendship_event(
        event.event_uuid,
        notification_type.to_string(),
        event.recipient_person_uuid,
        event.actor_person_uuid,
        event.friendship_uuid,
        snippet.to_string(),
    ))
}

#[cfg(test)]
mod tests {
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
                .message_group_id("c006-tc008-friendship")
                .message_deduplication_id(format!("c006-tc008-delivery-{index}"))
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
}
