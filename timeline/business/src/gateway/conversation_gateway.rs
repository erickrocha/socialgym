use domain::business_error::BusinessError;
use domain::conversation::{Conversation, ConversationParticipant, LastMessagePreview};
use futures::stream::TryStreamExt;
use mongodb::bson::{doc, to_bson, DateTime};
use mongodb::{Collection, Database};

const COLLECTION_NAME: &str = "conversations";

pub struct ConversationGateway {
    collection: Collection<Conversation>,
}

fn is_duplicate_key(e: &mongodb::error::Error) -> bool {
    let err = e.to_string().to_lowercase();
    err.contains("e11000") || err.contains("duplicate")
}

impl ConversationGateway {
    pub fn new(db: &Database) -> Self {
        Self {
            collection: db.collection::<Conversation>(COLLECTION_NAME),
        }
    }

    /// Inserts a conversation. On a duplicate `dedupeKey` (another request won
    /// the get-or-create race) the already-stored conversation is returned
    /// instead of an error.
    pub async fn insert(&self, conversation: Conversation) -> Result<Conversation, BusinessError> {
        let dedupe_key = conversation.dedupe_key.clone();
        match self.collection.insert_one(&conversation).await {
            Ok(_) => Ok(conversation),
            Err(e) if is_duplicate_key(&e) => self
                .find_by_dedupe_key(&dedupe_key)
                .await?
                .ok_or_else(|| BusinessError::infrastructure("Conversation vanished after conflict")),
            Err(e) => Err(BusinessError::infrastructure(format!(
                "Failed to insert conversation: {e}"
            ))),
        }
    }

