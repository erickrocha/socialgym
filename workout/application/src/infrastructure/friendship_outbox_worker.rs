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

#[cfg(test)]
mod tests {
    use super::{start, QUEUE_URL_ENV};
    use business::domain::friendship_outbox_event::FriendshipOutboxEvent;
    use business::gateway::friendship_outbox_gateway::FriendshipOutboxGateway;
    use business::sea_orm::{ConnectionTrait, Database};
    use migration::{Migrator, MigratorTrait};
    use std::sync::Arc;
    use std::time::Duration;

    #[tokio::test]
    #[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database and the LocalStack test queue"]
    async fn c006_outbox_worker_publishes_pending_events_and_survives_disabled_config_and_outages() {
        let url = std::env::var("TEST_DATABASE_URL")
            .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
        let queue_url = std::env::var(QUEUE_URL_ENV)
            .expect("AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL must point to the test FIFO queue");
        let db = Arc::new(Database::connect(url.clone()).await.unwrap());
        Migrator::refresh(&*db).await.unwrap();

        // Without a queue URL the publisher disables itself instead of looping.
        unsafe { std::env::remove_var(QUEUE_URL_ENV) };
        start(db.clone());
        tokio::time::sleep(Duration::from_millis(200)).await;
        unsafe { std::env::set_var(QUEUE_URL_ENV, &queue_url) };

        // With a queue it drains the outbox on its own.
        let friendship = uuid::Uuid::new_v4().to_string();
        let event = FriendshipOutboxEvent {
            event_uuid: uuid::Uuid::new_v4().to_string(),
            event_type: "friend_request_created".into(),
            friendship_uuid: friendship,
            actor_person_uuid: uuid::Uuid::new_v4().to_string(),
            recipient_person_uuid: uuid::Uuid::new_v4().to_string(),
            occurred_at: chrono::Utc::now(),
        };
        FriendshipOutboxGateway::persist(&*db, event.clone()).await.unwrap();
        start(db.clone());
        let mut published = false;
        for _ in 0..100 {
            let row = db
                .query_one_raw(business::sea_orm::Statement::from_string(
                    business::sea_orm::DbBackend::Postgres,
                    format!("SELECT published_at IS NOT NULL AS done FROM friendship_notification_outbox WHERE event_uuid = '{}'", event.event_uuid),
                ))
                .await
                .unwrap()
                .unwrap();
            if row.try_get::<bool>("", "done").unwrap() {
                published = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        assert!(published, "worker did not publish the pending event within 20s");

        // A database outage is logged and the loop keeps running.
        let closed = Database::connect(url).await.unwrap();
        closed.close_by_ref().await.unwrap();
        start(Arc::new(closed));
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}
