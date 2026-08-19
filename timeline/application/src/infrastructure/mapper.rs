use crate::http::json::body_composition_json::BodyCompositionJson;
use crate::http::json::circumferences_json::CircumferencesJson;
use crate::http::json::evolution_check_in_json::EvolutionCheckInJson;
use crate::http::json::exercise_json::ExerciseJson;
use crate::http::json::notification_json::NotificationJson;
use crate::http::json::post_json::{CommentJson, MediaJson, MentionJson, PostJson, ReactionJson};
use crate::http::json::workout_session_json::WorkoutSessionJson;
use crate::infrastructure::data_tools::{bson_datetime_to_naive, naive_to_bson_datetime, opt_bson_datetime_to_naive, opt_naive_to_bson_datetime};
use domain::body_composition::BodyComposition;
use domain::circumferences::{Biceps, Circumferences, Thighs};
use domain::comment::Comment;
use domain::enums::{Category, MediaType, ReactionType, Visibility};
use domain::evolution_check_in::EvolutionCheckIn;
use domain::exercise::Exercise;
use domain::in_app_notification::InAppNotification;
use domain::media::Media;
use domain::post::Post;
use domain::reaction::Reaction;
use domain::workout_session::WorkoutSession;
use uuid::Uuid;
use domain::mention::Mention;

pub trait Mapper<T, U> {
    fn json(t: T) -> U;
    fn domain(u: U) -> T;

    fn json_vec(t: Vec<T>) -> Vec<U>{
        t.into_iter().map(Self::json).collect()
    }

    fn domain_vec(u: Vec<U>) -> Vec<T>{
        u.into_iter().map(Self::domain).collect()
    }
}

// ── Workout ───────────────────────────────────────────────────────────────────

pub struct WorkoutMapper {}

impl Mapper<WorkoutSession, WorkoutSessionJson> for WorkoutMapper {
    fn json(t: WorkoutSession) -> WorkoutSessionJson {
        WorkoutSessionJson {
            uuid: Some(t.uuid),
            person_uuid: t.person_uuid.unwrap(),
            workout_name: t.workout_name,
            duration: t.duration,
            started_at: opt_bson_datetime_to_naive(t.started_at.unwrap()),
            day_of_week: t.day_of_week,
            completed_at: opt_bson_datetime_to_naive(t.completed_at.unwrap()),
            executed_sets: t.exercises.into_iter().map(ExerciseMapper::json).collect(),
            total_volume: t.total_volume,
            total_sets: t.total_sets,
        }
    }

    fn domain(u: WorkoutSessionJson) -> WorkoutSession {
        WorkoutSession {
            uuid: match &u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            person_uuid: Some(u.person_uuid.to_string()),
            workout_name: u.workout_name,
            duration: u.duration,
            started_at: opt_naive_to_bson_datetime(u.started_at.unwrap()),
            day_of_week: u.day_of_week,
            completed_at: opt_naive_to_bson_datetime(u.completed_at.unwrap()),
            exercises: ExerciseMapper::domain_vec(u.executed_sets),
            total_volume: u.total_volume,
            total_sets: u.total_sets,
        }
    }
}

// ── Exercise ──────────────────────────────────────────────────────────────────

pub struct ExerciseMapper {}

impl Mapper<Exercise, ExerciseJson> for ExerciseMapper {
    fn json(entity: Exercise) -> ExerciseJson {
        ExerciseJson {
            uuid: Some(entity.uuid),
            exercise_name: entity.exercise_name,
            owner_id: entity.owner_id,
            owner_name: Some(entity.owner_name),
            category: Some(entity.category.to_string()),
            visibility: Some(entity.visibility.to_string()),
            set_number: entity.set_number,
            reps_or_duration: entity.reps_or_duration,
            weight: entity.weight,
            started_at: opt_bson_datetime_to_naive(entity.started_at.unwrap()),
            completed_at: opt_bson_datetime_to_naive(entity.completed_at.unwrap()),
        }
    }

