use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use crate::domain::enums::InviteStatus;
use chrono::NaiveDateTime;
use entity::friends_entity::{ActiveModel, FriendsEntity};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct Friend {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub person_id: i32,
    pub person_uuid: String,
    pub friend_id: i32,
    pub friend_uuid: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: InviteStatus,
}

pub struct FriendEntityMapper {}

impl EntityMapper<Friend, FriendsEntity, ActiveModel> for FriendEntityMapper {
    fn build_active_model(d: Friend) -> ActiveModel {
        ActiveModel {
            id: match d.id {
                Some(id) => Set(id),
                None => NotSet,
            },
            uuid: match d.uuid {
                Some(uuid) => Set(string_to_uuid(uuid.as_str())),
                None => NotSet,
            },
            person_id: Set(d.person_id),
            friend_id: Set(d.friend_id),
            status: Set(d.status.as_str().to_string()),
            person_uuid: Set(string_to_uuid(d.person_uuid.as_str())),
            friend_uuid: Set(string_to_uuid(d.friend_uuid.as_str())),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: FriendsEntity) -> Friend {
        Friend {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            person_id: e.person_id,
            person_uuid: uuid_to_string(e.person_uuid),
            friend_id: e.friend_id,
            friend_uuid: uuid_to_string(e.friend_uuid),
            status: InviteStatus::from_string(e.status.as_str()),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> Friend {
        Friend {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            person_id: e.person_id.unwrap(),
            person_uuid: uuid_to_string(e.person_uuid.unwrap()),
            friend_id: e.friend_id.unwrap(),
            friend_uuid: uuid_to_string(e.friend_uuid.unwrap()),
            status: InviteStatus::from_string(e.status.unwrap().as_str()),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}

impl Friend {
    pub fn new(
        person_id: i32,
        friend_id: i32,
        person_uuid: String,
        friend_uuid: String,
        status: InviteStatus,
    ) -> Friend {
        Friend {
            id: None,
            person_id,
            friend_id,
            created_at: None,
            updated_at: None,
            status,
            uuid: None,
            person_uuid,
            friend_uuid,
        }
    }

    pub fn update(
        id: Option<i32>,
        person_id: i32,
        friend_id: i32,
        person_uuid: String,
        friend_uuid: String,
        status: InviteStatus,
    ) -> Friend {
        Friend {
            id,
            person_id,
            friend_id,
            created_at: None,
            updated_at: None,
            status,
            uuid: None,
            person_uuid,
            friend_uuid,
        }
    }
}
