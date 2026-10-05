use super::*;

#[test]
fn unwrap_sns_envelope_passthrough_for_plain_s3_event() {
    let body = r#"{"Records":[{"eventName":"ObjectCreated:Put","s3":{"bucket":{"name":"b"},"object":{"key":"k","size":1}}}]}"#;
    assert_eq!(SqsConsumerUseCase::unwrap_sns_envelope(body), body);
}

#[test]
fn unwrap_sns_envelope_extracts_inner_message() {
    let inner = r#"{"Records":[{"eventName":"ObjectCreated:Put","s3":{"bucket":{"name":"b"},"object":{"key":"k","size":1}}}]}"#;
    let envelope = serde_json::json!({
        "Type": "Notification",
        "MessageId": "abc",
        "Message": inner,
    })
    .to_string();

    assert_eq!(SqsConsumerUseCase::unwrap_sns_envelope(&envelope), inner);
}

#[test]
fn key_segments_are_parsed_correctly() {
    let key = "person/uuid-1234/avatar/file-uuid-5678";
    let parts: Vec<&str> = key.splitn(4, '/').collect();
    assert_eq!(parts, ["person", "uuid-1234", "avatar", "file-uuid-5678"]);
}

#[test]
fn key_with_wrong_prefix_is_detected() {
    let key = "uploads/owner-1/some-uuid.jpg";
    let parts: Vec<&str> = key.splitn(4, '/').collect();
    assert_ne!(parts[0], "person");
}

#[test]
fn mime_type_from_content_type_roundtrips() {
    for content_type in ["image/jpeg", "video/mp4", "application/octet-stream"] {
        assert_eq!(
            MimeType::from_content_type(content_type).to_string(),
            content_type
        );
    }
}
