use crate::http::json::chat_json::{ClientFrameJson, ServerEventJson};

/// The whole API speaks camelCase — REST, gRPC and the socket. Serde's
/// enum-level `rename_all` renames variants, not their fields, so each
/// variant needs its own. Without it every frame the app sends fails to
/// deserialize and is dropped, and the client never learns it happened.
#[test]
fn client_frames_parse_camel_case_fields() {
    let send: ClientFrameJson = serde_json::from_str(
        r#"{"type":"send","conversationUuid":"c1","body":"oi","media":[],"clientMessageId":"m1"}"#,
    )
    .expect("send frame");
    match send {
        ClientFrameJson::Send {
            conversation_uuid,
            client_message_id,
            body,
            ..
        } => {
            assert_eq!(conversation_uuid, "c1");
            assert_eq!(client_message_id, "m1");
            assert_eq!(body, "oi");
        }
        other => panic!("wrong variant: {other:?}"),
    }

    let read: ClientFrameJson = serde_json::from_str(
        r#"{"type":"read","conversationUuid":"c1","lastReadMessageUuid":"m9"}"#,
    )
    .expect("read frame");
    assert!(matches!(read, ClientFrameJson::Read { .. }));

    let typing: ClientFrameJson =
        serde_json::from_str(r#"{"type":"typing","conversationUuid":"c1"}"#)
            .expect("typing frame");
    assert!(matches!(typing, ClientFrameJson::Typing { .. }));
}

/// The Dart client reads these frames by camelCase key. Serde's enum-level
/// `rename_all` renames variants, not their fields, so each variant carries
/// its own — drop it and every socket frame silently lands in the wrong
/// conversation on the client.
#[test]
fn server_frames_use_camel_case_field_names() {
    let typing = ServerEventJson::Typing {
        conversation_uuid: "c1".to_string(),
        person_uuid: "p1".to_string(),
    }
    .to_frame();
    assert_eq!(typing, r#"{"type":"typing","conversationUuid":"c1","personUuid":"p1"}"#);

    let read = ServerEventJson::MessageRead {
        conversation_uuid: "c1".to_string(),
        person_uuid: "p1".to_string(),
        last_read_message_uuid: "m9".to_string(),
    }
    .to_frame();
    assert!(read.contains(r#""conversationUuid":"c1""#), "got {read}");
    assert!(read.contains(r#""lastReadMessageUuid":"m9""#), "got {read}");
}
