use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::{string_to_uuid, uuid_to_string};
use chrono::NaiveDateTime;
use entity::team_member::{ActiveModel, Model};
use sea_orm::{NotSet, Set};

#[derive(Debug, Clone)]
pub struct TeamMember {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub business_profile_id: i32,
    pub business_profile_uuid: String,
    pub person_id: i32,
    pub person_uuid: String,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    pub status: TeamMemberStatus,
}

impl TeamMember {
    pub fn new(
        business_profile_id: i32,
        business_profile_uuid: String,
        person_id: i32,
        person_uuid: String,
        status: TeamMemberStatus,
    ) -> TeamMember {
        TeamMember {
            id: None,
            uuid: None,
            business_profile_id,
            business_profile_uuid,
            person_id,
            person_uuid,
            created_at: None,
            updated_at: None,
            status,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TeamMemberStatus {
    Pending,
    Accepted,
    Rejected,
    Cancelled,
}

impl TeamMemberStatus {
    pub fn as_str(&self) -> String {
        match self {
            TeamMemberStatus::Pending => "Pending".to_string(),
            TeamMemberStatus::Accepted => "Accepted".to_string(),
            TeamMemberStatus::Rejected => "Rejected".to_string(),
            TeamMemberStatus::Cancelled => "Cancelled".to_string(),
        }
    }

    pub fn from_string(status: &str) -> TeamMemberStatus {
        match status {
            "Pending" => TeamMemberStatus::Pending,
            "Accepted" => TeamMemberStatus::Accepted,
            "Rejected" => TeamMemberStatus::Rejected,
            "Cancelled" => TeamMemberStatus::Cancelled,
            _ => TeamMemberStatus::Pending,
        }
    }
}

pub struct TeamMemberMapper {}

impl EntityMapper<TeamMember, Model, ActiveModel> for TeamMemberMapper {
    fn build_active_model(d: TeamMember) -> ActiveModel {
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
            business_profile_uuid: Set(string_to_uuid(d.business_profile_uuid.as_str())),
            status: Set(d.status.as_str().to_string()),
            person_uuid: Set(string_to_uuid(d.person_uuid.as_str())),
            person_id: Set(d.person_id),
            created_at: NotSet,
            updated_at: NotSet,
        }
    }

    fn from_model(e: Model) -> TeamMember {
        TeamMember {
            id: Some(e.id),
            uuid: Some(uuid_to_string(e.uuid)),
            business_profile_id: e.business_profile_id,
            business_profile_uuid: uuid_to_string(e.business_profile_uuid),
            person_id: e.person_id,
            person_uuid: uuid_to_string(e.person_uuid),
            status: TeamMemberStatus::from_string(e.status.as_str()),
            created_at: Some(e.created_at.naive_utc()),
            updated_at: Some(e.updated_at.naive_utc()),
        }
    }

    fn from_active_model(e: ActiveModel) -> TeamMember {
        TeamMember {
            id: Some(e.id.unwrap()),
            uuid: Some(uuid_to_string(e.uuid.unwrap())),
            business_profile_id: e.business_profile_id.unwrap(),
            business_profile_uuid: uuid_to_string(e.business_profile_uuid.unwrap()),
            person_id: e.person_id.unwrap(),
            person_uuid: uuid_to_string(e.person_uuid.unwrap()),
            status: TeamMemberStatus::from_string(e.status.unwrap().as_str()),
            created_at: Some(e.created_at.unwrap().naive_utc()),
            updated_at: Some(e.updated_at.unwrap().naive_utc()),
        }
    }
}
