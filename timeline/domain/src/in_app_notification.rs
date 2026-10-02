use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct InAppNotification {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub notification_type: String,
    pub recipient_person_uuid: String,
    pub actor_person_uuid: String,
    pub actor_name: String,
    pub post_uuid: Option<String>,
    pub comment_uuid: Option<String>,
    pub entity_type: String,
    pub entity_uuid: String,
    pub snippet: String,
    pub read: bool,
    pub created_at: DateTime,
    pub updated_at: DateTime,
    #[serde(default)]
    pub target_type: Option<String>,
    #[serde(default)]
    pub target_uuid: Option<String>,
    #[serde(default)]
    pub push_status: Option<String>,
    #[serde(default)]
    pub push_attempt_count: i32,
    #[serde(default)]
    pub push_completed_device_uuids: Vec<String>,
    #[serde(default)]
    pub push_claimed_at: Option<DateTime>,
    #[serde(default)]
    pub push_next_attempt_at: Option<DateTime>,
    #[serde(default)]
    pub push_last_error: Option<String>,
}

impl InAppNotification {
    #[allow(clippy::too_many_arguments)]
    pub fn from_mention_event(
        uuid: String,
        recipient_person_uuid: String,
        actor_person_uuid: String,
        actor_name: String,
        post_uuid: Option<String>,
        comment_uuid: Option<String>,
        entity_type: String,
        entity_uuid: String,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            notification_type: "Mention".to_string(),
            recipient_person_uuid,
            actor_person_uuid,
            actor_name,
            post_uuid: post_uuid.clone(),
            comment_uuid,
            entity_type,
            entity_uuid,
            snippet,
            read: false,
            created_at: now,
            updated_at: now,
            target_type: post_uuid.as_ref().map(|_| "post".to_string()),
            target_uuid: post_uuid.clone(),
            push_status: Some("Pending".to_string()),
            push_attempt_count: 0,
            push_completed_device_uuids: Vec::new(),
            push_claimed_at: None,
            push_next_attempt_at: None,
            push_last_error: None,
        }
    }

    /// A new chat message notification, surfaced through the same pull path as
    /// mentions so the unread badge keeps working. `uuid` is the idempotency
    /// key `format!("{message_uuid}:{recipient_person_uuid}")`.
    pub fn from_chat_message(
        uuid: String,
        recipient_person_uuid: String,
        actor_person_uuid: String,
        actor_name: String,
        conversation_uuid: String,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            notification_type: "ChatMessage".to_string(),
            recipient_person_uuid,
            actor_person_uuid,
            actor_name,
            post_uuid: None,
            comment_uuid: None,
            entity_type: "conversation".to_string(),
            entity_uuid: conversation_uuid,
            snippet,
            read: false,
            created_at: now,
            updated_at: now,
            target_type: None,
            target_uuid: None,
            push_status: None,
            push_attempt_count: 0,
            push_completed_device_uuids: Vec::new(),
            push_claimed_at: None,
            push_next_attempt_at: None,
            push_last_error: None,
        }
    }

    pub fn from_social_interaction(
        uuid: String,
        notification_type: String,
        recipient_person_uuid: String,
        actor_person_uuid: String,
        actor_name: String,
        post_uuid: String,
        comment_uuid: Option<String>,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            notification_type,
            recipient_person_uuid,
            actor_person_uuid,
            actor_name,
            post_uuid: Some(post_uuid.clone()),
            comment_uuid,
            entity_type: "post".to_string(),
            entity_uuid: post_uuid.clone(),
            snippet,
            read: false,
            created_at: now,
            updated_at: now,
            target_type: Some("post".to_string()),
            target_uuid: Some(post_uuid),
            push_status: Some("Pending".to_string()),
            push_attempt_count: 0,
            push_completed_device_uuids: Vec::new(),
            push_claimed_at: None,
            push_next_attempt_at: None,
            push_last_error: None,
        }
    }

    pub fn from_friendship_event(
        event_uuid: String,
        notification_type: String,
        recipient_person_uuid: String,
        actor_person_uuid: String,
        friendship_uuid: String,
        snippet: String,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid: event_uuid,
            notification_type,
            recipient_person_uuid,
            actor_person_uuid,
            actor_name: "A friend".to_string(),
            post_uuid: None,
            comment_uuid: None,
            entity_type: "friendship_request".to_string(),
            entity_uuid: friendship_uuid.clone(),
            snippet,
            read: false,
            created_at: now,
            updated_at: now,
            target_type: Some("friendship_request".to_string()),
            target_uuid: Some(friendship_uuid),
            push_status: Some("Pending".to_string()),
            push_attempt_count: 0,
            push_completed_device_uuids: Vec::new(),
            push_claimed_at: None,
            push_next_attempt_at: None,
            push_last_error: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::InAppNotification;

    #[test]
    fn mention_push_targets_the_related_post() {
        let notification = InAppNotification::from_mention_event(
            "event-1".to_string(),
            "recipient-1".to_string(),
            "actor-1".to_string(),
            "Actor".to_string(),
            Some("post-1".to_string()),
            Some("comment-1".to_string()),
            "comment".to_string(),
            "comment-1".to_string(),
            "A private comment excerpt".to_string(),
        );

        assert_eq!(notification.target_type.as_deref(), Some("post"));
        assert_eq!(notification.target_uuid.as_deref(), Some("post-1"));
        assert_eq!(notification.push_status.as_deref(), Some("Pending"));
    }

    #[test]
    fn chat_notifications_do_not_enter_the_social_push_pipeline() {
        let notification = InAppNotification::from_chat_message(
            "message-1:recipient-1".to_string(),
            "recipient-1".to_string(),
            "actor-1".to_string(),
            "Actor".to_string(),
            "conversation-1".to_string(),
            "A message".to_string(),
        );

        assert_eq!(notification.push_status, None);
    }
}

