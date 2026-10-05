use crate::domain::business_error::BusinessError;
use crate::domain::friend::Friend;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendshipOutboxEvent {
    pub event_uuid: String,
    pub event_type: String,
    pub friendship_uuid: String,
    pub actor_person_uuid: String,
    pub recipient_person_uuid: String,
    pub occurred_at: DateTime<Utc>,
}

impl FriendshipOutboxEvent {
    pub fn request_created(friendship: &Friend) -> Result<Self, BusinessError> {
        Self::from_friendship(friendship, "friend_request_created", false)
    }

    pub fn request_accepted(friendship: &Friend) -> Result<Self, BusinessError> {
        Self::from_friendship(friendship, "friend_request_accepted", true)
    }

    fn from_friendship(
        friendship: &Friend,
        event_type: &str,
        accepted: bool,
    ) -> Result<Self, BusinessError> {
        let friendship_uuid = friendship
            .uuid
            .clone()
            .ok_or_else(|| BusinessError::new("Friendship UUID is missing".to_string()))?;

        let (actor_person_uuid, recipient_person_uuid) = if accepted {
            (
                friendship.friend_uuid.clone(),
                friendship.person_uuid.clone(),
            )
        } else {
            (
                friendship.person_uuid.clone(),
                friendship.friend_uuid.clone(),
            )
        };

        Ok(Self {
            event_uuid: uuid::Uuid::new_v4().to_string(),
            event_type: event_type.to_string(),
            friendship_uuid,
            actor_person_uuid,
            recipient_person_uuid,
            occurred_at: Utc::now(),
        })
    }
}
