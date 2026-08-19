use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::user_entity::{ActiveModel, UserEntity};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct User {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub email: String,
    pub name: Option<String>,
    pub password: String,
    pub enabled: bool,
    pub first_login: bool,
    pub person_id: i32,
    pub person_uuid: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub failed_login_attempts: i32,
    pub locked_until: Option<NaiveDateTime>,
    pub token_valid_after: Option<NaiveDateTime>,
}

pub struct UserEntityMapper {}
impl EntityMapper<User, UserEntity, ActiveModel> for UserEntityMapper {
    fn build_active_model(d: User) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(&uuid)),
                None => NotSet,
            },
            name: Set(d.name.to_owned()),
            email: Set(d.email.to_owned()),
            password: Set(d.password.to_owned()),
            first_login: Set(d.first_login.to_owned()),
            enabled: Set(d.enabled.to_owned()),
            person_id: Set(d.person_id),
            person_uuid: Set(string_to_uuid(&d.person_uuid)),
            created_at: NotSet,
            updated_at: NotSet,
            failed_login_attempts: Set(d.failed_login_attempts),
            locked_until: Set(d.locked_until.map(|dt| dt.and_utc())),
            token_valid_after: Set(d.token_valid_after.map(|dt| dt.and_utc())),
        }
    }

    fn from_model(e: UserEntity) -> User {
        User {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            name: e.name,
            email: e.email,
            password: e.password,
            enabled: e.enabled,
            first_login: e.first_login,
            person_id: e.person_id,
            person_uuid: uuid_to_string(e.person_uuid),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
            failed_login_attempts: e.failed_login_attempts,
            locked_until: e.locked_until.map(|dt| dt.naive_utc()),
            token_valid_after: e.token_valid_after.map(|dt| dt.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> User {
        User {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            name: e.name.unwrap(),
            email: e.email.unwrap(),
            password: e.password.unwrap(),
            enabled: e.enabled.unwrap(),
            first_login: e.first_login.unwrap(),
            person_id: e.person_id.unwrap(),
            person_uuid: uuid_to_string(e.person_uuid.unwrap()),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
            failed_login_attempts: e.failed_login_attempts.unwrap(),
            locked_until: e.locked_until.unwrap().map(|dt| dt.naive_utc()),
            token_valid_after: e.token_valid_after.unwrap().map(|dt| dt.naive_utc()),
        }
    }
}

impl User {
    pub fn new(
        name: Option<String>,
        email: String,
        password: String,
        person_id: i32,
        person_uuid: String,
    ) -> User {
        Self::update(
            None,
            None,
            name,
            email,
            password,
            true,
            true,
            person_id,
            person_uuid,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        uuid: Option<String>,
        name: Option<String>,
        email: String,
        password: String,
        enabled: bool,
        first_login: bool,
        person_id: i32,
        person_uuid: String,
    ) -> User {
        User {
            id,
            uuid,
            name,
            email,
            password,
            enabled,
            first_login,
            person_id,
            person_uuid,
            created_at: None,
            updated_at: None,
            failed_login_attempts: 0,
            locked_until: None,
            token_valid_after: None,
        }
    }
}
