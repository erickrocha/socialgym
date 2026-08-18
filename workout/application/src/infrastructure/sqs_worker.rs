use business::sea_orm::DatabaseConnection;
use business::use_cases::sqs_consumer_use_case::SqsConsumerUseCase;
use std::sync::Arc;
use tokio::time::{sleep, Duration};

/// Spawns a background Tokio task that continuously polls SQS and processes
/// `ObjectCreated` S3 event notifications, persisting the uploaded file
/// metadata into the `person_media` table.
///
/// The worker uses 20-second long-polling, so the loop iterates roughly once
/// per call (immediately when messages arrive, after 20 s when the queue is
/// empty).  On transient errors it waits 5 seconds before retrying to avoid
/// hammering AWS on repeated failures.
///
/// If `AWS_SQS_QUEUE_URL` is not set the task exits immediately with an info
/// log so the rest of the application starts unaffected.
pub fn start(db: Arc<DatabaseConnection>) {
    tokio::spawn(async move {
        log::info!("SQS consumer worker starting…");

        loop {
            match SqsConsumerUseCase::poll_and_process(&db).await {
                Ok(0) => {
                    // Queue was empty; the 20-second long-poll already acted as
                    // a back-off, so we can loop immediately.
                }
                Ok(count) => {
                    log::info!("SQS consumer processed {} message(s)", count);
                }
                Err(e) => {
                    log::error!(
                        "SQS consumer encountered an error – backing off 5 s: {:?}",
                        e
                    );
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}

