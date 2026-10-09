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
        accepted_member_person_uuids: members.iter().map(|member| member.to_string()).collect(),
    }
}

fn image(key: &str) -> MessageMedia {
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
fn page_skip_matches_page_times_size_and_saturates() {
    assert_eq!(page_skip(0, 20), 0);
    assert_eq!(page_skip(1, 20), 20);
    assert_eq!(page_skip(u32::MAX, 30), u64::from(u32::MAX) * 30);
}

#[test]
fn validate_message_requires_text_or_image_and_trims_text() {
    assert_eq!(
        validate_message("   ", &[]).unwrap_err().kind,
        BusinessErrorKind::Validation
    );
    assert_eq!(validate_message("  hi ", &[]).unwrap(), "hi");
    assert_eq!(validate_message("", &[image("k")]).unwrap(), "");
}

#[test]
fn validate_message_rejects_oversized_body_non_image_and_too_many_media_items() {
    let long = "x".repeat(MAX_BODY_LEN + 1);
    assert_eq!(
        validate_message(&long, &[]).unwrap_err().kind,
        BusinessErrorKind::Validation
    );

    let video = MessageMedia {
        media_type: "Video".to_string(),
        object_key: "k".to_string(),
        url: String::new(),
    };
    assert_eq!(
        validate_message("ok", &[video]).unwrap_err().kind,
        BusinessErrorKind::Validation
    );

    let too_many: Vec<MessageMedia> = (0..MAX_MEDIA_PER_MESSAGE + 1)
        .map(|index| image(&index.to_string()))
        .collect();
    assert_eq!(
        validate_message("ok", &too_many).unwrap_err().kind,
        BusinessErrorKind::Validation
    );
}

#[test]
fn snippet_truncates_unicode_and_marks_media_only_messages() {
    let long = "é".repeat(SNIPPET_MAX_CHARS + 10);
    let snippet = chat_snippet(&long, false);
    assert!(snippet.ends_with("..."));
    assert_eq!(snippet.chars().count(), SNIPPET_MAX_CHARS + 3);
    assert_eq!(chat_snippet("   ", true), MEDIA_ONLY_SNIPPET);
    assert_eq!(chat_snippet("hello", true), "hello");
}

#[test]
fn unread_flag_covers_sender_and_read_timestamp_cases() {
    let mut participant = ConversationParticipant::new("me".to_string(), "member");
    let preview = |sender: &str, timestamp_ms| {
        Some(LastMessagePreview {
            message_uuid: "m".to_string(),
            sender_person_uuid: sender.to_string(),
            sender_display_name: "X".to_string(),
            snippet: "hi".to_string(),
            sent_at: DateTime::from_millis(timestamp_ms),
            has_media: false,
        })
    };

    assert!(unread_for_participant(
        &participant,
        &preview("other", 1000)
    ));
    assert!(!unread_for_participant(&participant, &preview("me", 1000)));
    participant.last_read_at = Some(DateTime::from_millis(2000));
    assert!(!unread_for_participant(
        &participant,
        &preview("other", 1000)
    ));
    participant.last_read_at = Some(DateTime::from_millis(500));
    assert!(unread_for_participant(
        &participant,
        &preview("other", 1000)
    ));
}

#[test]
fn resolves_other_direct_participant_for_both_member_orders_and_unknown_caller() {
    let conversation = Conversation::new_direct(
        "c1".to_string(),
        "direct:a:b".to_string(),
        "a".to_string(),
        "b".to_string(),
        "a".to_string(),
    );
    assert_eq!(
        other_direct_participant(&conversation, "a"),
        Some("b".to_string())
    );
    assert_eq!(
        other_direct_participant(&conversation, "b"),
        Some("a".to_string())
    );
    assert_eq!(
        other_direct_participant(&conversation, "z"),
        Some("a".to_string())
    );
}

#[test]
fn only_profile_owner_can_send_as_the_matching_business_profile() {
    assert!(resolve_send_as_business(
        Some("bp-1"),
        Some("bp-1"),
        "owner",
        "owner"
    ));
    assert!(!resolve_send_as_business(
        None,
        Some("bp-1"),
        "owner",
        "owner"
    ));
    assert!(!resolve_send_as_business(
        Some("bp-1"),
        Some("bp-1"),
        "member",
        "owner"
    ));
    assert!(!resolve_send_as_business(
        Some("bp-2"),
        Some("bp-1"),
        "owner",
        "owner"
    ));
}

#[test]
fn business_direct_member_resolution_enforces_team_membership() {
    let team = roster("owner", &["m1", "m2"]);
    assert_eq!(
        resolve_business_direct_member(&team, "owner", Some("m1")).unwrap(),
        "m1"
    );
    assert_eq!(
        resolve_business_direct_member(&team, "owner", Some("stranger"))
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
    assert_eq!(
        resolve_business_direct_member(&team, "owner", None)
            .unwrap_err()
            .kind,
        BusinessErrorKind::Validation
    );
    assert_eq!(
        resolve_business_direct_member(&team, "m2", None).unwrap(),
        "m2"
    );
    assert_eq!(
        resolve_business_direct_member(&team, "who", None)
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
}
