use domain::business_error::BusinessError;
use domain::message::Message;
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, DateTime};
use mongodb::{Collection, Database};

const COLLECTION_NAME: &str = "messages";

pub struct MessageGateway {
    collection: Collection<Message>,
}

fn is_duplicate_key(e: &mongodb::error::Error) -> bool {
    let err = e.to_string().to_lowercase();
    err.contains("e11000") || err.contains("duplicate")
}

impl MessageGateway {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<Message>(COLLECTION_NAME),
        }
    }

    /// Idempotent insert: a duplicate `dedupeKey` (client retry) returns the
    /// message that is already stored rather than an error.
    pub async fn insert(&self, message: Message) -> Result<Message, BusinessError> {
        let dedupe_key = message.dedupe_key.clone();
        match self.collection.insert_one(&message).await {
            Ok(_) => Ok(message),
            Err(e) if is_duplicate_key(&e) => self
                .find_by_dedupe_key(&dedupe_key)
                .await?
                .ok_or_else(|| BusinessError::infrastructure("Message vanished after conflict")),
            Err(e) => Err(BusinessError::infrastructure(format!(
                "Failed to insert message: {e}"
            ))),
        }
    }

    pub async fn find_by_dedupe_key(
        &self,
        dedupe_key: &str,
    ) -> Result<Option<Message>, BusinessError> {
        self.collection
            .find_one(doc! { "dedupeKey": dedupe_key })
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Failed to load message: {e}")))
    }

    /// One page of a conversation's history, newest first.
    pub async fn find_page(
        &self,
        conversation_uuid: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Message>, BusinessError> {
        let mut cursor = self
            .collection
            .find(doc! { "conversationUuid": conversation_uuid })
            .sort(doc! { "sentAt": -1, "_id": -1 })
            .skip(skip)
            .limit(i64::try_from(limit).unwrap_or(i64::MAX))
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Failed to list messages: {e}")))?;

        let mut messages = Vec::new();
        while let Some(message) = cursor.try_next().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to iterate messages: {e}"))
        })? {
            messages.push(message);
        }
        Ok(messages)
    }

    /// Messages in a conversation strictly newer than `after` — the WebSocket
    /// reconnect replay path. Oldest first, capped.
    pub async fn find_since(
        &self,
        conversation_uuid: &str,
        after: DateTime,
        limit: u64,
    ) -> Result<Vec<Message>, BusinessError> {
        let mut cursor = self
            .collection
            .find(doc! {
                "conversationUuid": conversation_uuid,
                "sentAt": { "$gt": after },
            })
            .sort(doc! { "sentAt": 1, "_id": 1 })
            .limit(i64::try_from(limit).unwrap_or(i64::MAX))
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Failed to list messages: {e}")))?;

        let mut messages = Vec::new();
        while let Some(message) = cursor.try_next().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to iterate messages: {e}"))
        })? {
            messages.push(message);
        }
        Ok(messages)
    }

    /// Account-deletion cascade: every message this person sent, anywhere.
    pub async fn delete_all_by_sender(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.collection
            .delete_many(doc! { "senderPersonUuid": person_uuid })
            .await
            .map(|_| ())
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to delete messages: {e}"))
            })
    }

    /// Account-deletion cascade: everything left in conversations being removed.
    pub async fn delete_all_by_conversations(
        &self,
        conversation_uuids: &[String],
    ) -> Result<(), BusinessError> {
        if conversation_uuids.is_empty() {
            return Ok(());
        }
        self.collection
            .delete_many(doc! { "conversationUuid": { "$in": conversation_uuids } })
            .await
            .map(|_| ())
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to delete messages: {e}"))
            })
    }
}
