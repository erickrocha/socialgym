//! Conversions between the REST JSON types and the gRPC messages. gRPC handlers build the same
//! JSON types REST does and reuse its mappers, so the two transports carry identical data.
use crate::http::json::body_composition_json::BodyCompositionJson;
use crate::http::json::circumferences_json::CircumferencesJson;
use crate::http::json::evolution_check_in_json::EvolutionCheckInJson;
use crate::http::json::notification_json::NotificationJson;
use crate::http::json::post_json::{CommentJson, MediaJson, MentionJson, PostJson, ReactionJson};
use business::proto::proto::timeline as pb;
use chrono::NaiveDateTime;
use tonic::Status;

/// The text REST puts in JSON for a date, so both transports show the same value.
pub fn date_to_text(date: NaiveDateTime) -> String {
    serde_json::to_value(date).ok().and_then(|v| v.as_str().map(str::to_owned)).unwrap_or_default()
}

#[allow(clippy::result_large_err)]
pub fn text_to_date(text: &str) -> Result<NaiveDateTime, Status> {
    serde_json::from_value(serde_json::Value::String(text.to_owned()))
        .map_err(|_| Status::invalid_argument("invalid date, expected ISO-8601"))
}

#[allow(clippy::result_large_err)]
pub fn optional_text_to_date(text: Option<&str>) -> Result<Option<NaiveDateTime>, Status> {
    text.map(text_to_date).transpose()
}

// ── requests -> JSON ───────────────────────────────────────────────────────────

pub fn media_from(m: pb::Media) -> MediaJson {
    MediaJson { uuid: m.uuid, url: m.url, media_type: m.media_type, object_key: m.object_key }
}

pub fn mention_from(m: pb::Mention) -> MentionJson {
    MentionJson { name: m.name, mentioned_uuid: m.mentioned_uuid }
}

pub fn post_from(request: pb::CreatePostRequest) -> PostJson {
    PostJson {
        uuid: None,
        author_id: 0,
        author_uuid: String::new(),
        author_name: String::new(),
        author_object_key: None,
        author_avatar: None,
        content: request.content,
        media: request.media.into_iter().map(media_from).collect(),
        reactions: Vec::new(),
        comments: Vec::new(),
        created_at: None,
        updated_at: None,
        mentions: request.mentions.into_iter().map(mention_from).collect(),
        third_party_consent_confirmed: request.third_party_consent_confirmed,
    }
}

pub fn comment_from(request: pb::AddCommentRequest) -> CommentJson {
    CommentJson {
        uuid: None,
        post_uuid: request.post_uuid,
        author_uuid: String::new(),
        author_name: String::new(),
        author_object_key: None,
        author_avatar: None,
        content: request.content,
        parent_uuid: request.parent_uuid,
        created_at: None,
        updated_at: None,
        mentions: request.mentions.into_iter().map(mention_from).collect(),
    }
}

#[allow(clippy::result_large_err)]
pub fn check_in_from(request: pb::AddEvolutionCheckInRequest, person_uuid: &str) -> Result<EvolutionCheckInJson, Status> {
    Ok(EvolutionCheckInJson {
        uuid: None,
        person_uuid: person_uuid.to_owned(),
        created_at: text_to_date(&request.created_at)?,
        note: request.note,
        visibility: request.visibility,
        composition: request.composition.map(|c| BodyCompositionJson {
            uuid: c.uuid,
            weight: c.weight,
            body_fat_pct: c.body_fat_pct,
            muscle_mass_pct: c.muscle_mass_pct,
            visceral_fat: c.visceral_fat,
        }),
        circumferences: request.circumferences.map(|c| CircumferencesJson {
            uuid: c.uuid,
            neck: c.neck,
            chest: c.chest,
            waist: c.waist,
            abdomen: c.abdomen,
            hip: c.hip,
            biceps_right: c.biceps_right,
            biceps_left: c.biceps_left,
            thigh_right: c.thigh_right,
            thigh_left: c.thigh_left,
        }),
    })
}

// ── JSON -> responses ──────────────────────────────────────────────────────────

