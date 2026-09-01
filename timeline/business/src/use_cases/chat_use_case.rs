use domain::business_error::BusinessError;
use domain::conversation::{
    team_participants, Conversation, ConversationParticipant, LastMessagePreview,
    CONVERSATION_TYPE_BUSINESS_DIRECT, CONVERSATION_TYPE_BUSINESS_TEAM_GROUP,
    CONVERSATION_TYPE_DIRECT_PERSON,
};
use domain::in_app_notification::InAppNotification;
use domain::message::{
    Message, MessageMedia, MESSAGE_MEDIA_TYPE_IMAGE, SENDER_KIND_BUSINESS_PROFILE,
    SENDER_KIND_PERSON,
};
use domain::user::User;
use futures::stream::StreamExt;
use mongodb::bson::DateTime;
use mongodb::Database;
use uuid::Uuid;

use crate::commons::grpc_config::GrpcConfig;
use crate::gateway::conversation_gateway::ConversationGateway;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::mention_notification_gateway::MentionNotificationGateway;
use crate::gateway::message_gateway::MessageGateway;
use crate::gateway::team_member_gateway::{TeamMemberGateway, TeamRoster};
use crate::use_cases::media_use_case::MediaUseCase;

pub const CONVERSATION_PAGE_SIZE: u64 = 20;
pub const MESSAGE_PAGE_SIZE: u64 = 30;
pub const RECONNECT_REPLAY_LIMIT: u64 = 200;

const MAX_BODY_LEN: usize = 4000;
const MAX_MEDIA_PER_MESSAGE: usize = 10;
const SNIPPET_MAX_CHARS: usize = 120;
const MEDIA_ONLY_SNIPPET: &str = "\u{1F4F7} Photo";
const SIGNED_URL_CONCURRENCY: usize = 6;

/// A conversation plus the caller's unread flag, for the list view.
#[derive(Debug, Clone)]
pub struct ConversationView {
    pub conversation: Conversation,
    pub unread: bool,
}

/// Result of a successful send: the stored message (media hydrated) plus the
/// people the transport layer should push it to.
#[derive(Debug, Clone)]
pub struct SendOutcome {
    pub message: Message,
    pub conversation_type: String,
    pub recipients: Vec<String>,
}

/// Result of a mark-read: who to notify of the moved read pointer.
#[derive(Debug, Clone)]
pub struct ReadOutcome {
    pub conversation_uuid: String,
    pub reader_person_uuid: String,
    pub last_read_message_uuid: String,
    pub recipients: Vec<String>,
}

pub struct ChatUseCase;

impl ChatUseCase {
    // ── create / get-or-create ───────────────────────────────────────────────

    pub async fn get_or_create_direct(
        db: &Database,
        user: &User,
        target_person_uuid: &str,
    ) -> Result<Conversation, BusinessError> {
        let target = target_person_uuid.trim();
        if target.is_empty() {
            return Err(BusinessError::validation("targetPersonUuid is required"));
        }
        if target == user.person_uuid {
            return Err(BusinessError::validation("Cannot start a chat with yourself"));
        }

        let friends = FriendGateway::new(GrpcConfig::build_endpoint())
            .find_friend_uuids(user.person_id, &user.person_uuid)
            .await?;
        if !friends.iter().any(|u| u == target) {
            return Err(BusinessError::forbidden(
                "You can only message your friends",
            ));
        }

        let dedupe_key = direct_dedupe_key(&user.person_uuid, target);
        let gateway = ConversationGateway::new(db);
        if let Some(existing) = gateway.find_by_dedupe_key(&dedupe_key).await? {
            return Ok(existing);
        }

        let conversation = Conversation::new_direct(
            Uuid::new_v4().to_string(),
            dedupe_key,
            user.person_uuid.clone(),
            target.to_string(),
            user.person_uuid.clone(),
        );
        gateway.insert(conversation).await
    }

