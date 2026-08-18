use crate::gateway::mention_notification_gateway::MentionNotificationGateway;
use domain::business_error::BusinessError;
use domain::in_app_notification::InAppNotification;
use domain::mention_notification_event::MentionNotificationEvent;
use mongodb::Database;

pub struct MentionNotificationUseCase;

impl MentionNotificationUseCase {
    pub async fn enqueue(db: &Database,events: Vec<MentionNotificationEvent>,) -> Result<(), BusinessError> {
        if events.is_empty() {
            return Ok(());
        }
        MentionNotificationGateway::new(db).enqueue_many(events).await
    }

    pub async fn process_pending(db: &Database, limit: i64) -> Result<usize, BusinessError> {
        let gateway = MentionNotificationGateway::new(db);
        let pending = gateway.list_pending(limit).await?;
        let mut processed = 0usize;

        for event in pending {
            if !gateway.claim_pending(&event.uuid).await? {
                continue;
            }

            let notification = InAppNotification::from_mention_event(
                event.uuid.clone(),
                event.mentioned_person_uuid.clone(),
                event.author_person_uuid.clone(),
                event.author_name.clone(),
                event.post_uuid.clone(),
                event.comment_uuid.clone(),
                event.entity_type.clone(),
                event.entity_uuid.clone(),
                event.snippet.clone(),
            );

            match gateway.persist_in_app_notification(notification).await {
                Ok(_) => {
                    gateway.mark_processed(&event.uuid).await?;
                    processed += 1;
                }
                Err(e) => {
                    let next_retry = event.retry_count + 1;
                    gateway
                        .mark_failed(&event.uuid, next_retry, &e.message)
                        .await?;
                }
            }
        }

        Ok(processed)
    }

    pub async fn list_notifications(db: &Database,recipient_person_uuid: &str,unread_only: bool,limit: i64) -> Result<Vec<InAppNotification>, BusinessError> {
        log::info!("Reading notifications for owner uuid: {} unread only: {} limit: {}", recipient_person_uuid, unread_only, limit);
        MentionNotificationGateway::new(db).list_in_app_notifications(recipient_person_uuid, unread_only, limit).await
    }

    pub async fn mark_as_read(db: &Database,recipient_person_uuid: &str,idempotency_key: &str) -> Result<bool, BusinessError> {
        log::info!("Marking notification read for owner uuid: {} idempotency key: {}", recipient_person_uuid, idempotency_key);
        MentionNotificationGateway::new(db).mark_in_app_notification_read(idempotency_key, recipient_person_uuid).await
    }
}

