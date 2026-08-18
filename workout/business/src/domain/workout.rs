use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
pub use crate::domain::enums::Visibility;
use crate::domain::exercise::Exercise;
use chrono::NaiveDateTime;
use entity::workout::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct Workout {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub owner_id: i32,
    pub owner_uuid: String,
    pub name: String,
    pub description: Option<String>,
    pub difficulty: String,
    pub muscle_group: String,
    pub exercises: Vec<Exercise>,
    pub visibility: Visibility,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct WorkoutEntityMapper {}

impl EntityMapper<Workout, Model, ActiveModel> for WorkoutEntityMapper {
    fn build_active_model(d: Workout) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(&uuid)),
                None => NotSet,
            },
            owner_id: Set(d.owner_id),
            owner_uuid: Set(string_to_uuid(&d.owner_uuid)),
            name: Set(d.name),
            description: Set(d.description),
            difficulty: Set(d.difficulty),
            muscle_group: Set(d.muscle_group),
            visibility: Set(d.visibility.to_string()),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: Model) -> Workout {
        Workout {
            id: Some(e.id),
            name: e.name,
            description: e.description,
            difficulty: e.difficulty,
            muscle_group: e.muscle_group,
            owner_id: e.owner_id,
            owner_uuid: uuid_to_string(e.owner_uuid),
            exercises: Vec::new(),
            uuid: Some(uuid_to_string(e.uuid)),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
            visibility: Visibility::from_string(e.visibility.as_str()),
        }
    }

    fn from_active_model(e: ActiveModel) -> Workout {
        Workout {
            id: Some(e.id.unwrap()),
            name: e.name.unwrap(),
            description: e.description.unwrap(),
            difficulty: e.difficulty.unwrap(),
            muscle_group: e.muscle_group.unwrap(),
            owner_id: e.owner_id.unwrap(),
            owner_uuid: uuid_to_string(e.owner_uuid.unwrap()),
            exercises: Vec::new(),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
            visibility: Visibility::from_string(e.visibility.unwrap().as_str()),
        }
    }
}

impl Workout {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner_id: i32,
        owner_uuid: String,
        name: String,
        description: Option<String>,
        difficulty: String,
        muscle_group: String,
        exercises: Vec<Exercise>,
        visibility: Visibility,
    ) -> Workout {
        Self::update(
            None,
            None,
            owner_id,
            owner_uuid,
            name,
            description,
            difficulty,
            muscle_group,
            exercises,
            visibility,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        uuid: Option<String>,
        owner_id: i32,
        owner_uuid: String,
        name: String,
        description: Option<String>,
        difficulty: String,
        muscle_group: String,
        exercises: Vec<Exercise>,
        visibility: Visibility,
    ) -> Workout {
        Workout {
            id,
            uuid,
            owner_id,
            owner_uuid,
            name,
            description,
            difficulty,
            muscle_group,
            visibility,
            exercises,
            created_at: None,
            updated_at: None,
        }
    }
}