    pub async fn get_or_create_business_team_group(
        db: &Database,
        user: &User,
        business_profile_uuid: &str,
    ) -> Result<Conversation, BusinessError> {
        let bp = business_profile_uuid.trim();
        if bp.is_empty() {
            return Err(BusinessError::validation("businessProfileUuid is required"));
        }

        let roster = Self::load_roster(bp).await?;
        if !roster.allows(&user.person_uuid) {
            return Err(BusinessError::forbidden(
                "You are not part of this team",
            ));
        }

        let dedupe_key = business_team_group_dedupe_key(bp);
        let gateway = ConversationGateway::new(db);

        if let Some(existing) = gateway.find_by_dedupe_key(&dedupe_key).await? {
            let (uuids, participants) =
                team_participants(&roster.owner_person_uuid, &roster.accepted_member_person_uuids);
            gateway
                .sync_participants(&existing.uuid, uuids, participants)
                .await?;
            return gateway
                .find_by_uuid(&existing.uuid)
                .await?
                .ok_or_else(|| BusinessError::infrastructure("Conversation vanished"));
        }

        let conversation = Conversation::new_business_team_group(
            Uuid::new_v4().to_string(),
            dedupe_key,
            roster.business_profile_uuid.clone(),
            Some(roster.business_profile_name.clone()),
            roster.business_profile_logo_object_key.clone(),
            roster.owner_person_uuid.clone(),
            roster.accepted_member_person_uuids.clone(),
            user.person_uuid.clone(),
        );
        gateway.insert(conversation).await
    }

    pub async fn get_or_create_business_direct(
        db: &Database,
        user: &User,
        business_profile_uuid: &str,
        member_person_uuid: Option<&str>,
    ) -> Result<Conversation, BusinessError> {
        let bp = business_profile_uuid.trim();
        if bp.is_empty() {
            return Err(BusinessError::validation("businessProfileUuid is required"));
        }

        let roster = Self::load_roster(bp).await?;
        let member = resolve_business_direct_member(&roster, &user.person_uuid, member_person_uuid)?;

        let dedupe_key = business_direct_dedupe_key(bp, &member);
        let gateway = ConversationGateway::new(db);
        if let Some(existing) = gateway.find_by_dedupe_key(&dedupe_key).await? {
            return Ok(existing);
        }

        let conversation = Conversation::new_business_direct(
            Uuid::new_v4().to_string(),
            dedupe_key,
            roster.business_profile_uuid.clone(),
            Some(roster.business_profile_name.clone()),
            roster.business_profile_logo_object_key.clone(),
            roster.owner_person_uuid.clone(),
            member,
            user.person_uuid.clone(),
        );
        gateway.insert(conversation).await
    }

    // ── read paths ──────────────────────────────────────────────────────────

    pub async fn list_conversations(
        db: &Database,
        user: &User,
        page: u32,
    ) -> Result<Vec<ConversationView>, BusinessError> {
        let skip = page_skip(page, CONVERSATION_PAGE_SIZE);
        let conversations = ConversationGateway::new(db)
            .find_for_participant(&user.person_uuid, skip, CONVERSATION_PAGE_SIZE)
            .await?;

        let mut views = Vec::with_capacity(conversations.len());
        for mut conversation in conversations {
            Self::hydrate_conversation_logo(&mut conversation).await;
            let unread = conversation
                .participants
                .iter()
                .find(|p| p.person_uuid == user.person_uuid)
                .map(|p| unread_for_participant(p, &conversation.last_message))
                .unwrap_or(false);
            views.push(ConversationView { conversation, unread });
        }
        Ok(views)
    }

    pub async fn list_messages(
        db: &Database,
        user: &User,
        conversation_uuid: &str,
        page: u32,
    ) -> Result<Vec<Message>, BusinessError> {
        let conversation = Self::load_participant_conversation(db, user, conversation_uuid).await?;
        let messages = MessageGateway::new(db)
            .find_page(
                &conversation.uuid,
                page_skip(page, MESSAGE_PAGE_SIZE),
                MESSAGE_PAGE_SIZE,
            )
            .await?;
        Ok(Self::hydrate_messages_media(messages).await)
    }

