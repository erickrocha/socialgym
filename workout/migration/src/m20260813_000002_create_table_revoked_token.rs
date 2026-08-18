use crate::m20260129_000004_create_table_user::User;
use crate::{DbErr, DeriveMigrationName, async_trait};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(RevokedToken::Table)
                    .if_not_exists()
                    .col(pk_auto(RevokedToken::Id))
                    .col(uuid_uniq(RevokedToken::Uuid))
                    .col(string_len_uniq(RevokedToken::Jti, 36).not_null())
                    .col(integer(RevokedToken::UserId).not_null())
                    .col(string_len(RevokedToken::TokenType, 20).not_null())
                    .col(timestamp_with_time_zone(RevokedToken::ExpiresAt).not_null())
                    .col(timestamp_with_time_zone(RevokedToken::CreatedAt).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_revoked_token_user")
                    .from(RevokedToken::Table, RevokedToken::UserId)
                    .to(User::Table, User::Id)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_revoked_token_user")
                    .table(RevokedToken::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(RevokedToken::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum RevokedToken {
    Table,
    Id,
    Uuid,
    Jti,
    UserId,
    TokenType,
    ExpiresAt,
    CreatedAt,
}
