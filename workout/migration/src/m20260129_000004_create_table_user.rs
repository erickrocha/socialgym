use crate::m20260129_000003_create_table_person::Person;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(User::Table)
                    .if_not_exists()
                    .col(pk_auto(User::Id).integer())
                    .col(uuid_uniq(User::Uuid))
                    .col(string_len(User::Name, 500).null())
                    .col(string_len(User::Email, 500).unique_key())
                    .col(string_len(User::Password, 500).not_null())
                    .col(boolean(User::FirstLogin).default(true))
                    .col(boolean(User::Enabled).default(true))
                    .col(integer(User::PersonId).not_null())
                    .col(uuid_uniq(User::PersonUuid))
                    .col(timestamp_with_time_zone(User::CreatedAt).null())
                    .col(timestamp_with_time_zone(User::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_user_person")
                    .from_tbl(User::Table)
                    .from_col(User::PersonId)
                    .to_tbl(Person::Table)
                    .to_col(Person::Id)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(User::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum User {
    Table,
    Id,
    Name,
    Email,
    Password,
    FirstLogin,
    Enabled,
    PersonId,
    PersonUuid,
    Uuid,
    CreatedAt,
    UpdatedAt,
}
