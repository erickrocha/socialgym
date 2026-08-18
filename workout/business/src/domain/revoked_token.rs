use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::revoked_token::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct RevokedToken {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub jti: String,
    pub user_id: i32,
    pub token_type: String,
    pub expires_at: NaiveDateTime,
    pub created_at: Option<NaiveDateTime>,
}

impl RevokedToken {
    pub fn new(
        jti: String,
        user_id: i32,
        token_type: String,
        expires_at: NaiveDateTime,
    ) -> RevokedToken {
        RevokedToken {
            id: None,
            uuid: None,
            jti,
            user_id,
            token_type,
            expires_at,
            created_at: None,
        }
    }
}

pub struct RevokedTokenMapper {}

impl EntityMapper<RevokedToken, Model, ActiveModel> for RevokedTokenMapper {
    fn build_active_model(d: RevokedToken) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(&uuid)),
                None => NotSet,
            },
            jti: Set(d.jti),
            user_id: Set(d.user_id),
            token_type: Set(d.token_type),
            expires_at: Set(d.expires_at.and_utc()),
            created_at: NotSet,
        }
    }

    fn from_model(e: Model) -> RevokedToken {
        RevokedToken {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            jti: e.jti,
            user_id: e.user_id,
            token_type: e.token_type,
            expires_at: e.expires_at.naive_utc(),
            created_at: Some(e.created_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> RevokedToken {
        RevokedToken {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            jti: e.jti.unwrap(),
            user_id: e.user_id.unwrap(),
            token_type: e.token_type.unwrap(),
            expires_at: e.expires_at.unwrap().naive_utc(),
            created_at: Some(e.created_at.unwrap().naive_utc()),
        }
    }
}
