//! Conversions between the gRPC messages and the domain structs. Dates travel as the ISO-8601 text
//! REST puts in JSON, so both transports show the same value.
use crate::proto::timeline as pb;
use business::commons::chat_hub::ChatEvent;
use business::commons::data_tools::{
    bson_datetime_to_naive, naive_to_bson_datetime, opt_bson_datetime_to_naive,
    opt_naive_to_bson_datetime,
};
use business::use_cases::chat_session_use_case::ClientFrame;
use business::use_cases::chat_use_case::ConversationView;
use chrono::NaiveDateTime;
use domain::body_composition::BodyComposition;
use domain::circumferences::{Biceps, Circumferences, Thighs};
use domain::comment::Comment;
use domain::content_report::{ContentReport, ModerationEvent};
use domain::conversation::{Conversation, LastMessagePreview};
use domain::enums::{Category, MediaType, Visibility};
use domain::evolution_check_in::EvolutionCheckIn;
use domain::exercise::Exercise;
use domain::in_app_notification::InAppNotification;
use domain::media::Media;
use domain::mention::Mention;
use domain::message::{Message, MessageMedia};
use domain::post::Post;
use domain::reaction::Reaction;
use domain::workout_session::WorkoutSession;
use mongodb::bson::DateTime;
use tonic::Status;
use uuid::Uuid;

/// The text REST puts in JSON for a date.
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

fn bson_to_text(date: DateTime) -> String {
    date_to_text(bson_datetime_to_naive(date))
}

fn bson_to_rfc3339(date: DateTime) -> String {
    date.try_to_rfc3339_string().unwrap_or_default()
}

fn id_or_new(id: Option<String>) -> String {
    id.unwrap_or_else(|| Uuid::new_v4().to_string())
}

// ── Post ───────────────────────────────────────────────────────────────────────

pub struct MentionMapper {}

impl MentionMapper {
    pub fn domain(m: pb::Mention) -> Mention {
        Mention { name: m.name, mentioned_uuid: m.mentioned_uuid }
    }

    pub fn proto(m: Mention) -> pb::Mention {
        pb::Mention { name: m.name, mentioned_uuid: m.mentioned_uuid }
    }
}

pub struct MediaMapper {}

impl MediaMapper {
    pub fn domain(m: pb::Media) -> Media {
        Media {
            uuid: id_or_new(m.uuid),
            url: m.url,
            media_type: MediaType::from_string(&m.media_type),
            object_key: m.object_key,
        }
    }

    pub fn proto(m: Media) -> pb::Media {
        pb::Media { uuid: Some(m.uuid), url: m.url, media_type: m.media_type.to_string(), object_key: m.object_key }
    }
}

pub struct ReactionMapper {}

impl ReactionMapper {
    pub fn proto(r: Reaction) -> pb::Reaction {
        pb::Reaction {
            uuid: Some(r.uuid),
            author_id: r.author_id,
            author_name: r.author_name,
            reaction_type: r.reaction_type.to_string(),
        }
    }
}

pub struct CommentMapper {}

impl CommentMapper {
    /// A new comment: the author is filled in from the token by the use case.
    pub fn domain(request: pb::AddCommentRequest) -> Comment {
        Comment::new(
            Uuid::new_v4().to_string(),
            request.post_uuid,
            String::new(),
            String::new(),
            None,
            None,
            request.content,
            request.parent_uuid,
            request.mentions.into_iter().map(MentionMapper::domain).collect(),
        )
    }

    pub fn proto(c: Comment) -> pb::Comment {
        pb::Comment {
            uuid: Some(c.uuid),
            post_uuid: c.post_uuid,
            author_uuid: c.author_uuid,
            author_name: c.author_name,
            author_object_key: c.author_object_key,
            author_avatar: c.author_avatar,
            content: c.content,
            parent_uuid: c.parent_uuid,
            created_at: Some(bson_to_text(c.created_at)),
            updated_at: Some(bson_to_text(c.updated_at)),
            mentions: c.mentions.into_iter().map(MentionMapper::proto).collect(),
        }
    }
}

pub struct PostMapper {}

