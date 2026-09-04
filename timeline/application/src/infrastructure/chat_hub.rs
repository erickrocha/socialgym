use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use serde::Serialize;
use tokio::sync::mpsc;

use crate::http::json::chat_json::{ConversationJson, MessageJson};

/// A frame pushed from the server to a connected chat client. Serialized as
/// `{ "type": "<kebab>", ...payload }`.
#[derive(Debug, Clone, Serialize)]
#[serde(tag = "type", rename_all = "kebab-case")]
pub enum ServerEvent {
    #[serde(rename = "message.new", rename_all = "camelCase")]
    MessageNew {
        conversation_uuid: String,
        conversation_type: String,
        message: MessageJson,
    },
    #[serde(rename = "conversation.updated")]
    ConversationUpdated { conversation: ConversationJson },
    #[serde(rename = "message.read", rename_all = "camelCase")]
    MessageRead {
        conversation_uuid: String,
        person_uuid: String,
        last_read_message_uuid: String,
    },
    #[serde(rename = "typing", rename_all = "camelCase")]
    Typing {
        conversation_uuid: String,
        person_uuid: String,
    },
    #[serde(rename = "pong")]
    Pong,
    #[serde(rename = "error")]
    Error { message: String },
}

impl ServerEvent {
    pub fn to_frame(&self) -> String {
        serde_json::to_string(self).unwrap_or_else(|_| "{\"type\":\"error\"}".to_string())
    }
}

type Sink = mpsc::UnboundedSender<String>;

/// In-process registry of live chat sockets, keyed by person uuid (one person
/// may have several — multiple devices/tabs). Fan-out is best-effort: a missed
/// frame is recovered by the client's REST reconnect replay, so the persisted
/// message stays the source of truth.
///
/// This only fans out within a single `timeline` process. Running more than one
/// instance needs a shared bus (Mongo change streams / Redis pub-sub) feeding
/// each node's hub.
#[derive(Clone)]
pub struct ChatHub {
    inner: Arc<Mutex<HashMap<String, Vec<(u64, Sink)>>>>,
    next_id: Arc<AtomicU64>,
}

impl Default for ChatHub {
    fn default() -> Self {
        Self::new()
    }
}

impl ChatHub {
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(HashMap::new())),
            next_id: Arc::new(AtomicU64::new(1)),
        }
    }

    /// Registers a new connection for `person_uuid`. Returns the connection id
    /// (pass to [`ChatHub::unregister`] on disconnect) and the receiver the
    /// socket task should forward to the client.
    pub fn register(&self, person_uuid: &str) -> (u64, mpsc::UnboundedReceiver<String>) {
        let (tx, rx) = mpsc::unbounded_channel();
        let id = self.next_id.fetch_add(1, Ordering::Relaxed);
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        guard.entry(person_uuid.to_string()).or_default().push((id, tx));
        (id, rx)
    }

    pub fn unregister(&self, person_uuid: &str, connection_id: u64) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        if let Some(sinks) = guard.get_mut(person_uuid) {
            sinks.retain(|(id, _)| *id != connection_id);
            if sinks.is_empty() {
                guard.remove(person_uuid);
            }
        }
    }

    /// Of `candidates`, returns those with at least one live connection to
    /// this node. Presence is per-node: with more than one instance behind the
    /// gateway a person connected elsewhere reads as offline here.
    pub fn online_among(&self, candidates: &[String]) -> Vec<String> {
        let guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        candidates
            .iter()
            .filter(|uuid| guard.get(*uuid).is_some_and(|sinks| !sinks.is_empty()))
            .cloned()
            .collect()
    }

    /// Sends `event` to every live connection of each recipient. Closed sinks
    /// are pruned in passing.
    pub fn publish(&self, recipients: &[String], event: &ServerEvent) {
        let frame = event.to_frame();
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        for recipient in recipients {
            let Some(sinks) = guard.get_mut(recipient) else {
                continue;
            };
            sinks.retain(|(_, tx)| tx.send(frame.clone()).is_ok());
            if sinks.is_empty() {
                guard.remove(recipient);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ChatHub;

    /// The Dart client reads these frames by camelCase key. Serde's enum-level
    /// `rename_all` renames variants, not their fields, so each variant carries
    /// its own — drop it and every socket frame silently lands in the wrong
    /// conversation on the client.
    #[test]
    fn frames_use_camel_case_field_names() {
        let typing = super::ServerEvent::Typing {
            conversation_uuid: "c1".to_string(),
            person_uuid: "p1".to_string(),
        }
        .to_frame();
        assert_eq!(
            typing,
            r#"{"type":"typing","conversationUuid":"c1","personUuid":"p1"}"#
        );

        let read = super::ServerEvent::MessageRead {
            conversation_uuid: "c1".to_string(),
            person_uuid: "p1".to_string(),
            last_read_message_uuid: "m9".to_string(),
        }
        .to_frame();
        assert!(read.contains(r#""conversationUuid":"c1""#), "got {read}");
        assert!(read.contains(r#""lastReadMessageUuid":"m9""#), "got {read}");
    }

    #[test]
    fn online_among_reports_only_connected_people() {
        let hub = ChatHub::new();
        let (conn, _rx_a) = hub.register("person-a");
        let (_, _rx_b) = hub.register("person-b");

        let mut online = hub.online_among(&[
            "person-a".to_string(),
            "person-b".to_string(),
            "person-c".to_string(),
        ]);
        online.sort();
        assert_eq!(online, vec!["person-a".to_string(), "person-b".to_string()]);

        // Last connection closing takes the person offline.
        hub.unregister("person-a", conn);
        assert_eq!(hub.online_among(&["person-a".to_string()]), Vec::<String>::new());

        // A second device keeps them online until every connection is gone.
        let (first, _rx1) = hub.register("person-d");
        let (_second, _rx2) = hub.register("person-d");
        hub.unregister("person-d", first);
        assert_eq!(hub.online_among(&["person-d".to_string()]), vec!["person-d".to_string()]);
    }
}
