use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FriendshipNotificationEvent {
    pub event_uuid: String,
    pub event_type: String,
    pub friendship_uuid: String,
    pub actor_person_uuid: String,
    pub recipient_person_uuid: String,
    pub occurred_at: String,
}