    fn domain(json: ExerciseJson) -> Exercise {
        Exercise {
            uuid: match &json.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            exercise_name: json.exercise_name,
            owner_id: json.owner_id,
            owner_name: json.owner_name.unwrap(),
            category: match json.category {
                Some(category) => Category::from_string(category.as_str()),
                None => Category::Force,
            },
            visibility: match json.visibility {
                Some(visibility) => Visibility::from_string(visibility.as_str()),
                None => Visibility::Public,
            },
            set_number: json.set_number,
            reps_or_duration: json.reps_or_duration,
            weight: json.weight,
            started_at: opt_naive_to_bson_datetime(json.started_at.unwrap()),
            completed_at: opt_naive_to_bson_datetime(json.completed_at.unwrap()),
        }
    }
}

// ── Media ─────────────────────────────────────────────────────────────────────

pub struct MediaMapper {}

impl Mapper<Media, MediaJson> for MediaMapper {
    fn json(t: Media) -> MediaJson {
        MediaJson {
            uuid: Some(t.uuid),
            url: t.url,
            media_type: t.media_type.to_string(),
            object_key: t.object_key,
        }
    }

    fn domain(u: MediaJson) -> Media {
        Media {
            uuid: match &u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            url: u.url,
            media_type: MediaType::from_string(&u.media_type),
            object_key: u.object_key,
        }
    }
}

// ── Reaction ──────────────────────────────────────────────────────────────────

pub struct ReactionMapper {}

impl Mapper<Reaction, ReactionJson> for ReactionMapper {
    fn json(t: Reaction) -> ReactionJson {
        ReactionJson {
            uuid: Some(t.uuid),
            author_id: t.author_id,
            author_name: t.author_name,
            reaction_type: t.reaction_type.to_string(),
        }
    }

    fn domain(u: ReactionJson) -> Reaction {
        Reaction {
            uuid: match u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            author_id: u.author_id,
            author_name: u.author_name,
            reaction_type: ReactionType::from_string(&u.reaction_type),
        }
    }

    fn json_vec(t: Vec<Reaction>) -> Vec<ReactionJson> {
        t.into_iter().map(Self::json).collect()
    }

    fn domain_vec(u: Vec<ReactionJson>) -> Vec<Reaction> {
        u.into_iter().map(Self::domain).collect()
    }
}

// ── Comment ───────────────────────────────────────────────────────────────────

pub struct CommentMapper {}

impl Mapper<Comment, CommentJson> for CommentMapper {
    fn json(t: Comment) -> CommentJson {
        CommentJson {
            uuid: Some(t.uuid),
            post_uuid: t.post_uuid,
            author_uuid: t.author_uuid,
            author_name: t.author_name,
            author_object_key: t.author_object_key,
            author_avatar: t.author_avatar,
            content: t.content,
            parent_uuid: t.parent_uuid,
            created_at: opt_bson_datetime_to_naive(t.created_at),
            updated_at: opt_bson_datetime_to_naive(t.updated_at),
            mentions: MentionMapper::json_vec(t.mentions),
        }
    }

    fn domain(u: CommentJson) -> Comment {
        let now = mongodb::bson::DateTime::now();
        Comment {
            uuid: match u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            post_uuid: u.post_uuid,
            author_uuid: u.author_uuid,
            author_name: u.author_name,
            author_object_key: u.author_object_key,
            author_avatar: u.author_avatar,
            content: u.content,
            parent_uuid: u.parent_uuid,
            created_at: u.created_at.and_then(opt_naive_to_bson_datetime).unwrap_or(now),
            updated_at: u.updated_at.and_then(opt_naive_to_bson_datetime).unwrap_or(now),
            mentions: MentionMapper::domain_vec(u.mentions),
        }
    }
}

