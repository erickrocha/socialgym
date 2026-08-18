use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use crate::domain::business_profile::BusinessProfile;
use crate::domain::person_address::PersonAddress;
use crate::domain::person_info::PersonInfo;
use crate::domain::user::User;
use chrono::{NaiveDate, NaiveDateTime};
use entity::person::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct Person {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub firstname: String,
    pub surname: String,
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub user: Option<User>,
    pub person_info: Option<PersonInfo>,
    pub addresses: Vec<PersonAddress>,
    pub avatar: Option<String>,
    pub object_key: Option<String>,
    pub cover: Option<String>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub business_profiles: Vec<BusinessProfile>,
}

pub struct PersonEntityMapper {}

impl EntityMapper<Person, Model, ActiveModel> for PersonEntityMapper {
    fn build_active_model(d: Person) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(&uuid)),
                None => NotSet,
            },
            first_name: Set(d.firstname.to_owned()),
            surname: Set(d.surname.to_owned()),
            date_of_birth: Set(d.date_of_birth.to_owned()),
            gender: Set(d.gender.to_owned()),
            created_at: NotSet,
            updated_at: NotSet,
            avatar: Set(d.avatar.to_owned()),
            cover_image: Set(d.cover.to_owned()),
        }
    }

    fn from_model(e: Model) -> Person {
        Person {
            id: Some(e.id),
            firstname: e.first_name,
            surname: e.surname,
            date_of_birth: e.date_of_birth,
            gender: e.gender,
            user: None,
            person_info: None,
            addresses: Vec::new(),
            business_profiles: Vec::new(),
            uuid: Some(uuid_to_string(e.uuid)),
            avatar: None,
            object_key: e.avatar,
            cover: e.cover_image,
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> Person {
        Person {
            id: Some(e.id.unwrap()),
            firstname: e.first_name.unwrap(),
            surname: e.surname.unwrap(),
            date_of_birth: e.date_of_birth.unwrap(),
            gender: e.gender.unwrap(),
            user: None,
            person_info: None,
            addresses: Vec::new(),
            business_profiles: Vec::new(),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            avatar: None,
            object_key: e.avatar.unwrap(),
            cover: e.cover_image.unwrap(),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl Person {
    pub fn new(
        firstname: String,
        surname: String,
        date_of_birth: NaiveDate,
        gender: String,
    ) -> Person {
        Self::update(
            None,
            firstname,
            surname,
            date_of_birth,
            gender,
            None,
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        firstname: String,
        surname: String,
        date_of_birth: NaiveDate,
        gender: String,
        object_key: Option<String>,
        cover: Option<String>,
        user: Option<User>,
        person_info: Option<PersonInfo>,
        addresses: Vec<PersonAddress>,
        business_profiles: Vec<BusinessProfile>,
    ) -> Person {
        Person {
            id,
            uuid: None,
            firstname,
            surname,
            date_of_birth,
            gender,
            user,
            person_info,
            addresses,
            business_profiles,
            avatar: None,
            object_key,
            cover,
            created_at: None,
            updated_at: None,
        }
    }
}
