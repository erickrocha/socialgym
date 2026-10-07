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
