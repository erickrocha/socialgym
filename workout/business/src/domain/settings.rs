use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use crate::domain::enums::Position;
use chrono::NaiveDateTime;
use entity::settings_entity::{ActiveModel, SettingsEntity};
use sea_orm::Set;

#[derive(Debug, Clone)]
pub struct Settings {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub person_id: i32,
    pub person_uuid: String,
    pub language: String,
    pub theme: String,
    pub notifications_enabled: bool,
    pub context_menu_position: Position,
    pub home_page: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
}

pub struct SettingsEntityMapper {}

impl EntityMapper<Settings, SettingsEntity, ActiveModel> for SettingsEntityMapper {
    fn build_active_model(d: Settings) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => sea_orm::NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(uuid.as_str())),
                None => sea_orm::NotSet,
            },
            person_id: Set(d.person_id),
            person_uuid: Set(string_to_uuid(d.person_uuid.as_str())),
            language: Set(d.language),
            theme: Set(d.theme),
            notifications_enabled: Set(d.notifications_enabled),
            context_menu_position: Set(d.context_menu_position.to_text()),
            home_page: Set(d.home_page),
            created_at: sea_orm::NotSet,
            updated_at: sea_orm::NotSet,
        }
    }

    fn from_model(e: SettingsEntity) -> Settings {
        Settings {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            person_id: e.person_id,
            person_uuid: uuid_to_string(e.person_uuid),
            language: e.language,
            theme: e.theme,
            notifications_enabled: e.notifications_enabled,
            context_menu_position: Position::from_string(&e.context_menu_position),
            home_page: e.home_page,
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> Settings {
        Settings {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            person_id: e.person_id.unwrap(),
            person_uuid: uuid_to_string(e.person_uuid.unwrap()),
            language: e.language.unwrap(),
            theme: e.theme.unwrap(),
            notifications_enabled: e.notifications_enabled.unwrap(),
            context_menu_position: Position::from_string(&e.context_menu_position.unwrap()),
            home_page: e.home_page.unwrap(),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl Settings {
    pub fn new(
        person_id: i32,
        person_uuid: String,
        language: String,
        theme: String,
        notifications_enable: bool,
        context_menu_position: Position,
        home_page: String,
    ) -> Settings {
        Self::update(
            None,
            None,
            person_id,
            person_uuid,
            language,
            theme,
            notifications_enable,
            context_menu_position,
            home_page,
        )
    }

    #[allow(clippy::too_many_arguments)]
    pub fn update(
        id: Option<i32>,
        uuid: Option<String>,
        person_id: i32,
        person_uuid: String,
        language: String,
        theme: String,
        notifications_enable: bool,
        context_menu_position: Position,
        home_page: String,
    ) -> Self {
        Settings {
            id,
            uuid,
            person_id,
            person_uuid,
            language,
            theme,
            notifications_enabled: notifications_enable,
            context_menu_position,
            home_page,
            created_at: None,
            updated_at: None,
        }
    }
}
