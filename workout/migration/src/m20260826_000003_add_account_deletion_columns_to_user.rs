use crate::{DbErr, DeriveMigrationName, async_trait};
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .add_column(
                        ColumnDef::new(User::DeletionRequestedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(User::DeletionScheduledAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(User::Table)
                    .drop_column(User::DeletionRequestedAt)
                    .drop_column(User::DeletionScheduledAt)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    DeletionRequestedAt,
    DeletionScheduledAt,
}
