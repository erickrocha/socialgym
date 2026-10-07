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
#[path = "../tests/friendship_notification_worker_unit_test.rs"]
mod tests;
