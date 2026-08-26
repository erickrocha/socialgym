use crate::commons::entity_mapper::EntityMapper;
use crate::domain::team_member::{TeamMember, TeamMemberMapper, TeamMemberStatus};
use entity::prelude::TeamMemberEntity as TeamMemberQuery;
use entity::team_member_entity as team_member;
use entity::team_member_entity::Column;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr, DeleteResult, EntityTrait,
    QueryFilter,
};

pub struct TeamMemberGateway {}

impl TeamMemberGateway {
    pub async fn persist(
        db: &DbConn,
        team_member: TeamMember,
    ) -> Result<team_member::ActiveModel, DbErr> {
        let active_model = TeamMemberMapper::build_active_model(team_member);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, team_member: TeamMember) -> Result<team_member::TeamMemberEntity, DbErr> {
        let active_model = TeamMemberMapper::build_active_model(team_member);
        active_model.update(db).await
    }

    /// The (business_profile, person) pair is the membership key — a pair is only ever
    /// represented by a single row, whatever its current status.
    pub async fn find_membership(
        db: &DbConn,
        business_profile_id: i32,
        person_id: i32,
    ) -> Result<Option<team_member::TeamMemberEntity>, DbErr> {
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
    ) -> Result<Vec<team_member::TeamMemberEntity>, DbErr> {
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
    ) -> Result<Vec<team_member::TeamMemberEntity>, DbErr> {
        TeamMemberQuery::find()
            .filter(Column::PersonId.eq(person_id))
            .filter(Column::Status.eq(status.as_str()))
            .all(db)
            .await
    }

    /// Bulk-deletes every membership row for a business profile (account-purge
    /// cascade, when the business profile itself is being deleted).
    pub async fn delete_all_by_business_profile_id<C: ConnectionTrait>(
        db: &C,
        business_profile_id: i32,
    ) -> Result<DeleteResult, DbErr> {
        TeamMemberQuery::delete_many()
            .filter(Column::BusinessProfileId.eq(business_profile_id))
            .exec(db)
            .await
    }

    /// Bulk-deletes every membership row for a person — covers memberships at
    /// *other* people's businesses, not just their own (account-purge cascade).
    pub async fn delete_all_by_person_id<C: ConnectionTrait>(db: &C, person_id: i32) -> Result<DeleteResult, DbErr> {
        TeamMemberQuery::delete_many()
            .filter(Column::PersonId.eq(person_id))
            .exec(db)
            .await
    }
}