    pub async fn list_messages_since(
        db: &Database,
        user: &User,
        conversation_uuid: &str,
        since_epoch_ms: i64,
    ) -> Result<Vec<Message>, BusinessError> {
        let conversation = Self::load_participant_conversation(db, user, conversation_uuid).await?;
        let messages = MessageGateway::new(db)
            .find_since(
                &conversation.uuid,
                DateTime::from_millis(since_epoch_ms),
                RECONNECT_REPLAY_LIMIT,
            )
            .await?;
        Ok(Self::hydrate_messages_media(messages).await)
    }

    // ── send ────────────────────────────────────────────────────────────────

    pub async fn send_message(
        db: &Database,
        user: &User,
        conversation_uuid: &str,
        body: &str,
        media: Vec<MessageMedia>,
        client_message_id: &str,
    ) -> Result<SendOutcome, BusinessError> {
        let client_message_id = client_message_id.trim();
        if client_message_id.is_empty() {
            return Err(BusinessError::validation("clientMessageId is required"));
        }
        let body = validate_message(body, &media)?;

        let conversation = ConversationGateway::new(db)
            .find_by_uuid(conversation_uuid)
            .await?
            .ok_or_else(|| BusinessError::not_found("Conversation not found"))?;
        ensure_participant(&conversation, &user.person_uuid)?;

        // Re-check the live relationship every send.
        let mut send_as_business = false;
        let mut business_name: Option<String> = None;
        let mut business_logo: Option<String> = None;

        match conversation.conversation_type.as_str() {
            CONVERSATION_TYPE_DIRECT_PERSON => {
                let other = other_direct_participant(&conversation, &user.person_uuid)
                    .ok_or_else(|| BusinessError::infrastructure("Malformed direct conversation"))?;
                let friends = FriendGateway::new(GrpcConfig::build_endpoint())
                    .find_friend_uuids(user.person_id, &user.person_uuid)
                    .await?;
                if !friends.iter().any(|u| *u == other) {
                    return Err(BusinessError::forbidden(
                        "You are no longer friends with this person",
                    ));
                }
            }
            CONVERSATION_TYPE_BUSINESS_TEAM_GROUP | CONVERSATION_TYPE_BUSINESS_DIRECT => {
                let bp_uuid = conversation
                    .business_profile_uuid
                    .as_deref()
                    .ok_or_else(|| BusinessError::infrastructure("Malformed business conversation"))?;
                let roster = Self::load_roster(bp_uuid).await?;
                if !roster.allows(&user.person_uuid) {
                    return Err(BusinessError::forbidden(
                        "You are no longer a member of this team",
                    ));
                }
                if conversation.conversation_type == CONVERSATION_TYPE_BUSINESS_TEAM_GROUP {
                    let (uuids, participants) = team_participants(
                        &roster.owner_person_uuid,
                        &roster.accepted_member_person_uuids,
                    );
                    ConversationGateway::new(db)
                        .sync_participants(&conversation.uuid, uuids, participants)
                        .await?;
                }
                if resolve_send_as_business(
                    user.active_business_profile_uuid.as_deref(),
                    Some(bp_uuid),
                    &user.person_uuid,
                    &roster.owner_person_uuid,
                ) {
                    send_as_business = true;
                    business_name = Some(roster.business_profile_name.clone());
                    business_logo = roster.business_profile_logo_object_key.clone();
                }
            }
            other => {
                return Err(BusinessError::infrastructure(format!(
                    "Unknown conversation type '{other}'"
                )));
            }
        }

        let (sender_kind, sender_display_name, sender_object_key, sender_business_profile_uuid) =
            if send_as_business {
                (
                    SENDER_KIND_BUSINESS_PROFILE.to_string(),
                    business_name.unwrap_or_else(|| user.name.clone()),
                    business_logo,
                    conversation.business_profile_uuid.clone(),
                )
            } else {
                (
                    SENDER_KIND_PERSON.to_string(),
                    user.name.clone(),
                    Some(user.person_object_key.clone()),
                    None,
                )
            };

        let message = Message::new(
            Uuid::new_v4().to_string(),
            conversation.uuid.clone(),
            user.person_uuid.clone(),
            sender_kind,
            sender_display_name,
            sender_object_key,
            sender_business_profile_uuid,
            body,
            media,
            client_message_id.to_string(),
        );

        let stored = MessageGateway::new(db).insert(message).await?;

        let preview = LastMessagePreview {
            message_uuid: stored.uuid.clone(),
            sender_person_uuid: stored.sender_person_uuid.clone(),
            sender_display_name: stored.sender_display_name.clone(),
            snippet: chat_snippet(&stored.body, !stored.media.is_empty()),
            sent_at: stored.sent_at,
            has_media: !stored.media.is_empty(),
        };
        ConversationGateway::new(db)
            .set_last_message(&conversation.uuid, preview.clone())
            .await?;

        // Fan out unread notifications through the existing pull path.
        let notification_gateway = MentionNotificationGateway::new(db);
        for participant in &conversation.participant_person_uuids {
            if participant == &user.person_uuid {
                continue;
            }
            let notification = InAppNotification::from_chat_message(
                format!("{}:{}", stored.uuid, participant),
                participant.clone(),
                user.person_uuid.clone(),
                preview.sender_display_name.clone(),
                conversation.uuid.clone(),
                preview.snippet.clone(),
            );
            if let Err(e) = notification_gateway
                .persist_in_app_notification(notification)
                .await
            {
                log::error!("Failed to persist chat notification: {}", e.message);
            }
        }

        let hydrated = Self::hydrate_messages_media(vec![stored]).await;
        let message = hydrated.into_iter().next().unwrap();

        Ok(SendOutcome {
            message,
            conversation_type: conversation.conversation_type.clone(),
            recipients: conversation.participant_person_uuids,
        })
    }

