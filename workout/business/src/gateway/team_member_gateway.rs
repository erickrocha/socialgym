use crate::commons::entity_mapper::EntityMapper;
use crate::domain::team_member::{TeamMember, TeamMemberMapper, TeamMemberStatus};
use entity::prelude::TeamMember as TeamMemberQuery;
use entity::team_member;
use entity::team_member::Column;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

pub struct TeamMemberGateway {}

impl TeamMemberGateway {
    pub async fn persist(
        db: &DbConn,
        team_member: TeamMember,
    ) -> Result<team_member::ActiveModel, DbErr> {
        let active_model = TeamMemberMapper::build_active_model(team_member);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, team_member: TeamMember) -> Result<team_member::Model, DbErr> {
        let active_model = TeamMemberMapper::build_active_model(team_member);
        active_model.update(db).await
    }

    /// The (business_profile, person) pair is the membership key — a pair is only ever
    /// represented by a single row, whatever its current status.
    pub async fn find_membership(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<Option<team_member::Model>, DbErr> {
        TeamMemberQuery::find()
            .filter(Column::BusinessProfileId.eq(business_profile_id))
            .filter(Column::PersonId.eq(person_id))
            .one(db)
            .await
    }

    pub async fn find_all_by_business_profile_and_status(
        db: &DbConn,
        business_profile_id: i32,
        status: TeamMemberStatus,
    ) -> Result<Vec<team_member::Model>, DbErr> {
        TeamMemberQuery::find()
            .filter(Column::BusinessProfileId.eq(business_profile_id))
            .filter(Column::Status.eq(status.as_str()))
            .all(db)
            .await
    }

    pub async fn find_all_by_person_and_status(
        db: &DbConn,
        person_id: i32,
        status: TeamMemberStatus,
    ) -> Result<Vec<team_member::Model>, DbErr> {
        TeamMemberQuery::find()
            .filter(Column::PersonId.eq(person_id))
            .filter(Column::Status.eq(status.as_str()))
            .all(db)
            .await
    }
}