impl CommentMapper {
    /// Convert comment to JSON, resolving the avatar URL from the shared CloudFront cache.
    pub fn json_with_avatar(
        comment: Comment,
        url_cache: &std::collections::HashMap<String, String>,
    ) -> CommentJson {
        let author_avatar_url = comment
            .author_object_key
            .as_ref()
            .and_then(|key| url_cache.get(key).cloned());

        CommentJson {
            uuid: Some(comment.uuid),
            post_uuid: comment.post_uuid,
            author_uuid: comment.author_uuid,
            author_name: comment.author_name,
            author_object_key: comment.author_object_key,
            author_avatar: author_avatar_url,
            content: comment.content,
            parent_uuid: comment.parent_uuid,
            created_at: opt_bson_datetime_to_naive(comment.created_at),
            updated_at: opt_bson_datetime_to_naive(comment.updated_at),
            mentions: MentionMapper::json_vec(comment.mentions),
        }
    }
}

// ── Post ─────────────────────────────────────────────────────────────────────

pub struct PostMapper {}

impl Mapper<Post, PostJson> for PostMapper {
    fn json(t: Post) -> PostJson {
        PostJson {
            uuid: Some(t.uuid),
            author_id: t.author_id,
            author_uuid: t.author_uuid,
            author_name: t.author_name,
            author_object_key: t.author_object_key,
            author_avatar: t.author_avatar,
            content: t.content,
            media: t.media.into_iter().map(MediaMapper::json).collect(),
            reactions: ReactionMapper::json_vec(t.reactions),
            comments: CommentMapper::json_vec(t.comments),
            mentions: MentionMapper::json_vec(t.mentions),
            created_at: opt_bson_datetime_to_naive(t.created_at),
            updated_at: opt_bson_datetime_to_naive(t.updated_at),
        }
    }

    fn domain(u: PostJson) -> Post {
        let now = mongodb::bson::DateTime::now();
        Post {
            uuid: match u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            author_id: u.author_id,
            author_uuid: u.author_uuid,
            author_name: u.author_name,
            author_object_key: u.author_object_key,
            author_avatar: u.author_avatar,
            content: u.content,
            media: u.media.into_iter().map(MediaMapper::domain).collect(),
            reactions: ReactionMapper::domain_vec(u.reactions),
            comments: CommentMapper::domain_vec(u.comments),
            mentions: MentionMapper::domain_vec(u.mentions),
            created_at: u.created_at.and_then(opt_naive_to_bson_datetime).unwrap_or(now),
            updated_at: u.updated_at.and_then(opt_naive_to_bson_datetime).unwrap_or(now),
        }
    }
}

impl PostMapper {
    
    /// Convert post to JSON, replacing media and comment avatar URLs
    /// from the shared CloudFront signed-URL cache.
    pub fn json_with_avatars(
        post: Post,
        url_cache: &std::collections::HashMap<String, String>,
    ) -> PostJson {
        PostJson {
            uuid: Some(post.uuid),
            author_id: post.author_id,
            author_uuid: post.author_uuid,
            author_name: post.author_name,
            author_object_key: post.author_object_key.clone(),
            author_avatar: post
                .author_object_key
                .as_ref()
                .and_then(|k| url_cache.get(k).cloned())
                .or(post.author_avatar),
            content: post.content,
            media: post
                .media
                .into_iter()
                .map(|m| {
                    let url = url_cache.get(&m.object_key).cloned().unwrap_or(m.url);
                    MediaJson {
                        uuid: Some(m.uuid),
                        url,
                        media_type: m.media_type.to_string(),
                        object_key: m.object_key,
                    }
                })
                .collect(),
            reactions:ReactionMapper::json_vec(post.reactions),
            comments: post
                .comments
                .into_iter()
                .map(|c| CommentMapper::json_with_avatar(c, url_cache))
                .collect(),
            created_at: opt_bson_datetime_to_naive(post.created_at),
            updated_at: opt_bson_datetime_to_naive(post.updated_at),
            mentions: MentionMapper::json_vec(post.mentions),
        }
    }
}

pub struct BodyCompositeMapper {}

impl Mapper<BodyComposition, BodyCompositionJson> for BodyCompositeMapper {
    fn json(t: BodyComposition) -> BodyCompositionJson {
        BodyCompositionJson::new(Some(t.uuid), t.weight, t.body_fat_pct, t.muscle_mass_pct, t.visceral_fat)
    }

