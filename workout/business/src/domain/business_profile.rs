use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use crate::domain::business_profile_address::BusinessProfileAddress;
use crate::domain::enums::ProfileType;
use chrono::NaiveDateTime;
use entity::business_profile::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct BusinessProfile {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub owner_id: i32,
    pub owner_uuid: String,
    pub tax_id: String,
    pub business_name: String,
    pub business_type: ProfileType,
    pub social_name: Option<String>,
    pub logo: Option<String>,
    pub object_key: Option<String>,
    pub cover_image: Option<String>,
    pub addresses: Vec<BusinessProfileAddress>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct BusinessProfileEntityMapper {}

impl EntityMapper<BusinessProfile, Model, ActiveModel> for BusinessProfileEntityMapper {
    fn build_active_model(d: BusinessProfile) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(uuid.as_str())),
                None => NotSet,
            },
            owner_id: Set(d.owner_id),
            owner_uuid: Set(string_to_uuid(d.owner_uuid.as_str())),
            tax_id: Set(d.tax_id),
            business_name: Set(d.business_name),
            business_type: Set(d.business_type.to_text()),
            social_name: Set(d.social_name),
            logo: Set(d.logo),
            cover_image: Set(d.cover_image),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: Model) -> BusinessProfile {
        BusinessProfile {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            owner_id: e.owner_id,
            owner_uuid: uuid_to_string(e.owner_uuid),
            tax_id: e.tax_id,
            business_name: e.business_name,
            business_type: ProfileType::from_string(&e.business_type),
            social_name: e.social_name,
            logo: None,
            object_key: e.logo,
            cover_image: e.cover_image,
            addresses: Vec::new(),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> BusinessProfile {
        BusinessProfile {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            owner_id: e.owner_id.unwrap(),
            owner_uuid: uuid_to_string(e.owner_uuid.unwrap()),
            tax_id: e.tax_id.unwrap(),
            business_name: e.business_name.unwrap(),
            business_type: ProfileType::from_string(&e.business_type.unwrap()),
            social_name: e.social_name.unwrap(),
            logo: None,
            object_key: e.logo.unwrap(),
            cover_image: e.cover_image.unwrap(),
            addresses: Vec::new(),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl BusinessProfile {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        owner_id: i32,
        owner_uuid: String,
        tax_id: String,
        business_name: String,
        business_type: ProfileType,
        social_name: Option<String>,
    ) -> Self {
        Self::update(
            None,
            None,
            owner_id,
            owner_uuid,
            tax_id,
            business_name,
            business_type,
            social_name,
            None,
            None,
            Vec::new(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        uuid: Option<String>,
        owner_id: i32,
        owner_uuid: String,
        tax_id: String,
        business_name: String,
        business_type: ProfileType,
        social_name: Option<String>,
        logo: Option<String>,
        cover_image: Option<String>,
        addresses: Vec<BusinessProfileAddress>,
    ) -> Self {
        Self {
            id,
            uuid,
            owner_id,
            owner_uuid,
            tax_id,
            business_name,
            business_type,
            social_name,
            logo: None,
            object_key: logo,
            cover_image,
            addresses,
            created_at: None,
            updated_at: None,
        }
    }
}
