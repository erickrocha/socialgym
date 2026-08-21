use crate::m20260129_000007_create_table_workout::Workout;
use crate::m20260129_000008_create_table_exercise::Exercise;
use crate::m20260129_000011_create_table_friends::Friends;
use crate::m20260811_000001_create_table_team_members::TeamMembers;
use sea_orm_migration::prelude::*;

/// FK/UUID columns filtered on nearly every list/read endpoint (`find_by_owner_id`,
/// friend lookups, team membership checks) but so far relying only on implicit
/// PK indexes — each of these was a sequential scan.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_index(
                Index::create()
                    .name("idx_workout_owner_id")
                    .table(Workout::Table)
                    .col(Workout::OwnerId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_workout_owner_uuid")
                    .table(Workout::Table)
                    .col(Workout::OwnerUuid)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_exercise_owner_id")
                    .table(Exercise::Table)
                    .col(Exercise::OwnerId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_friends_person_id")
                    .table(Friends::Table)
                    .col(Friends::PersonId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_friends_friend_id")
                    .table(Friends::Table)
                    .col(Friends::FriendId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_team_members_business_profile_id")
                    .table(TeamMembers::Table)
                    .col(TeamMembers::BusinessProfileId)
                    .to_owned(),
            )
            .await?;
        manager
            .create_index(
                Index::create()
                    .name("idx_team_members_person_id")
                    .table(TeamMembers::Table)
                    .col(TeamMembers::PersonId)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        for name in [
            "idx_workout_owner_id",
            "idx_workout_owner_uuid",
            "idx_exercise_owner_id",
            "idx_friends_person_id",
            "idx_friends_friend_id",
            "idx_team_members_business_profile_id",
            "idx_team_members_person_id",
        ] {
            manager
                .drop_index(Index::drop().name(name).to_owned())
                .await?;
        }
        Ok(())
    }
}