impl PostMapper {
    /// A new post: the author is filled in from the token by the use case.
    pub fn domain(request: pb::CreatePostRequest) -> Post {
        Post::new(
            0,
            String::new(),
            String::new(),
            None,
            None,
            request.content,
            request.media.into_iter().map(MediaMapper::domain).collect(),
            request.mentions.into_iter().map(MentionMapper::domain).collect(),
        )
    }

    pub fn proto(p: Post) -> pb::Post {
        pb::Post {
            uuid: Some(p.uuid),
            author_id: p.author_id,
            author_uuid: p.author_uuid,
            author_name: p.author_name,
            author_object_key: p.author_object_key,
            author_avatar: p.author_avatar,
            content: p.content,
            media: p.media.into_iter().map(MediaMapper::proto).collect(),
            reactions: p.reactions.into_iter().map(ReactionMapper::proto).collect(),
            comments: p.comments.into_iter().map(CommentMapper::proto).collect(),
            created_at: Some(bson_to_text(p.created_at)),
            updated_at: Some(bson_to_text(p.updated_at)),
            mentions: p.mentions.into_iter().map(MentionMapper::proto).collect(),
        }
    }
}

// ── Notification ───────────────────────────────────────────────────────────────

pub struct NotificationMapper {}

impl NotificationMapper {
    pub fn proto(n: InAppNotification) -> pb::Notification {
        pb::Notification {
            uuid: Some(n.uuid),
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
            created_at: bson_to_text(n.created_at),
            updated_at: bson_to_text(n.updated_at),
        }
    }
}

// ── Evolution check-in ─────────────────────────────────────────────────────────

pub struct EvolutionCheckInMapper {}

impl EvolutionCheckInMapper {
    #[allow(clippy::result_large_err)]
    pub fn domain(request: pb::AddEvolutionCheckInRequest, person_uuid: &str) -> Result<EvolutionCheckIn, Status> {
        Ok(EvolutionCheckIn::new(
            Uuid::new_v4().to_string(),
            person_uuid.to_owned(),
            naive_to_bson_datetime(text_to_date(&request.created_at)?),
            request.note,
            Visibility::from_string(&request.visibility),
            request.composition.map(|c| {
                BodyComposition::new(id_or_new(c.uuid), c.weight, c.body_fat_pct, c.muscle_mass_pct, c.visceral_fat)
            }),
            request.circumferences.map(|c| {
                Circumferences::new(
                    id_or_new(c.uuid),
                    c.neck,
                    c.chest,
                    c.waist,
                    c.abdomen,
                    c.hip,
                    Biceps::new(c.biceps_right, c.biceps_left),
                    Thighs::new(c.thigh_right, c.thigh_left),
                )
            }),
        ))
    }

    pub fn proto(e: EvolutionCheckIn) -> pb::EvolutionCheckIn {
        pb::EvolutionCheckIn {
            uuid: Some(e.uuid),
            person_uuid: e.person_uuid,
            created_at: bson_to_text(e.created_at),
            note: e.note,
            visibility: e.visibility.to_string(),
            composition: e.composition.map(|c| pb::BodyComposition {
                uuid: Some(c.uuid),
                weight: c.weight,
                body_fat_pct: c.body_fat_pct,
                muscle_mass_pct: c.muscle_mass_pct,
                visceral_fat: c.visceral_fat,
            }),
            circumferences: e.circumferences.map(|c| pb::Circumferences {
                uuid: Some(c.uuid),
                neck: c.neck,
                chest: c.chest,
                waist: c.waist,
                abdomen: c.abdomen,
                hip: c.hip,
                biceps_right: c.biceps.right,
                biceps_left: c.biceps.left,
                thigh_right: c.thigh.right,
                thigh_left: c.thigh.left,
            }),
        }
    }
}

// ── Chat ───────────────────────────────────────────────────────────────────────

pub struct MessageMapper {}

impl MessageMapper {
    pub fn media_domain(m: pb::MessageMedia) -> MessageMedia {
        MessageMedia { media_type: m.media_type, object_key: m.object_key, url: String::new() }
    }

    fn media_proto(m: MessageMedia) -> pb::MessageMedia {
        pb::MessageMedia { media_type: m.media_type, object_key: m.object_key, url: m.url }
    }

