use crate::m20260129_000004_create_table_user::User;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;
#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(UserRole::Table)
                    .if_not_exists()
                    .col(pk_auto(UserRole::Id))
                    .col(integer(UserRole::UserId).not_null())
                    .col(string_len(UserRole::Role, 64).not_null())
                    .col(timestamp_with_time_zone(UserRole::CreatedAt).not_null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_user_role_user")
                            .from(UserRole::Table, UserRole::UserId)
                            .to(User::Table, User::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .index(
                        Index::create()
                            .name("uq_user_role")
                            .col(UserRole::UserId)
                            .col(UserRole::Role)
                            .unique(),
                    )
                    .to_owned(),
            )
            .await
    }
    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(UserRole::Table).to_owned())
            .await
    }
}
#[derive(DeriveIden)]
enum UserRole {
    Table,
    Id,
    UserId,
    Role,
    CreatedAt,
}
