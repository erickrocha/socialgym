use crate::m20260129_000003_create_table_person::Person;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer, pk_auto, string_len_null, uuid_uniq};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Friends::Table)
                    .if_not_exists()
                    .col(pk_auto(Friends::Id))
                    .col(uuid_uniq(Friends::Uuid))
                    .col(integer(Friends::PersonId).not_null())
                    .col(uuid_uniq(Friends::PersonUuid))
                    .col(integer(Friends::FriendId).not_null())
                    .col(uuid_uniq(Friends::FriendUuid))
                    .col(
                        ColumnDef::new(Friends::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Friends::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(string_len_null(Friends::Status, 20))
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_friends_person_person")
                    .from_tbl(Friends::Table)
                    .from_col(Friends::PersonId)
                    .to_tbl(Person::Table)
                    .to_col(Person::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_friends_person_friend")
                    .from_tbl(Friends::Table)
                    .from_col(Friends::FriendId)
                    .to_tbl(Person::Table)
                    .to_col(Person::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Friends::Table)
                    .name("fk_friends_person_person")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(Friends::Table)
                    .name("fk_friends_person_friend")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Friends::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Friends {
    Table,
    Id,
    Uuid,
    PersonId,
    PersonUuid,
    FriendId,
    FriendUuid,
    CreatedAt,
    UpdatedAt,
    Status,
}