    pub fn proto(m: Message) -> pb::Message {
        pb::Message {
            uuid: m.uuid,
            conversation_uuid: m.conversation_uuid,
            sender_person_uuid: m.sender_person_uuid,
            sender_kind: m.sender_kind,
            sender_display_name: m.sender_display_name,
            sender_avatar_url: m.sender_object_key.filter(|s| !s.is_empty()),
            sender_business_profile_uuid: m.sender_business_profile_uuid,
            body: m.body,
            media: m.media.into_iter().map(Self::media_proto).collect(),
            client_message_id: m.client_message_id,
            sent_at: bson_to_text(m.sent_at),
        }
    }
}

pub struct ConversationMapper {}

impl ConversationMapper {
    fn preview(p: LastMessagePreview) -> pb::LastMessagePreview {
        pb::LastMessagePreview {
            message_uuid: p.message_uuid,
            sender_person_uuid: p.sender_person_uuid,
            sender_display_name: p.sender_display_name,
            snippet: p.snippet,
            sent_at: bson_to_text(p.sent_at),
            has_media: p.has_media,
        }
    }

    pub fn proto(view: ConversationView) -> pb::Conversation {
        let ConversationView { conversation, unread } = view;
        let Conversation {
            uuid,
            conversation_type,
            business_profile_uuid,
            business_profile_name,
            business_profile_logo_object_key,
            participant_person_uuids,
            participants,
            last_message,
            created_at,
            updated_at,
            ..
        } = conversation;
        pb::Conversation {
            uuid,
            conversation_type,
            business_profile_uuid,
            business_profile_name,
            // `ChatUseCase` already replaced the object key with a signed URL (or left it empty).
            business_profile_logo_url: business_profile_logo_object_key.filter(|s| !s.is_empty()),
            participant_person_uuids,
            participants: participants
                .into_iter()
                .map(|p| pb::ConversationParticipant {
                    person_uuid: p.person_uuid,
                    role: p.role,
                    last_read_at: p.last_read_at.map(bson_to_text),
                    last_read_message_uuid: p.last_read_message_uuid,
                })
                .collect(),
            last_message: last_message.map(Self::preview),
            unread,
            created_at: bson_to_text(created_at),
            updated_at: bson_to_text(updated_at),
        }
    }
}

pub struct ChatFrameMapper {}

impl ChatFrameMapper {
    /// The gRPC form of an event the hub fans out.
    pub fn server_frame(event: ChatEvent) -> pb::ServerFrame {
        use pb::server_frame::Event;
        let event = match event {
            ChatEvent::MessageNew { conversation_uuid, conversation_type, message } => {
                Event::MessageNew(pb::MessageNewEvent {
                    conversation_uuid,
                    conversation_type,
                    message: Some(MessageMapper::proto(message)),
                })
            }
            ChatEvent::ConversationUpdated { conversation } => {
                Event::ConversationUpdated(pb::ConversationUpdatedEvent {
                    conversation: Some(ConversationMapper::proto(conversation)),
                })
            }
            ChatEvent::MessageRead { conversation_uuid, person_uuid, last_read_message_uuid } => {
                Event::MessageRead(pb::MessageReadEvent { conversation_uuid, person_uuid, last_read_message_uuid })
            }
            ChatEvent::Typing { conversation_uuid, person_uuid } => {
                Event::Typing(pb::TypingEvent { conversation_uuid, person_uuid })
            }
            ChatEvent::Pong => Event::Pong(pb::PongEvent {}),
            ChatEvent::Error { message } => Event::Error(pb::ErrorEvent { message }),
        };
        pb::ServerFrame { event: Some(event) }
    }

    /// What a frame the stream client sent asks the chat use cases to do; `None` for an empty frame.
    pub fn client_frame(frame: pb::ClientFrame) -> Option<ClientFrame> {
        use pb::client_frame::Frame;
        Some(match frame.frame? {
            Frame::Send(f) => ClientFrame::Send {
                conversation_uuid: f.conversation_uuid,
                body: f.body,
                media: f.media.into_iter().map(MessageMapper::media_domain).collect(),
                client_message_id: f.client_message_id,
            },
            Frame::Read(f) => {
                ClientFrame::Read { conversation_uuid: f.conversation_uuid, last_read_message_uuid: f.last_read_message_uuid }
            }
            Frame::Typing(f) => ClientFrame::Typing { conversation_uuid: f.conversation_uuid },
            Frame::Ping(_) => ClientFrame::Ping,
        })
    }
}