    pub async fn mark_read(
        db: &Database,
        user: &User,
        conversation_uuid: &str,
        last_read_message_uuid: &str,
    ) -> Result<ReadOutcome, BusinessError> {
        let last_read = last_read_message_uuid.trim();
        if last_read.is_empty() {
            return Err(BusinessError::validation("lastReadMessageUuid is required"));
        }
        let conversation = Self::load_participant_conversation(db, user, conversation_uuid).await?;
        ConversationGateway::new(db)
            .mark_read(
                &conversation.uuid,
                &user.person_uuid,
                last_read,
                DateTime::now(),
            )
            .await?;

        let recipients = conversation
            .participant_person_uuids
            .iter()
            .filter(|p| *p != &user.person_uuid)
            .cloned()
            .collect();

        Ok(ReadOutcome {
            conversation_uuid: conversation.uuid,
            reader_person_uuid: user.person_uuid.clone(),
            last_read_message_uuid: last_read.to_string(),
            recipients,
        })
    }

    // ── internals ───────────────────────────────────────────────────────────

    async fn load_roster(business_profile_uuid: &str) -> Result<TeamRoster, BusinessError> {
        TeamMemberGateway::new(GrpcConfig::build_endpoint())
            .get_team_roster(business_profile_uuid)
            .await
    }

    async fn load_participant_conversation(
        db: &Database,
        user: &User,
        conversation_uuid: &str,
    ) -> Result<Conversation, BusinessError> {
        let conversation = ConversationGateway::new(db)
            .find_by_uuid(conversation_uuid)
            .await?
            .ok_or_else(|| BusinessError::not_found("Conversation not found"))?;
        ensure_participant(&conversation, &user.person_uuid)?;
        Ok(conversation)
    }

    async fn hydrate_conversation_logo(conversation: &mut Conversation) {
        if let Some(key) = conversation.business_profile_logo_object_key.clone() {
            if !key.is_empty() {
                match MediaUseCase::generate_cloud_front_signed_url(&key).await {
                    Ok(url) => conversation.business_profile_logo_object_key = Some(url),
                    Err(e) => log::error!("Failed to sign conversation logo: {}", e.message),
                }
            }
        }
    }

