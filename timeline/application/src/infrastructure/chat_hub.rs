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
    #[serde(rename = "message.new")]
    MessageNew {
        conversation_uuid: String,
        conversation_type: String,
        message: MessageJson,
    },
    #[serde(rename = "conversation.updated")]
    ConversationUpdated { conversation: ConversationJson },
    #[serde(rename = "message.read")]
    MessageRead {
        conversation_uuid: String,
        person_uuid: String,
        last_read_message_uuid: String,
    },
    #[serde(rename = "typing")]
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