// ── Content report ─────────────────────────────────────────────────────────────

pub struct ContentReportMapper {}

impl ContentReportMapper {
    pub fn proto(r: ContentReport) -> pb::ContentReport {
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
                    created_at: bson_to_rfc3339(e.created_at),
                })
                .collect(),
            created_at: bson_to_rfc3339(r.created_at),
            updated_at: bson_to_rfc3339(r.updated_at),
        }
    }
}

// ── Workout session ────────────────────────────────────────────────────────────

pub struct WorkoutSessionMapper {}

impl WorkoutSessionMapper {
    /// The session a client sent. Both dates and each set's dates and owner name are required
    /// (`INVALID_ARGUMENT` otherwise); the client's session id and person are ignored.
    #[allow(clippy::result_large_err)]
    pub fn domain(s: pb::WorkoutSession) -> Result<WorkoutSession, Status> {
        let incomplete = || Status::invalid_argument("session dates and set owner names are required");
        let started_at = optional_text_to_date(s.started_at.as_deref())?;
        let completed_at = optional_text_to_date(s.completed_at.as_deref())?;
        let mut exercises = Vec::with_capacity(s.executed_sets.len());
        let mut sets_complete = true;
        for e in s.executed_sets {
            let (set_start, set_end) =
                (optional_text_to_date(e.started_at.as_deref())?, optional_text_to_date(e.completed_at.as_deref())?);
            let (Some(set_start), Some(set_end), Some(owner_name)) = (set_start, set_end, e.owner_name) else {
                sets_complete = false;
                continue;
            };
            exercises.push(Exercise {
                uuid: id_or_new(e.uuid),
                exercise_name: e.exercise_name,
                owner_id: e.owner_id,
                owner_name,
                category: e.category.map_or(Category::Force, |c| Category::from_string(&c)),
                visibility: e.visibility.map_or(Visibility::Public, |v| Visibility::from_string(&v)),
                set_number: e.set_number,
                reps_or_duration: e.reps_or_duration,
                weight: e.weight,
                started_at: opt_naive_to_bson_datetime(set_start),
                completed_at: opt_naive_to_bson_datetime(set_end),
            });
        }
        let (Some(started_at), Some(completed_at), true) = (started_at, completed_at, sets_complete) else {
            return Err(incomplete());
        };
        Ok(WorkoutSession {
            uuid: Uuid::new_v4().to_string(),
            person_uuid: Some(String::new()),
            workout_name: s.workout_name,
            duration: s.duration,
            started_at: opt_naive_to_bson_datetime(started_at),
            day_of_week: s.day_of_week,
            completed_at: opt_naive_to_bson_datetime(completed_at),
            exercises,
            total_volume: s.total_volume,
            total_sets: s.total_sets,
        })
    }

    pub fn proto(s: WorkoutSession) -> pb::WorkoutSession {
        let date = |d: Option<DateTime>| d.and_then(opt_bson_datetime_to_naive).map(date_to_text);
        pb::WorkoutSession {
            uuid: Some(s.uuid),
            person_uuid: s.person_uuid.unwrap_or_default(),
            workout_name: s.workout_name,
            duration: s.duration,
            started_at: date(s.started_at),
            day_of_week: s.day_of_week,
            completed_at: date(s.completed_at),
            executed_sets: s
                .exercises
                .into_iter()
                .map(|e| pb::ExecutedSet {
                    uuid: Some(e.uuid),
                    exercise_name: e.exercise_name,
                    owner_id: e.owner_id,
                    owner_name: Some(e.owner_name),
                    category: Some(e.category.to_string()),
                    visibility: Some(e.visibility.to_string()),
                    set_number: e.set_number,
                    reps_or_duration: e.reps_or_duration,
                    weight: e.weight,
                    started_at: date(e.started_at),
                    completed_at: date(e.completed_at),
                })
                .collect(),
            total_volume: s.total_volume,
            total_sets: s.total_sets,
        }
    }
}

#[cfg(test)]
#[path = "../tests/mapper_unit_test.rs"]
mod tests;