fn media_to(m: MediaJson) -> pb::Media {
    pb::Media { uuid: m.uuid, url: m.url, media_type: m.media_type, object_key: m.object_key }
}

fn reaction_to(r: ReactionJson) -> pb::Reaction {
    pb::Reaction { uuid: r.uuid, author_id: r.author_id, author_name: r.author_name, reaction_type: r.reaction_type }
}

fn mention_to(m: MentionJson) -> pb::Mention {
    pb::Mention { name: m.name, mentioned_uuid: m.mentioned_uuid }
}

fn comment_to(c: CommentJson) -> pb::Comment {
    pb::Comment {
        uuid: c.uuid,
        post_uuid: c.post_uuid,
        author_uuid: c.author_uuid,
        author_name: c.author_name,
        author_object_key: c.author_object_key,
        author_avatar: c.author_avatar,
        content: c.content,
        parent_uuid: c.parent_uuid,
        created_at: c.created_at.map(date_to_text),
        updated_at: c.updated_at.map(date_to_text),
        mentions: c.mentions.into_iter().map(mention_to).collect(),
    }
}

pub fn post_to(p: PostJson) -> pb::Post {
    pb::Post {
        uuid: p.uuid,
        author_id: p.author_id,
        author_uuid: p.author_uuid,
        author_name: p.author_name,
        author_object_key: p.author_object_key,
        author_avatar: p.author_avatar,
        content: p.content,
        media: p.media.into_iter().map(media_to).collect(),
        reactions: p.reactions.into_iter().map(reaction_to).collect(),
        comments: p.comments.into_iter().map(comment_to).collect(),
        created_at: p.created_at.map(date_to_text),
        updated_at: p.updated_at.map(date_to_text),
        mentions: p.mentions.into_iter().map(mention_to).collect(),
    }
}

pub fn notification_to(n: NotificationJson) -> pb::Notification {
    pb::Notification {
        uuid: n.uuid,
        notification_type: n.notification_type,
        recipient_person_uuid: n.recipient_person_uuid,
        actor_person_uuid: n.actor_person_uuid,
        actor_name: n.actor_name,
        post_uuid: n.post_uuid,
        comment_uuid: n.comment_uuid,
        entity_type: n.entity_type,
        entity_uuid: n.entity_uuid,
        target_type: n.target_type,
        target_uuid: n.target_uuid,
        snippet: n.snippet,
        read: n.read,
        created_at: date_to_text(n.created_at),
        updated_at: date_to_text(n.updated_at),
    }
}

pub fn check_in_to(e: EvolutionCheckInJson) -> pb::EvolutionCheckIn {
    pb::EvolutionCheckIn {
        uuid: e.uuid,
        person_uuid: e.person_uuid,
        created_at: date_to_text(e.created_at),
        note: e.note,
        visibility: e.visibility,
        composition: e.composition.map(|c| pb::BodyComposition {
            uuid: c.uuid,
            weight: c.weight,
            body_fat_pct: c.body_fat_pct,
            muscle_mass_pct: c.muscle_mass_pct,
            visceral_fat: c.visceral_fat,
        }),
        circumferences: e.circumferences.map(|c| pb::Circumferences {
            uuid: c.uuid,
            neck: c.neck,
            chest: c.chest,
            waist: c.waist,
            abdomen: c.abdomen,
            hip: c.hip,
            biceps_right: c.biceps_right,
            biceps_left: c.biceps_left,
            thigh_right: c.thigh_right,
            thigh_left: c.thigh_left,
        }),
    }
}

// ── chat ───────────────────────────────────────────────────────────────────────

use crate::http::json::chat_json::{ConversationJson, LastMessagePreviewJson, MessageJson, MessageMediaJson};

pub fn message_media_to(m: MessageMediaJson) -> pb::MessageMedia {
    pb::MessageMedia { media_type: m.media_type, object_key: m.object_key, url: m.url }
}

pub fn message_media_from(m: pb::MessageMedia) -> MessageMediaJson {
    MessageMediaJson { media_type: m.media_type, object_key: m.object_key, url: m.url }
}