    pub async fn find_by_dedupe_key(
        &self,
        dedupe_key: &str,
    ) -> Result<Option<Conversation>, BusinessError> {
        self.collection
            .find_one(doc! { "dedupeKey": dedupe_key })
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to load conversation: {e}"))
            })
    }

    pub async fn find_by_uuid(&self, uuid: &str) -> Result<Option<Conversation>, BusinessError> {
        self.collection
            .find_one(doc! { "_id": uuid })
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to load conversation: {e}"))
            })
    }

    /// Conversations visible to `person_uuid`, newest activity first, paginated.
    pub async fn find_for_participant(
        &self,
        person_uuid: &str,
        skip: u64,
        limit: u64,
    ) -> Result<Vec<Conversation>, BusinessError> {
        let mut cursor = self
            .collection
            .find(doc! { "participantPersonUuids": person_uuid })
            .sort(doc! { "updatedAt": -1 })
            .skip(skip)
            .limit(i64::try_from(limit).unwrap_or(i64::MAX))
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to list conversations: {e}"))
            })?;

        let mut conversations = Vec::new();
        while let Some(conversation) = cursor.try_next().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to iterate conversations: {e}"))
        })? {
            conversations.push(conversation);
        }
        Ok(conversations)
    }

    /// Updates the denormalized `lastMessage` preview and bumps `updatedAt`.
    /// Guarded so a slow write can't clobber a newer preview.
    pub async fn set_last_message(
        &self,
        conversation_uuid: &str,
        preview: LastMessagePreview,
    ) -> Result<(), BusinessError> {
        let sent_at = preview.sent_at;
        let preview_bson = to_bson(&preview).map_err(|e| {
            BusinessError::infrastructure(format!("Failed to serialize last message: {e}"))
        })?;

        self.collection
            .update_one(
                doc! {
                    "_id": conversation_uuid,
                    "$or": [
                        { "lastMessage": { "$exists": false } },
                        { "lastMessage.sentAt": { "$lte": sent_at } },
                    ],
                },
                doc! {
                    "$set": {
                        "lastMessage": preview_bson,
                        "updatedAt": sent_at,
                    }
                },
            )
            .await
            .map(|_| ())
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to set last message: {e}"))
            })
    }

    /// Marks the conversation read up to `last_read_message_uuid` for one
    /// participant. Returns false when the person is not a participant.
    pub async fn mark_read(
        &self,
        conversation_uuid: &str,
        person_uuid: &str,
        last_read_message_uuid: &str,
        read_at: DateTime,
    ) -> Result<bool, BusinessError> {
        let result = self
            .collection
            .update_one(
                doc! {
                    "_id": conversation_uuid,
                    "participants.personUuid": person_uuid,
                },
                doc! {
                    "$set": {
                        "participants.$.lastReadAt": read_at,
                        "participants.$.lastReadMessageUuid": last_read_message_uuid,
                    }
                },
            )
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to mark conversation read: {e}"))
            })?;

        Ok(result.matched_count > 0)
    }

    /// Replaces the participant set of a business conversation with the current
    /// roster (self-heal: members added/removed since the conversation was
    /// created). New members keep no read state; departed members lose access.
    pub async fn sync_participants(
        &self,
        conversation_uuid: &str,
        participant_person_uuids: Vec<String>,
        participants: Vec<ConversationParticipant>,
    ) -> Result<(), BusinessError> {
        let current = match self.find_by_uuid(conversation_uuid).await? {
            Some(c) => c,
            None => return Ok(()),
        };

        // Preserve read state for participants that are still present.
        let merged: Vec<ConversationParticipant> = participants
            .into_iter()
            .map(|mut p| {
                if let Some(existing) = current
                    .participants
                    .iter()
                    .find(|e| e.person_uuid == p.person_uuid)
                {
                    p.last_read_at = existing.last_read_at;
                    p.last_read_message_uuid = existing.last_read_message_uuid.clone();
                    p.joined_at = existing.joined_at;
                }
                p
            })
            .collect();

        let same_members = {
            let mut a = current.participant_person_uuids.clone();
            let mut b = participant_person_uuids.clone();
            a.sort();
            b.sort();
            a == b
        };
        if same_members {
            return Ok(());
        }

        let participants_bson = to_bson(&merged).map_err(|e| {
            BusinessError::infrastructure(format!("Failed to serialize participants: {e}"))
        })?;

        self.collection
            .update_one(
                doc! { "_id": conversation_uuid },
                doc! {
                    "$set": {
                        "participantPersonUuids": participant_person_uuids,
                        "participants": participants_bson,
                    }
                },
            )
            .await
            .map(|_| ())
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to sync participants: {e}"))
            })
    }

    /// Account-deletion cascade: drops the person from every conversation and
    /// deletes any direct/1:1 conversation that becomes single-sided.
    pub async fn delete_all_involving_person(
        &self,
        person_uuid: &str,
    ) -> Result<Vec<String>, BusinessError> {
        // Collect direct-style conversations the person is in before mutating.
        let mut cursor = self
            .collection
            .find(doc! {
                "participantPersonUuids": person_uuid,
                "conversationType": { "$in": ["DirectPerson", "BusinessDirect"] },
            })
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to scan conversations: {e}"))
            })?;

        let mut orphaned_conversation_uuids = Vec::new();
        while let Some(c) = cursor.try_next().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to iterate conversations: {e}"))
        })? {
            orphaned_conversation_uuids.push(c.uuid);
        }

        // Remove the person from group rooms.
        self.collection
            .update_many(
                doc! { "conversationType": "BusinessTeamGroup" },
                doc! {
                    "$pull": {
                        "participantPersonUuids": person_uuid,
                        "participants": { "personUuid": person_uuid },
                    }
                },
            )
            .await
            .map_err(|e| {
                BusinessError::infrastructure(format!("Failed to prune group participants: {e}"))
            })?;

        // Delete the now single-sided direct/1:1 conversations wholesale.
        if !orphaned_conversation_uuids.is_empty() {
            self.collection
                .delete_many(doc! { "_id": { "$in": &orphaned_conversation_uuids } })
                .await
                .map_err(|e| {
                    BusinessError::infrastructure(format!("Failed to delete conversations: {e}"))
                })?;
        }

        Ok(orphaned_conversation_uuids)
    }
}
