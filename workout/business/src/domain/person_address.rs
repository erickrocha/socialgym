use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::person_address_entity::{ActiveModel, PersonAddressEntity};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct PersonAddress {
    pub id: Option<i32>,
    pub person_id: i32,
    pub uuid: Option<String>,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: Option<String>,
    pub country_code: String,
    pub current: bool,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct PersonAddressEntityMapper {}

impl EntityMapper<PersonAddress, PersonAddressEntity, ActiveModel> for PersonAddressEntityMapper {
    fn build_active_model(d: PersonAddress) -> ActiveModel {
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
            address_line1: Set(d.address_line1),
            address_line2: Set(d.address_line2),
            locality: Set(d.locality),
            administrative_area: Set(d.administrative_area),
            postal_code: Set(d.postal_code),
            country_code: Set(d.country_code),
            current: Set(d.current),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: PersonAddressEntity) -> PersonAddress {
        PersonAddress {
            id: Some(e.id),
            person_id: e.person_id,
            address_line1: e.address_line1,
            address_line2: e.address_line2,
            locality: e.locality,
            administrative_area: e.administrative_area,
            postal_code: e.postal_code,
            country_code: e.country_code,
            current: e.current,
            uuid: Some(uuid_to_string(e.uuid)),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> PersonAddress {
        PersonAddress {
            id: Some(e.id.unwrap()),
            person_id: e.person_id.unwrap(),
            address_line1: e.address_line1.unwrap(),
            address_line2: e.address_line2.unwrap(),
            locality: e.locality.unwrap(),
            administrative_area: e.administrative_area.unwrap(),
            postal_code: e.postal_code.unwrap(),
            country_code: e.country_code.unwrap(),
            current: e.current.unwrap(),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl PersonAddress {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        person_id: i32,
        address_line1: String,
        address_line2: Option<String>,
        locality: String,
        administrative_area: String,
        postal_code: Option<String>,
        country_code: String,
        current: bool,
    ) -> PersonAddress {
        Self::update(
            None,
            person_id,
            address_line1,
            address_line2,
            locality,
            administrative_area,
            postal_code,
            country_code,
            current,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        person_id: i32,
        address_line1: String,
        address_line2: Option<String>,
        locality: String,
        administrative_area: String,
        postal_code: Option<String>,
        country_code: String,
        current: bool,
    ) -> PersonAddress {
        PersonAddress {
            id,
            person_id,
            address_line1,
            address_line2,
            locality,
            administrative_area,
            postal_code,
            country_code,
            current,
            uuid: None,
            created_at: None,
            updated_at: None,
        }
    }
}
