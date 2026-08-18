use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
pub use crate::domain::enums::{Category, Visibility};
use chrono::NaiveDateTime;
use entity::exercise::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct Exercise {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub name: String,
    pub description: Option<String>,
    pub sets: i32,
    pub owner_id: i32,
    pub owner_uuid: String,
    pub owner_name: String,
    pub category: Category,
    pub reps_or_duration: i32,
    pub visibility: Visibility,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct ExerciseEntityMapper {}

impl EntityMapper<Exercise, Model, ActiveModel> for ExerciseEntityMapper {
    fn build_active_model(d: Exercise) -> ActiveModel {
        ActiveModel {
            id: NotSet,
            uuid: NotSet,
            name: Set(d.name),
            owner_id: Set(d.owner_id),
            owner_uuid: Set(string_to_uuid(d.owner_uuid.as_str())),
            owner_name: Set(d.owner_name),
            description: Set(d.description),
            sets: Set(d.sets),
            category: Set(d.category.to_text()),
            reps_or_duration: Set(d.reps_or_duration),
            visibility: Set(d.visibility.to_string()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: Model) -> Exercise {
        Exercise {
            id: Some(e.id),
            name: e.name,
            description: e.description,
            owner_id: e.owner_id,
            owner_name: e.owner_name,
            owner_uuid: uuid_to_string(e.owner_uuid),
            sets: e.sets,
            category: Category::from_string(e.category.as_str()),
            reps_or_duration: e.reps_or_duration,
            uuid: Some(uuid_to_string(e.uuid)),
            visibility: Visibility::from_string(e.visibility.as_str()),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> Exercise {
        Exercise {
            id: Some(e.id.unwrap()),
            name: e.name.unwrap(),
            owner_id: e.owner_id.unwrap(),
            owner_uuid: uuid_to_string(e.owner_uuid.unwrap()),
            owner_name: e.owner_name.unwrap(),
            description: e.description.unwrap(),
            sets: e.sets.unwrap(),
            category: Category::from_string(e.category.unwrap().as_str()),
            reps_or_duration: e.reps_or_duration.unwrap(),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            visibility: Visibility::from_string(e.visibility.unwrap().as_str()),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl Exercise {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        name: String,
        owner_id: i32,
        owner_uuid: String,
        owner_name: String,
        description: Option<String>,
        sets: i32,
        reps_or_duration: i32,
        category: Category,
        visibility: Visibility,
    ) -> Exercise {
        Self::update(
            None,
            name,
            owner_id,
            owner_uuid,
            owner_name,
            description,
            category,
            sets,
            reps_or_duration,
            visibility,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        name: String,
        owner_id: i32,
        owner_uuid: String,
        owner_name: String,
        description: Option<String>,
        category: Category,
        sets: i32,
        reps_or_duration: i32,
        visibility: Visibility,
    ) -> Exercise {
        Exercise {
            id,
            name,
            description,
            owner_id,
            owner_uuid,
            owner_name,
            sets,
            reps_or_duration,
            category,
            uuid: None,
            visibility,
            created_at: None,
            updated_at: None,
        }
    }
}
