use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

/// A one-to-one chat between two friends. Exactly two `Person` participants.
pub const CONVERSATION_TYPE_DIRECT_PERSON: &str = "DirectPerson";
/// The shared team room of a business profile: owner + every Accepted member.
pub const CONVERSATION_TYPE_BUSINESS_TEAM_GROUP: &str = "BusinessTeamGroup";
/// A private thread between a business profile and one individual team member.
pub const CONVERSATION_TYPE_BUSINESS_DIRECT: &str = "BusinessDirect";

pub const PARTICIPANT_ROLE_MEMBER: &str = "member";
pub const PARTICIPANT_ROLE_OWNER: &str = "owner";

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Conversation {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub conversation_type: String,
    /// Unique key that makes get-or-create idempotent; see the helpers in
    /// `business::use_cases::chat_use_case`.
    pub dedupe_key: String,
    /// Flat list of person uuids that can see this conversation — the `$in`
    /// target for the visibility query.
    pub participant_person_uuids: Vec<String>,
    /// Set for `BusinessTeamGroup` / `BusinessDirect`.
    pub business_profile_uuid: Option<String>,
    pub business_profile_name: Option<String>,
    pub business_profile_logo_object_key: Option<String>,
    pub created_by_person_uuid: String,
    pub participants: Vec<ConversationParticipant>,
    pub last_message: Option<LastMessagePreview>,
    pub created_at: DateTime,
    /// Bumped on every message; the conversation-list sort key.
    pub updated_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct ConversationParticipant {
    pub person_uuid: String,
    pub role: String,
    pub last_read_at: Option<DateTime>,
    pub last_read_message_uuid: Option<String>,
    pub joined_at: DateTime,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LastMessagePreview {
    pub message_uuid: String,
    pub sender_person_uuid: String,
    pub sender_display_name: String,
    pub snippet: String,
    pub sent_at: DateTime,
    pub has_media: bool,
}

impl ConversationParticipant {
    pub fn new(person_uuid: String, role: &str) -> Self {
        Self {
            person_uuid,
            role: role.to_string(),
            last_read_at: None,
            last_read_message_uuid: None,
            joined_at: DateTime::now(),
        }
    }
}

impl Conversation {
    #[allow(clippy::too_many_arguments)]
    fn build(
        uuid: String,
        conversation_type: &str,
        dedupe_key: String,
        participant_person_uuids: Vec<String>,
        business_profile_uuid: Option<String>,
        business_profile_name: Option<String>,
        business_profile_logo_object_key: Option<String>,
        created_by_person_uuid: String,
        participants: Vec<ConversationParticipant>,
    ) -> Self {
        let now = DateTime::now();
        Self {
            uuid,
            conversation_type: conversation_type.to_string(),
            dedupe_key,
            participant_person_uuids,
            business_profile_uuid,
            business_profile_name,
            business_profile_logo_object_key,
            created_by_person_uuid,
            participants,
            last_message: None,
            created_at: now,
            updated_at: now,
        }
    }

    /// Direct person-to-person conversation. `a` / `b` are person uuids;
    /// ordering does not matter — the dedupe key is computed from a sorted pair.
    pub fn new_direct(uuid: String, dedupe_key: String, a: String, b: String, creator: String) -> Self {
        let participants = vec![
            ConversationParticipant::new(a.clone(), PARTICIPANT_ROLE_MEMBER),
            ConversationParticipant::new(b.clone(), PARTICIPANT_ROLE_MEMBER),
        ];
        Self::build(
            uuid,
            CONVERSATION_TYPE_DIRECT_PERSON,
            dedupe_key,
            vec![a, b],
            None,
            None,
            None,
            creator,
            participants,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_business_team_group(
        uuid: String,
        dedupe_key: String,
        business_profile_uuid: String,
        business_profile_name: Option<String>,
        business_profile_logo_object_key: Option<String>,
        owner_person_uuid: String,
        member_person_uuids: Vec<String>,
        creator: String,
    ) -> Self {
        let (participant_person_uuids, participants) =
            team_participants(&owner_person_uuid, &member_person_uuids);
        Self::build(
            uuid,
            CONVERSATION_TYPE_BUSINESS_TEAM_GROUP,
            dedupe_key,
            participant_person_uuids,
            Some(business_profile_uuid),
            business_profile_name,
            business_profile_logo_object_key,
            creator,
            participants,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn new_business_direct(
        uuid: String,
        dedupe_key: String,
        business_profile_uuid: String,
        business_profile_name: Option<String>,
        business_profile_logo_object_key: Option<String>,
        owner_person_uuid: String,
        member_person_uuid: String,
        creator: String,
    ) -> Self {
        let participants = vec![
            ConversationParticipant::new(owner_person_uuid.clone(), PARTICIPANT_ROLE_OWNER),
            ConversationParticipant::new(member_person_uuid.clone(), PARTICIPANT_ROLE_MEMBER),
        ];
        Self::build(
            uuid,
            CONVERSATION_TYPE_BUSINESS_DIRECT,
            dedupe_key,
            vec![owner_person_uuid, member_person_uuid],
            Some(business_profile_uuid),
            business_profile_name,
            business_profile_logo_object_key,
            creator,
            participants,
        )
    }

    pub fn is_business(&self) -> bool {
        self.conversation_type == CONVERSATION_TYPE_BUSINESS_TEAM_GROUP
            || self.conversation_type == CONVERSATION_TYPE_BUSINESS_DIRECT
    }
}

/// Builds the `(participant_person_uuids, participants)` pair for a business
/// team group: the owner (role `owner`) followed by every distinct member
/// (role `member`), the owner never duplicated as a member.
pub fn team_participants(
    owner_person_uuid: &str,
    member_person_uuids: &[String],
) -> (Vec<String>, Vec<ConversationParticipant>) {
    let mut uuids = vec![owner_person_uuid.to_string()];
    let mut participants = vec![ConversationParticipant::new(
        owner_person_uuid.to_string(),
        PARTICIPANT_ROLE_OWNER,
    )];

    for member in member_person_uuids {
        let trimmed = member.trim();
        if trimmed.is_empty() || trimmed == owner_person_uuid || uuids.iter().any(|u| u == trimmed) {
            continue;
        }
        uuids.push(trimmed.to_string());
        participants.push(ConversationParticipant::new(
            trimmed.to_string(),
            PARTICIPANT_ROLE_MEMBER,
        ));
    }

    (uuids, participants)
}
