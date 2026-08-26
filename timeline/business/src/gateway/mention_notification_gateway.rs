use domain::business_error::BusinessError;
use domain::in_app_notification::InAppNotification;
use domain::mention_notification_event::MentionNotificationEvent;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, DateTime};
use mongodb::Collection;

const EVENT_COLLECTION_NAME: &str = "mention_notification_events";
const IN_APP_NOTIFICATION_COLLECTION_NAME: &str = "in_app_notifications";

pub struct MentionNotificationGateway {
    event_collection: Collection<MentionNotificationEvent>,
    in_app_collection: Collection<InAppNotification>,
}

impl MentionNotificationGateway {
    pub fn new(db: &mongodb::Database) -> Self {
        Self {
            event_collection: db.collection::<MentionNotificationEvent>(EVENT_COLLECTION_NAME),
            in_app_collection: db.collection::<InAppNotification>(IN_APP_NOTIFICATION_COLLECTION_NAME),
        }
    }

    pub async fn enqueue_many(
        &self,
        events: Vec<MentionNotificationEvent>,
    ) -> Result<(), BusinessError> {
        for event in events {
            let result = self.event_collection.insert_one(event).await;
            if let Err(e) = result {
                let err = e.to_string().to_lowercase();
                if err.contains("e11000") || err.contains("duplicate") {
                    continue;
                }
                return Err(BusinessError::new(format!(
                    "failed to enqueue mention event: {e}"
                )));
            }
        }
        Ok(())
    }

    pub async fn list_pending(&self, limit: i64) -> Result<Vec<MentionNotificationEvent>, BusinessError> {
        let mut cursor = self
            .event_collection
            .find(doc! {
                "status": "Pending",
                "retryCount": { "$lt": 5 }
            })
            .sort(doc! { "createdAt": 1 })
            .limit(limit)
            .await
            .map_err(|e| BusinessError::new(format!("failed to list pending mention events: {e}")))?;

        let mut events = Vec::new();
        while let Some(event) = cursor
            .try_next()
            .await
            .map_err(|e| BusinessError::new(format!("failed to iterate mention events: {e}")))?
        {
            events.push(event);
        }

        Ok(events)
    }

    pub async fn claim_pending(&self, idempotency_key: &str) -> Result<bool, BusinessError> {
        let result = self
            .event_collection
            .update_one(
                doc! {
                    "_id": idempotency_key,
                    "status": "Pending",
                },
                doc! {
                    "$set": {
                        "status": "Processing",
                        "updatedAt": DateTime::now(),
                    }
                },
            )
            .await
            .map_err(|e| BusinessError::new(format!("failed to claim mention event: {e}")))?;

        Ok(result.matched_count > 0)
    }

    pub async fn mark_processed(&self, idempotency_key: &str) -> Result<(), BusinessError> {
        self.event_collection
            .update_one(
                doc! { "_id": idempotency_key },
                doc! {
                    "$set": {
                        "status": "Processed",
                        "updatedAt": DateTime::now(),
                        "processedAt": DateTime::now(),
                        "lastError": mongodb::bson::Bson::Null,
                    }
                },
            )
            .await
            .map_err(|e| BusinessError::new(format!("failed to mark mention event processed: {e}")))?;

        Ok(())
    }

    pub async fn mark_failed(
        &self,
        idempotency_key: &str,
        next_retry_count: i32,
        error_message: &str,
    ) -> Result<(), BusinessError> {
        let next_status = if next_retry_count >= 5 {
            "Failed"
        } else {
            "Pending"
        };

        self.event_collection
            .update_one(
                doc! { "_id": idempotency_key },
                doc! {
                    "$set": {
                        "status": next_status,
                        "retryCount": next_retry_count,
                        "lastError": error_message,
                        "updatedAt": DateTime::now(),
                    }
                },
            )
            .await
            .map_err(|e| BusinessError::new(format!("failed to mark mention event failed: {e}")))?;

        Ok(())
    }

    pub async fn persist_in_app_notification(
        &self,
        notification: InAppNotification,
    ) -> Result<(), BusinessError> {
        let result = self.in_app_collection.insert_one(notification).await;
        if let Err(e) = result {
            let err = e.to_string().to_lowercase();
            if err.contains("e11000") || err.contains("duplicate") {
                return Ok(());
            }
            return Err(BusinessError::new(format!(
                "failed to persist in-app notification: {e}"
            )));
        }
        Ok(())
    }

    pub async fn list_in_app_notifications(
        &self,
        recipient_person_uuid: &str,
        unread_only: bool,
        limit: i64,
    ) -> Result<Vec<InAppNotification>, BusinessError> {
        let mut filter = doc! { "recipientPersonUuid": recipient_person_uuid };
        if unread_only {
            filter.insert("read", false);
        }

        let mut cursor = self
            .in_app_collection
            .find(filter)
            .sort(doc! { "createdAt": -1 })
            .limit(limit)
            .await
            .map_err(|e| BusinessError::new(format!("failed to list in-app notifications: {e}")))?;

        let mut notifications = Vec::new();
        while let Some(notification) = cursor
            .try_next()
            .await
            .map_err(|e| BusinessError::new(format!("failed to iterate in-app notifications: {e}")))?
        {
            notifications.push(notification);
        }

        Ok(notifications)
    }

    pub async fn mark_in_app_notification_read(
        &self,
        idempotency_key: &str,
        recipient_person_uuid: &str,
    ) -> Result<bool, BusinessError> {
        let result = self
            .in_app_collection
            .update_one(
                doc! {
                    "_id": idempotency_key,
                    "recipientPersonUuid": recipient_person_uuid,
                },
                doc! {
                    "$set": {
                        "read": true,
                        "updatedAt": DateTime::now(),
                    }
                },
            )
            .await
            .map_err(|e| BusinessError::new(format!("failed to mark notification read: {e}")))?;

        Ok(result.matched_count > 0)
    }

    /// Account-deletion cascade: removes every mention event and in-app
    /// notification involving this person, on either side (author/mentioned,
    /// actor/recipient).
    pub async fn delete_all_involving_person(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.event_collection
            .delete_many(doc! {
                "$or": [
                    { "authorPersonUuid": person_uuid },
                    { "mentionedPersonUuid": person_uuid },
                ]
            })
            .await
            .map_err(|e| BusinessError::new(format!("failed to delete mention events: {e}")))?;

        self.in_app_collection
            .delete_many(doc! {
                "$or": [
                    { "recipientPersonUuid": person_uuid },
                    { "actorPersonUuid": person_uuid },
                ]
            })
            .await
            .map_err(|e| BusinessError::new(format!("failed to delete in-app notifications: {e}")))?;

        Ok(())
    }
}

