use std::collections::HashMap;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::{Arc, Mutex};

use domain::message::Message;
use tokio::sync::mpsc;

use crate::use_cases::chat_use_case::ConversationView;

/// An event pushed from the server to a connected chat client. It carries domain data only: each
/// transport (WebSocket, gRPC stream) renders it in its own wire form.
#[derive(Debug, Clone)]
pub enum ChatEvent {
    MessageNew {
        conversation_uuid: String,
        conversation_type: String,
        message: Message,
    },
    ConversationUpdated {
        conversation: ConversationView,
    },
    MessageRead {
        conversation_uuid: String,
        person_uuid: String,
        last_read_message_uuid: String,
    },
    Typing {
        conversation_uuid: String,
        person_uuid: String,
    },
    Pong,
    Error {
        message: String,
    },
}

type Sink = mpsc::UnboundedSender<ChatEvent>;

/// In-process registry of live chat connections, keyed by person uuid (one person
/// may have several — multiple devices/tabs). Fan-out is best-effort: a missed
/// event is recovered by the client's REST reconnect replay, so the persisted
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
    /// connection task should forward to the client.
    pub fn register(&self, person_uuid: &str) -> (u64, mpsc::UnboundedReceiver<ChatEvent>) {
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
    pub fn publish(&self, recipients: &[String], event: &ChatEvent) {
        let mut guard = self.inner.lock().unwrap_or_else(|e| e.into_inner());
        for recipient in recipients {
            let Some(sinks) = guard.get_mut(recipient) else {
                continue;
            };
            sinks.retain(|(_, tx)| tx.send(event.clone()).is_ok());
            if sinks.is_empty() {
                guard.remove(recipient);
            }
        }
    }
}

#[cfg(test)]
#[path = "../tests/chat_hub_unit_test.rs"]
mod tests;
