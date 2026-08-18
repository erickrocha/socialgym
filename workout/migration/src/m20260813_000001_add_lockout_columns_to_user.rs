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
                        ColumnDef::new(User::FailedLoginAttempts)
                            .integer()
                            .not_null()
                            .default(0),
                    )
                    .add_column(
                        ColumnDef::new(User::LockedUntil)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .add_column(
                        ColumnDef::new(User::TokenValidAfter)
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
                    .drop_column(User::FailedLoginAttempts)
                    .drop_column(User::LockedUntil)
                    .drop_column(User::TokenValidAfter)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    FailedLoginAttempts,
    LockedUntil,
    TokenValidAfter,
}
