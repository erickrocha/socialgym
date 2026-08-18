use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::business_profile_address::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct BusinessProfileAddress {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub business_profile_id: i32,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: Option<String>,
    pub country_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct BusinessProfileAddressEntityMapper {}

impl EntityMapper<BusinessProfileAddress, Model, ActiveModel>
    for BusinessProfileAddressEntityMapper
{
    fn build_active_model(d: BusinessProfileAddress) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(uuid.as_str())),
                None => NotSet,
            },
            business_profile_id: Set(d.business_profile_id),
            address_line1: Set(d.address_line1),
            address_line2: Set(d.address_line2),
            locality: Set(d.locality),
            administrative_area: Set(d.administrative_area),
            postal_code: Set(d.postal_code),
            country_code: Set(d.country_code),
            latitude: Set(d.latitude),
            longitude: Set(d.longitude),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: Model) -> BusinessProfileAddress {
        BusinessProfileAddress {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            business_profile_id: e.business_profile_id,
            address_line1: e.address_line1,
            address_line2: e.address_line2,
            locality: e.locality,
            administrative_area: e.administrative_area,
            postal_code: e.postal_code,
            country_code: e.country_code,
            latitude: e.latitude,
            longitude: e.longitude,
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> BusinessProfileAddress {
        BusinessProfileAddress {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            business_profile_id: e.business_profile_id.unwrap(),
            address_line1: e.address_line1.unwrap(),
            address_line2: e.address_line2.unwrap(),
            locality: e.locality.unwrap(),
            administrative_area: e.administrative_area.unwrap(),
            postal_code: e.postal_code.unwrap(),
            country_code: e.country_code.unwrap(),
            latitude: e.latitude.unwrap(),
            longitude: e.longitude.unwrap(),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl BusinessProfileAddress {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        business_profile_id: i32,
        address_line1: String,
        address_line2: Option<String>,
        locality: String,
        administrative_area: String,
        postal_code: Option<String>,
        country_code: String,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Self {
        Self::update(
            None,
            business_profile_id,
            address_line1,
            address_line2,
            locality,
            administrative_area,
            postal_code,
            country_code,
            latitude,
            longitude,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        business_profile_id: i32,
        address_line1: String,
        address_line2: Option<String>,
        locality: String,
        administrative_area: String,
        postal_code: Option<String>,
        country_code: String,
        latitude: Option<f64>,
        longitude: Option<f64>,
    ) -> Self {
        Self {
            id,
            uuid: None,
            business_profile_id,
            address_line1,
            address_line2,
            locality,
            administrative_area,
            postal_code,
            country_code,
            latitude,
            longitude,
            created_at: None,
            updated_at: None,
        }
    }
}