pub fn message_to(m: MessageJson) -> pb::Message {
    pb::Message {
        uuid: m.uuid,
        conversation_uuid: m.conversation_uuid,
        sender_person_uuid: m.sender_person_uuid,
        sender_kind: m.sender_kind,
        sender_display_name: m.sender_display_name,
        sender_avatar_url: m.sender_avatar_url,
        sender_business_profile_uuid: m.sender_business_profile_uuid,
        body: m.body,
        media: m.media.into_iter().map(message_media_to).collect(),
        client_message_id: m.client_message_id,
        sent_at: date_to_text(m.sent_at),
    }
}

fn preview_to(p: LastMessagePreviewJson) -> pb::LastMessagePreview {
    pb::LastMessagePreview {
        message_uuid: p.message_uuid,
        sender_person_uuid: p.sender_person_uuid,
        sender_display_name: p.sender_display_name,
        snippet: p.snippet,
        sent_at: date_to_text(p.sent_at),
        has_media: p.has_media,
    }
}

pub fn conversation_to(c: ConversationJson) -> pb::Conversation {
    pb::Conversation {
        uuid: c.uuid,
        conversation_type: c.conversation_type,
        business_profile_uuid: c.business_profile_uuid,
        business_profile_name: c.business_profile_name,
        business_profile_logo_url: c.business_profile_logo_url,
        participant_person_uuids: c.participant_person_uuids,
        participants: c
            .participants
            .into_iter()
            .map(|p| pb::ConversationParticipant {
                person_uuid: p.person_uuid,
                role: p.role,
                last_read_at: p.last_read_at.map(date_to_text),
                last_read_message_uuid: p.last_read_message_uuid,
            })
            .collect(),
        last_message: c.last_message.map(preview_to),
        unread: c.unread,
        created_at: date_to_text(c.created_at),
        updated_at: date_to_text(c.updated_at),
    }
}

/// Turns a frame the hub fans out (JSON text, the WebSocket's wire form) into the gRPC event, so
/// the hub stays transport-neutral. Unknown shapes are dropped.
pub fn server_frame_from_hub_text(text: &str) -> Option<pb::ServerFrame> {
    use pb::server_frame::Event;
    let value: serde_json::Value = serde_json::from_str(text).ok()?;
    let text_of = |key: &str| value.get(key).and_then(|v| v.as_str()).unwrap_or_default().to_string();
    let event = match value.get("type")?.as_str()? {
        "message.new" => Event::MessageNew(pb::MessageNewEvent {
            conversation_uuid: text_of("conversationUuid"),
            conversation_type: text_of("conversationType"),
            message: Some(message_to(serde_json::from_value(value.get("message")?.clone()).ok()?)),
        }),
        "conversation.updated" => Event::ConversationUpdated(pb::ConversationUpdatedEvent {
            conversation: Some(conversation_to(serde_json::from_value(value.get("conversation")?.clone()).ok()?)),
        }),
        "message.read" => Event::MessageRead(pb::MessageReadEvent {
            conversation_uuid: text_of("conversationUuid"),
            person_uuid: text_of("personUuid"),
            last_read_message_uuid: text_of("lastReadMessageUuid"),
        }),
        "typing" => Event::Typing(pb::TypingEvent {
            conversation_uuid: text_of("conversationUuid"),
            person_uuid: text_of("personUuid"),
        }),
        "pong" => Event::Pong(pb::PongEvent {}),
        "error" => Event::Error(pb::ErrorEvent { message: text_of("message") }),
        _ => return None,
    };
    Some(pb::ServerFrame { event: Some(event) })
}

