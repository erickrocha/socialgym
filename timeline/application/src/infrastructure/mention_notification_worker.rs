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

