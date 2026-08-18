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
                    .table(PersonInfo::Table)
                    .if_not_exists()
                    .col(pk_auto(PersonInfo::Id).integer())
                    .col(uuid_uniq(PersonInfo::Uuid))
                    .col(integer(PersonInfo::PersonId).not_null())
                    .col(string_len_null(PersonInfo::Biography, 1000))
                    .col(string_len_null(PersonInfo::Relationship, 200))
                    .col(string_len_null(PersonInfo::Job, 500))
                    .col(string_len_null(PersonInfo::HomeTown, 500))
                    .col(string_len_null(PersonInfo::CurrentCity, 500))
                    .col(float_null(PersonInfo::Weight))
                    .col(float_null(PersonInfo::Height))
                    .col(timestamp_with_time_zone(PersonInfo::CreatedAt).null())
                    .col(timestamp_with_time_zone(PersonInfo::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_person_info_person")
                    .from_tbl(PersonInfo::Table)
                    .from_col(PersonInfo::PersonId)
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
                    .table(PersonInfo::Table)
                    .name("fk_person_info_person")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(PersonInfo::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PersonInfo {
    Table,
    Id,
    PersonId,
    Biography,
    Relationship,
    Job,
    HomeTown,
    CurrentCity,
    Weight,
    Height,
    Uuid,
    CreatedAt,
    UpdatedAt,
}
