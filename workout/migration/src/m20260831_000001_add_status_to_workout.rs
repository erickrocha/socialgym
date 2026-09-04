use crate::{DbErr, DeriveMigrationName, async_trait};
use sea_orm_migration::prelude::*;

/// Adds the consent lifecycle to workouts. Existing rows are all self-created,
/// so they back-fill to `Accepted`. `assigned_by_profile_id` records which
/// business profile assigned a `Pending` workout, so only that profile can
/// cancel the assignment.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Workout::Table)
                    .add_column(
                        ColumnDef::new(Workout::Status)
                            .string_len(20)
                            .not_null()
                            .default("Accepted"),
                    )
                    .add_column(
                        ColumnDef::new(Workout::AssignedByProfileId)
                            .integer()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(Workout::AssignedByProfileUuid)
                            .uuid()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_workout_assigned_by_profile_id")
                    .table(Workout::Table)
                    .col(Workout::AssignedByProfileId)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_index(
                Index::drop()
                    .name("idx_workout_assigned_by_profile_id")
                    .to_owned(),
            )
            .await?;

        manager
            .alter_table(
                Table::alter()
                    .table(Workout::Table)
                    .drop_column(Workout::Status)
                    .drop_column(Workout::AssignedByProfileId)
                    .drop_column(Workout::AssignedByProfileUuid)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Workout {
    Table,
    Status,
    AssignedByProfileId,
    AssignedByProfileUuid,
}
