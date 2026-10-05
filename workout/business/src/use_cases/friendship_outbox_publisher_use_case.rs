use crate::domain::business_error::BusinessError;
use crate::domain::friendship_outbox_event::FriendshipOutboxEvent;
use crate::gateway::friendship_outbox_gateway::FriendshipOutboxGateway;
use crate::gateway::sqs_gateway::SqsGateway;
use aws_sdk_sqs::Client;
use sea_orm::DbConn;
use std::collections::HashSet;

const BATCH_SIZE: u64 = 10;

pub struct FriendshipOutboxPublisherUseCase;

impl FriendshipOutboxPublisherUseCase {
    pub async fn publish_pending(
        db: &DbConn,
        client: &Client,
        queue_url: &str,
    ) -> Result<usize, BusinessError> {
        let pending = FriendshipOutboxGateway::find_pending(db, BATCH_SIZE).await?;
        let mut published = 0;
        let mut blocked_friendships = HashSet::new();

        for record in pending {
            let friendship_uuid = record.friendship_uuid.to_string();
            if blocked_friendships.contains(&friendship_uuid) {
                continue;
            }
            if FriendshipOutboxGateway::has_earlier_unpublished(db, &record).await? {
                blocked_friendships.insert(friendship_uuid);
                continue;
            }
            let event = FriendshipOutboxEvent {
                event_uuid: record.event_uuid.to_string(),
                event_type: record.event_type.clone(),
                friendship_uuid: record.friendship_uuid.to_string(),
                actor_person_uuid: record.actor_person_uuid.to_string(),
                recipient_person_uuid: record.recipient_person_uuid.to_string(),
                occurred_at: record.occurred_at,
            };
            let body = serde_json::to_string(&event).map_err(|error| {
                BusinessError::new(format!("Failed to serialize friendship event: {error}"))
            })?;

            match SqsGateway::send_fifo_message(
                client,
                queue_url,
                &body,
                &event.friendship_uuid,
                &event.event_uuid,
            )
            .await
            {
                Ok(()) => {
                    FriendshipOutboxGateway::mark_published(db, record).await?;
                    published += 1;
                }
                Err(error) => {
                    let attempts =
                        FriendshipOutboxGateway::mark_failed(db, record, &error.message).await?;
                    if attempts == 5 {
                        log::error!(
                            "ALERT: friendship event has failed to publish five consecutive times; eventUuid={}",
                            event.event_uuid
                        );
                    }
                    log::warn!(
                        "Friendship event publish attempt {} failed; eventUuid={}: {}",
                        attempts,
                        event.event_uuid,
                        error.message
                    );
                    blocked_friendships.insert(friendship_uuid);
                }
            }
        }

        Ok(published)
    }
}
