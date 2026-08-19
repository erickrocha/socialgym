use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::profile_entity::{ActiveModel, ProfileEntity};

#[derive(Debug, Clone)]
pub struct Profile {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub person_id: i32,
    pub person_uuid: String,
    pub business_profile_id: i32,
    pub business_profile_uuid: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct ProfileEntityMapper {}

impl Profile {
    pub fn new(
        person_id: i32,
        person_uuid: String,
        business_profile_id: i32,
        business_profile_uuid: String,
    ) -> Self {
        Self {
            id: None,
            uuid: None,
            person_id,
            person_uuid,
            business_profile_id,
            business_profile_uuid,
            created_at: Some(chrono::Utc::now().naive_utc()),
            updated_at: Some(chrono::Utc::now().naive_utc()),
        }
    }
}

impl EntityMapper<Profile, ProfileEntity, ActiveModel> for ProfileEntityMapper {
    fn build_active_model(d: Profile) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => sea_orm::Set(id),
                None => sea_orm::NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => sea_orm::Set(string_to_uuid(uuid.as_str())),
                None => sea_orm::NotSet,
            },
            person_id: sea_orm::Set(d.person_id),
            person_uuid: sea_orm::Set(string_to_uuid(d.person_uuid.as_str())),
            business_profile_id: sea_orm::Set(d.business_profile_id),
            business_profile_uuid: sea_orm::Set(string_to_uuid(d.business_profile_uuid.as_str())),
            created_at: sea_orm::NotSet,
            updated_at: sea_orm::NotSet,
        }
    }

    fn from_model(e: ProfileEntity) -> Profile {
        Profile {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            person_id: e.person_id,
            person_uuid: uuid_to_string(e.person_uuid),
            business_profile_id: e.business_profile_id,
            business_profile_uuid: uuid_to_string(e.business_profile_uuid),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> Profile {
        Profile {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            person_id: e.person_id.unwrap(),
            person_uuid: uuid_to_string(e.person_uuid.unwrap()),
            business_profile_id: e.business_profile_id.unwrap(),
            business_profile_uuid: uuid_to_string(e.business_profile_uuid.unwrap()),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}
