use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
use mongodb::Database;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

pub fn start(db: Arc<Database>) {
    tokio::spawn(async move {
        log::info!("Mention notification worker starting");

        loop {
            match MentionNotificationUseCase::process_pending(&db, 20).await {
                Ok(0) => sleep(Duration::from_secs(2)).await,
                Ok(count) => {
                    log::info!("Mention notification worker processed {} event(s)", count);
                }
                Err(e) => {
                    log::error!(
                        "Mention notification worker failed to process events: {}",
                        e.message
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}


#[cfg(test)]
mod tests {
    use business::use_cases::mention_notification_use_case::MentionNotificationUseCase;
    use domain::mention_notification_event::MentionNotificationEvent;
    use mongodb::{Client, bson::doc};
    use std::{sync::Arc, time::Duration};

    #[tokio::test]
    #[ignore = "requires a disposable TEST_MONGO_URL targeting the timeline_test database"]
    async fn c006_mention_worker_turns_queued_events_into_notifications_and_survives_an_outage() {
        let url = std::env::var("TEST_MONGO_URL").expect("TEST_MONGO_URL must be set");
        assert!(url.contains("/timeline_test"), "refusing to run against a non-test database");
        let database = Arc::new(Client::with_uri_str(url).await.unwrap().database("timeline_test"));
        let events = database.collection::<MentionNotificationEvent>("mention_notification_events");
        let notifications = database.collection::<mongodb::bson::Document>("in_app_notifications");
        events.delete_many(doc! { "_id": "c006-worker-event" }).await.unwrap();
        notifications.delete_many(doc! { "_id": "c006-worker-event" }).await.unwrap();

        MentionNotificationUseCase::enqueue(
            &database,
            vec![MentionNotificationEvent::new(
                "c006-worker-event".into(), "post".into(), "p1".into(), Some("p1".into()), None, 1,
                "author".into(), "Author".into(), "c006-worker-recipient".into(), "hi".into(),
            )],
        )
        .await
        .unwrap();

        super::start(database.clone());
        let mut delivered = false;
        for _ in 0..50 {
            if notifications.count_documents(doc! { "_id": "c006-worker-event" }).await.unwrap() == 1 {
                delivered = true;
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }
        assert!(delivered, "worker did not materialize the notification within 5s");
        let event = events.find_one(doc! { "_id": "c006-worker-event" }).await.unwrap().unwrap();
        assert_eq!(event.status, "Processed");

        // With the database gone the loop logs the failure and keeps running (no panic).
        let unreachable = Client::with_uri_str("mongodb://127.0.0.1:1/?serverSelectionTimeoutMS=150&connectTimeoutMS=150")
            .await
            .unwrap()
            .database("unreachable");
        super::start(Arc::new(unreachable));
        tokio::time::sleep(Duration::from_millis(600)).await;

        events.delete_many(doc! { "_id": "c006-worker-event" }).await.unwrap();
        notifications.delete_many(doc! { "_id": "c006-worker-event" }).await.unwrap();
    }

}
