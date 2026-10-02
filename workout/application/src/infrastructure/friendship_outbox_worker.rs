use business::gateway::aws_clients::sqs_client;
use business::sea_orm::DatabaseConnection;
use business::use_cases::friendship_outbox_publisher_use_case::FriendshipOutboxPublisherUseCase;
use std::env;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

const QUEUE_URL_ENV: &str = "AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL";

pub fn start(db: Arc<DatabaseConnection>) {
    tokio::spawn(async move {
        let queue_url = match env::var(QUEUE_URL_ENV) {
            Ok(value) if !value.is_empty() => value,
            _ => {
                log::info!("{QUEUE_URL_ENV} is unset; friendship outbox publisher is disabled");
                return;
            }
        };
        let client = sqs_client().await;
        log::info!("Friendship outbox publisher started");

        loop {
            match FriendshipOutboxPublisherUseCase::publish_pending(&db, &client, &queue_url).await
            {
                Ok(0) => sleep(Duration::from_secs(5)).await,
                Ok(count) => log::info!("Published {count} friendship outbox event(s)"),
                Err(error) => {
                    log::error!("Friendship outbox publisher failed: {error}");
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}
