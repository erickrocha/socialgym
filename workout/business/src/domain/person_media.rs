use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use crate::domain::enums::MimeType;
use chrono::NaiveDateTime;
use entity::person_media_entity::{ActiveModel, PersonMediaEntity};

#[derive(Debug, Clone)]
pub struct PersonMedia {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub person_id: i32,
    pub person_uuid: String,
    pub mime_type: MimeType,
    pub album: String,
    pub s3_key: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct PersonMediaEntityMapper {}

impl EntityMapper<PersonMedia, PersonMediaEntity, ActiveModel> for PersonMediaEntityMapper {
    fn build_active_model(d: PersonMedia) -> ActiveModel {
        ActiveModel {
            person_id: sea_orm::Set(d.person_id),
            person_uuid: sea_orm::Set(string_to_uuid(&d.person_uuid)),
            mime_type: sea_orm::Set(d.mime_type.to_string()),
            album: sea_orm::Set(d.album),
            s3_key: sea_orm::Set(d.s3_key),
            ..Default::default()
        }
    }

    fn from_model(e: PersonMediaEntity) -> PersonMedia {
        PersonMedia {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            person_id: e.person_id,
            person_uuid: uuid_to_string(e.person_uuid),
            mime_type: MimeType::from_string(&e.mime_type),
            album: e.album,
            s3_key: e.s3_key,
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> PersonMedia {
        PersonMedia {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            person_id: e.person_id.unwrap(),
            person_uuid: uuid_to_string(e.person_uuid.unwrap()),
            mime_type: MimeType::from_string(&e.mime_type.unwrap()),
            album: e.album.unwrap(),
            s3_key: e.s3_key.unwrap(),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl PersonMedia {
    pub fn new(
        person_id: i32,
        person_uuid: String,
        mime_type: MimeType,
        album: String,
        s3_key: String,
    ) -> PersonMedia {
        PersonMedia {
            id: None,
            uuid: None,
            person_id,
            person_uuid,
            mime_type,
            album,
            s3_key,
            created_at: None,
            updated_at: None,
        }
    }

    pub fn update(
        id: Option<i32>,
        uuid: Option<String>,
        person_id: i32,
        person_uuid: String,
        mime_type: MimeType,
        album: String,
        s3_key: String,
    ) -> PersonMedia {
        PersonMedia {
            id,
            uuid,
            person_id,
            person_uuid,
            mime_type,
            album,
            s3_key,
            created_at: None,
            updated_at: None,
        }
    }
}
