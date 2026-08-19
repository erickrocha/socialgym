use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::person_info_entity::{ActiveModel, PersonInfoEntity};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct PersonInfo {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub person_id: i32,
    pub biography: Option<String>,
    pub relationship: Option<String>,
    pub job: Option<String>,
    pub home_town: Option<String>,
    pub current_city: Option<String>,
    pub weight: Option<f32>,
    pub height: Option<f32>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct PersonInfoEntityMapper {}

impl EntityMapper<PersonInfo, PersonInfoEntity, ActiveModel> for PersonInfoEntityMapper {
    fn build_active_model(d: PersonInfo) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(&uuid)),
                None => NotSet,
            },
            person_id: Set(d.person_id),
            biography: Set(d.biography),
            relationship: Set(d.relationship),
            job: Set(d.job),
            home_town: Set(d.home_town),
            current_city: Set(d.current_city),
            weight: Set(d.weight),
            height: Set(d.height),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: PersonInfoEntity) -> PersonInfo {
        PersonInfo {
            id: Some(e.id),
            person_id: e.person_id,
            biography: e.biography,
            relationship: e.relationship,
            job: e.job,
            home_town: e.home_town,
            current_city: e.current_city,
            weight: e.weight,
            height: e.height,
            uuid: Some(uuid_to_string(e.uuid)),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> PersonInfo {
        PersonInfo {
            id: Some(e.id.unwrap()),
            person_id: e.person_id.unwrap(),
            biography: e.biography.unwrap(),
            relationship: e.relationship.unwrap(),
            job: e.job.unwrap(),
            home_town: e.home_town.unwrap(),
            current_city: e.current_city.unwrap(),
            weight: e.weight.unwrap(),
            height: e.height.unwrap(),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl PersonInfo {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        person_id: i32,
        biography: Option<String>,
        relationship: Option<String>,
        job: Option<String>,
        home_town: Option<String>,
        current_city: Option<String>,
        weight: Option<f32>,
        height: Option<f32>,
    ) -> PersonInfo {
        Self::update(
            None,
            person_id,
            biography,
            relationship,
            job,
            home_town,
            current_city,
            weight,
            height,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        person_id: i32,
        biography: Option<String>,
        relationship: Option<String>,
        job: Option<String>,
        home_town: Option<String>,
        current_city: Option<String>,
        weight: Option<f32>,
        height: Option<f32>,
    ) -> PersonInfo {
        PersonInfo {
            id,
            person_id,
            biography,
            relationship,
            job,
            home_town,
            current_city,
            weight,
            height,
            uuid: None,
            created_at: None,
            updated_at: None,
        }
    }
}