    fn domain(u: BodyCompositionJson) -> BodyComposition {
        BodyComposition::new(match u.uuid {
            Some(uuid) => uuid.to_string(),
            None => Uuid::new_v4().to_string(),
        }, u.weight, u.body_fat_pct, u.muscle_mass_pct, u.visceral_fat)
    }
}

pub struct CircumferencesMapper {}

impl Mapper<Circumferences, CircumferencesJson> for CircumferencesMapper {
    fn json(t: Circumferences) -> CircumferencesJson {
        CircumferencesJson::new(Some(t.uuid),
            t.neck,
            t.chest,
            t.waist,
            t.abdomen,
            t.hip,
            t.biceps.right,
            t.biceps.left,
            t.thigh.right,
            t.thigh.left,
        )
    }

    fn domain(u: CircumferencesJson) -> Circumferences {
        Circumferences::new(
            match u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            u.neck,
            u.chest,
            u.waist,
            u.abdomen,
            u.hip,
            Biceps::new(u.biceps_right, u.biceps_left),
            Thighs::new(u.thigh_right, u.thigh_left),
        )
    }
}

pub struct EvolutionCheckInMapper {}

pub struct NotificationMapper {}

impl Mapper<EvolutionCheckIn, EvolutionCheckInJson> for EvolutionCheckInMapper {
    fn json(t: EvolutionCheckIn) -> EvolutionCheckInJson {
        EvolutionCheckInJson::new(
            Some(t.uuid),
            t.person_uuid,
            bson_datetime_to_naive(t.created_at),
            t.note,
            t.visibility.to_string(),
            t.composition.map(BodyCompositeMapper::json),
            t.circumferences.map(CircumferencesMapper::json),
        )
    }

    fn domain(u: EvolutionCheckInJson) -> EvolutionCheckIn {
        EvolutionCheckIn::new(
            match u.uuid {
                Some(uuid) => uuid.to_string(),
                None => Uuid::new_v4().to_string(),
            },
            u.person_uuid,
            naive_to_bson_datetime(u.created_at),
            u.note,
            Visibility::from_string(&u.visibility),
            u.composition.map(BodyCompositeMapper::domain),
            u.circumferences.map(CircumferencesMapper::domain),
        )
    }
}

impl Mapper<InAppNotification, NotificationJson> for NotificationMapper {
    fn json(t: InAppNotification) -> NotificationJson {
        NotificationJson {
            uuid: Some(t.uuid),
            notification_type: t.notification_type,
            recipient_person_uuid: t.recipient_person_uuid,
            actor_person_uuid: t.actor_person_uuid,
            actor_name: t.actor_name,
            post_uuid: t.post_uuid,
            comment_uuid: t.comment_uuid,
            entity_type: t.entity_type,
            entity_uuid: t.entity_uuid,
            snippet: t.snippet,
            read: t.read,
            created_at: bson_datetime_to_naive(t.created_at),
            updated_at: bson_datetime_to_naive(t.updated_at),
        }
    }

    fn domain(u: NotificationJson) -> InAppNotification {
        InAppNotification {
            uuid: u.uuid.unwrap_or_else(|| Uuid::new_v4().to_string()),
            notification_type: u.notification_type,
            recipient_person_uuid: u.recipient_person_uuid,
            actor_person_uuid: u.actor_person_uuid,
            actor_name: u.actor_name,
            post_uuid: u.post_uuid,
            comment_uuid: u.comment_uuid,
            entity_type: u.entity_type,
            entity_uuid: u.entity_uuid,
            snippet: u.snippet,
            read: u.read,
            created_at: naive_to_bson_datetime(u.created_at),
            updated_at: naive_to_bson_datetime(u.updated_at),
        }
    }
}

pub struct MentionMapper {}

impl Mapper<Mention,MentionJson> for MentionMapper {
    fn json(u: Mention) -> MentionJson {
        MentionJson {
            name: u.name,
            mentioned_uuid: u.mentioned_uuid,
        }
    }

    fn domain(t: MentionJson) -> Mention {
        Mention {
            name: t.name,
            mentioned_uuid: t.mentioned_uuid,
        }
    }
}