    async fn hydrate_messages_media(mut messages: Vec<Message>) -> Vec<Message> {
        for message in &mut messages {
            if message.media.is_empty() {
                continue;
            }
            let keys: Vec<String> = message.media.iter().map(|m| m.object_key.clone()).collect();
            let results = futures::stream::iter(keys.into_iter().enumerate())
                .map(|(idx, key)| async move {
                    (idx, MediaUseCase::generate_cloud_front_signed_url(&key).await)
                })
                .buffer_unordered(SIGNED_URL_CONCURRENCY)
                .collect::<Vec<(usize, Result<String, BusinessError>)>>()
                .await;
            for (idx, res) in results {
                match res {
                    Ok(url) => {
                        if let Some(media) = message.media.get_mut(idx) {
                            media.url = url;
                        }
                    }
                    Err(e) => log::error!("Failed to sign message media: {}", e.message),
                }
            }
        }
        messages
    }
}

// ── pure helpers (unit-tested) ──────────────────────────────────────────────

/// Order-independent key for a person-to-person conversation.
pub fn direct_dedupe_key(a: &str, b: &str) -> String {
    let (lo, hi) = if a <= b { (a, b) } else { (b, a) };
    format!("direct:{lo}:{hi}")
}

pub fn business_team_group_dedupe_key(business_profile_uuid: &str) -> String {
    format!("bpteam:{business_profile_uuid}")
}

pub fn business_direct_dedupe_key(business_profile_uuid: &str, member_person_uuid: &str) -> String {
    format!("bpdm:{business_profile_uuid}:{member_person_uuid}")
}

pub fn page_skip(page: u32, page_size: u64) -> u64 {
    u64::from(page).saturating_mul(page_size)
}

/// A message must carry text or at least one image; images must actually be
/// images; length and count are capped. Returns the trimmed body.
pub fn validate_message(body: &str, media: &[MessageMedia]) -> Result<String, BusinessError> {
    let trimmed = body.trim().to_string();
    if trimmed.is_empty() && media.is_empty() {
        return Err(BusinessError::validation("Message must have text or an image"));
    }
    if trimmed.chars().count() > MAX_BODY_LEN {
        return Err(BusinessError::validation(format!(
            "Message must be at most {MAX_BODY_LEN} characters"
        )));
    }
    if media.len() > MAX_MEDIA_PER_MESSAGE {
        return Err(BusinessError::validation(format!(
            "At most {MAX_MEDIA_PER_MESSAGE} images per message"
        )));
    }
    if media.iter().any(|m| m.media_type != MESSAGE_MEDIA_TYPE_IMAGE) {
        return Err(BusinessError::validation("Only image attachments are allowed"));
    }
    if media.iter().any(|m| m.object_key.trim().is_empty()) {
        return Err(BusinessError::validation("Attachment is missing its object key"));
    }
    Ok(trimmed)
}

pub fn chat_snippet(body: &str, has_media: bool) -> String {
    let trimmed = body.trim();
    if trimmed.is_empty() && has_media {
        return MEDIA_ONLY_SNIPPET.to_string();
    }
    let mut snippet: String = trimmed.chars().take(SNIPPET_MAX_CHARS).collect();
    if trimmed.chars().count() > SNIPPET_MAX_CHARS {
        snippet.push_str("...");
    }
    snippet
}

pub fn unread_for_participant(
    participant: &ConversationParticipant,
    last_message: &Option<LastMessagePreview>,
) -> bool {
    let Some(last) = last_message else {
        return false;
    };
    if last.sender_person_uuid == participant.person_uuid {
        return false;
    }
    match participant.last_read_at {
        None => true,
        Some(read_at) => last.sent_at.timestamp_millis() > read_at.timestamp_millis(),
    }
}

pub fn other_direct_participant(conversation: &Conversation, me: &str) -> Option<String> {
    conversation
        .participant_person_uuids
        .iter()
        .find(|u| u.as_str() != me)
        .cloned()
}

