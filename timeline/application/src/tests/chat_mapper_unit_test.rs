use super::{ClientFrameMapper, ServerEventMapper};
use crate::http::json::chat_json::{ClientFrameJson, ServerEventJson};
use business::commons::chat_hub::ChatEvent;
use business::use_cases::chat_session_use_case::ClientFrame;

#[test]
fn hub_events_without_payload_models_map_to_their_socket_frames() {
    let typing = ServerEventMapper::json(ChatEvent::Typing {
        conversation_uuid: "c1".into(),
        person_uuid: "p1".into(),
    });
    assert!(matches!(typing, ServerEventJson::Typing { conversation_uuid, person_uuid }
        if conversation_uuid == "c1" && person_uuid == "p1"));

    let read = ServerEventMapper::json(ChatEvent::MessageRead {
        conversation_uuid: "c1".into(),
        person_uuid: "p1".into(),
        last_read_message_uuid: "m1".into(),
    });
    assert!(matches!(read, ServerEventJson::MessageRead { last_read_message_uuid, .. }
        if last_read_message_uuid == "m1"));

    assert!(matches!(ServerEventMapper::json(ChatEvent::Pong), ServerEventJson::Pong));
    let error = ServerEventMapper::json(ChatEvent::Error { message: "no".into() });
    assert!(matches!(error, ServerEventJson::Error { message } if message == "no"));
}

#[test]
fn socket_frames_map_to_the_chat_session_frames() {
    let send = ClientFrameMapper::domain(ClientFrameJson::Send {
        conversation_uuid: "c1".into(),
        body: "oi".into(),
        media: vec![],
        client_message_id: "m1".into(),
    });
    assert!(matches!(send, ClientFrame::Send { body, client_message_id, media, .. }
        if body == "oi" && client_message_id == "m1" && media.is_empty()));

    let read = ClientFrameMapper::domain(ClientFrameJson::Read {
        conversation_uuid: "c1".into(),
        last_read_message_uuid: "m9".into(),
    });
    assert!(matches!(read, ClientFrame::Read { last_read_message_uuid, .. } if last_read_message_uuid == "m9"));

    let typing = ClientFrameMapper::domain(ClientFrameJson::Typing { conversation_uuid: "c1".into() });
    assert!(matches!(typing, ClientFrame::Typing { conversation_uuid } if conversation_uuid == "c1"));
    assert!(matches!(ClientFrameMapper::domain(ClientFrameJson::Ping), ClientFrame::Ping));
}