/// The client frame the shared session handler understands; `None` for an empty frame.
pub fn client_frame_from(frame: pb::ClientFrame) -> Option<crate::infrastructure::chat_session::ClientFrame> {
    use crate::infrastructure::chat_session::ClientFrame as Shared;
    use pb::client_frame::Frame;
    Some(match frame.frame? {
        Frame::Send(f) => Shared::Send {
            conversation_uuid: f.conversation_uuid,
            body: f.body,
            media: f.media.into_iter().map(message_media_from).collect(),
            client_message_id: f.client_message_id,
        },
        Frame::Read(f) => Shared::Read { conversation_uuid: f.conversation_uuid, last_read_message_uuid: f.last_read_message_uuid },
        Frame::Typing(f) => Shared::Typing { conversation_uuid: f.conversation_uuid },
        Frame::Ping(_) => Shared::Ping,
    })
}

// ── reports and workout sessions ───────────────────────────────────────────────

use crate::http::json::exercise_json::ExerciseJson;
use crate::http::json::workout_session_json::WorkoutSessionJson;
use domain::content_report::{ContentReport, ModerationEvent};

fn bson_text(date: mongodb::bson::DateTime) -> String {
    date.try_to_rfc3339_string().unwrap_or_default()
}

pub fn report_to(r: ContentReport) -> pb::ContentReport {
    pb::ContentReport {
        uuid: r.uuid,
        target_type: r.target_type,
        target_id: r.target_id,
        post_id: r.post_id,
        reporter_person_uuid: r.reporter_person_uuid,
        reason: r.reason,
        details: r.details,
        priority: r.priority,
        status: r.status,
        assigned_moderator_uuid: r.assigned_moderator_uuid,
        decision: r.decision,
        removal_reason: r.removal_reason,
        history: r
            .history
            .into_iter()
            .map(|e: ModerationEvent| pb::ModerationEvent {
                actor_person_uuid: e.actor_person_uuid,
                action: e.action,
                reason: e.reason,
                created_at: bson_text(e.created_at),
            })
            .collect(),
        created_at: bson_text(r.created_at),
        updated_at: bson_text(r.updated_at),
    }
}

#[allow(clippy::result_large_err)]
pub fn session_from(s: pb::WorkoutSession) -> Result<WorkoutSessionJson, Status> {
    let sets = s
        .executed_sets
        .into_iter()
        .map(|e| {
            Ok(ExerciseJson {
                uuid: e.uuid,
                exercise_name: e.exercise_name,
                owner_id: e.owner_id,
                owner_name: e.owner_name,
                category: e.category,
                visibility: e.visibility,
                set_number: e.set_number,
                reps_or_duration: e.reps_or_duration,
                weight: e.weight,
                started_at: optional_text_to_date(e.started_at.as_deref())?,
                completed_at: optional_text_to_date(e.completed_at.as_deref())?,
            })
        })
        .collect::<Result<Vec<_>, Status>>()?;
    Ok(WorkoutSessionJson {
        uuid: None,
        person_uuid: String::new(),
        workout_name: s.workout_name,
        duration: s.duration,
        started_at: optional_text_to_date(s.started_at.as_deref())?,
        day_of_week: s.day_of_week,
        completed_at: optional_text_to_date(s.completed_at.as_deref())?,
        executed_sets: sets,
        total_volume: s.total_volume,
        total_sets: s.total_sets,
    })
}

pub fn session_to(s: WorkoutSessionJson) -> pb::WorkoutSession {
    pb::WorkoutSession {
        uuid: s.uuid,
        person_uuid: s.person_uuid,
        workout_name: s.workout_name,
        duration: s.duration,
        started_at: s.started_at.map(date_to_text),
        day_of_week: s.day_of_week,
        completed_at: s.completed_at.map(date_to_text),
        executed_sets: s
            .executed_sets
            .into_iter()
            .map(|e| pb::ExecutedSet {
                uuid: e.uuid,
                exercise_name: e.exercise_name,
                owner_id: e.owner_id,
                owner_name: e.owner_name,
                category: e.category,
                visibility: e.visibility,
                set_number: e.set_number,
                reps_or_duration: e.reps_or_duration,
                weight: e.weight,
                started_at: e.started_at.map(date_to_text),
                completed_at: e.completed_at.map(date_to_text),
            })
            .collect(),
        total_volume: s.total_volume,
        total_sets: s.total_sets,
    }
}