pub fn ensure_participant(conversation: &Conversation, person_uuid: &str) -> Result<(), BusinessError> {
    if conversation
        .participant_person_uuids
        .iter()
        .any(|u| u == person_uuid)
    {
        return Ok(());
    }
    Err(BusinessError::forbidden("Not a participant of this conversation"))
}

/// A business thread message is attributed to the business profile only when
/// the caller is the owner *and* is currently acting in that profile's session.
pub fn resolve_send_as_business(
    active_business_profile_uuid: Option<&str>,
    conversation_business_profile_uuid: Option<&str>,
    caller_person_uuid: &str,
    owner_person_uuid: &str,
) -> bool {
    match (active_business_profile_uuid, conversation_business_profile_uuid) {
        (Some(active), Some(conv)) => {
            active == conv && caller_person_uuid == owner_person_uuid
        }
        _ => false,
    }
}

/// Resolves which team member a `BusinessDirect` thread is with.
/// - owner caller: `member_person_uuid` must be given and be an Accepted member.
/// - member caller: the thread is with themselves; `member_person_uuid` ignored.
pub fn resolve_business_direct_member(
    roster: &TeamRoster,
    caller_person_uuid: &str,
    member_person_uuid: Option<&str>,
) -> Result<String, BusinessError> {
    if caller_person_uuid == roster.owner_person_uuid {
        let member = member_person_uuid
            .map(str::trim)
            .filter(|m| !m.is_empty())
            .ok_or_else(|| BusinessError::validation("memberPersonUuid is required"))?;
        if !roster
            .accepted_member_person_uuids
            .iter()
            .any(|u| u == member)
        {
            return Err(BusinessError::forbidden("Not an accepted team member"));
        }
        Ok(member.to_string())
    } else if roster
        .accepted_member_person_uuids
        .iter()
        .any(|u| u == caller_person_uuid)
    {
        Ok(caller_person_uuid.to_string())
    } else {
        Err(BusinessError::forbidden("You are not part of this team"))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use domain::business_error::BusinessErrorKind;
    use domain::conversation::Conversation;

    fn roster(owner: &str, members: &[&str]) -> TeamRoster {
        TeamRoster {
            business_profile_id: 1,
            business_profile_uuid: "bp-1".to_string(),
            business_profile_name: "Gym".to_string(),
            business_profile_logo_object_key: None,
            owner_person_uuid: owner.to_string(),
            accepted_member_person_uuids: members.iter().map(|s| s.to_string()).collect(),
        }
    }

    fn img(key: &str) -> MessageMedia {
        MessageMedia {
            media_type: MESSAGE_MEDIA_TYPE_IMAGE.to_string(),
            object_key: key.to_string(),
            url: String::new(),
        }
    }

    #[test]
    fn direct_dedupe_key_is_order_independent() {
        assert_eq!(direct_dedupe_key("a", "b"), direct_dedupe_key("b", "a"));
        assert_eq!(direct_dedupe_key("a", "b"), "direct:a:b");
    }

    #[test]
    fn business_dedupe_keys_have_stable_shape() {
        assert_eq!(business_team_group_dedupe_key("bp-1"), "bpteam:bp-1");
        assert_eq!(business_direct_dedupe_key("bp-1", "p-9"), "bpdm:bp-1:p-9");
    }

    #[test]
    fn page_skip_matches_page_times_size() {
        assert_eq!(page_skip(0, 20), 0);
        assert_eq!(page_skip(1, 20), 20);
        assert_eq!(page_skip(u32::MAX, 30), u64::from(u32::MAX) * 30);
    }

    #[test]
    fn validate_message_requires_text_or_image() {
        assert_eq!(
            validate_message("   ", &[]).unwrap_err().kind,
            BusinessErrorKind::Validation
        );
        assert_eq!(validate_message("  hi ", &[]).unwrap(), "hi");
        assert_eq!(validate_message("", &[img("k")]).unwrap(), "");
    }

    #[test]
    fn validate_message_rejects_oversized_and_non_image() {
        let long = "x".repeat(MAX_BODY_LEN + 1);
        assert!(validate_message(&long, &[]).is_err());

        let bad = MessageMedia {
            media_type: "Video".to_string(),
            object_key: "k".to_string(),
            url: String::new(),
        };
        assert!(validate_message("ok", &[bad]).is_err());

        let too_many: Vec<MessageMedia> = (0..MAX_MEDIA_PER_MESSAGE + 1).map(|i| img(&i.to_string())).collect();
        assert!(validate_message("ok", &too_many).is_err());
    }

    #[test]
    fn chat_snippet_truncates_and_marks_media() {
        let long = "a".repeat(130);
        let s = chat_snippet(&long, false);
        assert!(s.ends_with("..."));
        assert_eq!(s.chars().count(), SNIPPET_MAX_CHARS + 3);
        assert_eq!(chat_snippet("   ", true), MEDIA_ONLY_SNIPPET);
        assert_eq!(chat_snippet("hello", true), "hello");
    }

    #[test]
    fn unread_flag_covers_the_four_cases() {
        let mut participant = ConversationParticipant::new("me".to_string(), "member");
        let preview = |sender: &str, ms: i64| {
            Some(LastMessagePreview {
                message_uuid: "m".to_string(),
                sender_person_uuid: sender.to_string(),
                sender_display_name: "X".to_string(),
                snippet: "hi".to_string(),
                sent_at: DateTime::from_millis(ms),
                has_media: false,
            })
        };

        // never read, message from someone else -> unread
        assert!(unread_for_participant(&participant, &preview("other", 1000)));
        // own last message -> not unread
        assert!(!unread_for_participant(&participant, &preview("me", 1000)));
        // read after the last message -> not unread
        participant.last_read_at = Some(DateTime::from_millis(2000));
        assert!(!unread_for_participant(&participant, &preview("other", 1000)));
        // read before the last message -> unread
        participant.last_read_at = Some(DateTime::from_millis(500));
        assert!(unread_for_participant(&participant, &preview("other", 1000)));
    }

    #[test]
    fn other_direct_participant_returns_the_counterpart() {
        let c = Conversation::new_direct(
            "c1".to_string(),
            "direct:a:b".to_string(),
            "a".to_string(),
            "b".to_string(),
            "a".to_string(),
        );
        assert_eq!(other_direct_participant(&c, "a"), Some("b".to_string()));
        assert_eq!(other_direct_participant(&c, "b"), Some("a".to_string()));
        assert_eq!(other_direct_participant(&c, "z"), Some("a".to_string()));
    }

    #[test]
    fn resolve_send_as_business_only_for_acting_owner() {
        // owner acting in the matching profile session
        assert!(resolve_send_as_business(Some("bp-1"), Some("bp-1"), "owner", "owner"));
        // owner not in a profile session
        assert!(!resolve_send_as_business(None, Some("bp-1"), "owner", "owner"));
        // member, even in a profile session
        assert!(!resolve_send_as_business(Some("bp-1"), Some("bp-1"), "member", "owner"));
        // owner acting as a different profile
        assert!(!resolve_send_as_business(Some("bp-2"), Some("bp-1"), "owner", "owner"));
    }

    #[test]
    fn business_direct_member_resolution() {
        let r = roster("owner", &["m1", "m2"]);
        // owner must name an accepted member
        assert_eq!(
            resolve_business_direct_member(&r, "owner", Some("m1")).unwrap(),
            "m1"
        );
        assert_eq!(
            resolve_business_direct_member(&r, "owner", Some("stranger"))
                .unwrap_err()
                .kind,
            BusinessErrorKind::Forbidden
        );
        assert_eq!(
            resolve_business_direct_member(&r, "owner", None).unwrap_err().kind,
            BusinessErrorKind::Validation
        );
        // member gets their own thread
        assert_eq!(
            resolve_business_direct_member(&r, "m2", None).unwrap(),
            "m2"
        );
        // stranger rejected
        assert_eq!(
            resolve_business_direct_member(&r, "who", None).unwrap_err().kind,
            BusinessErrorKind::Forbidden
        );
    }
}
