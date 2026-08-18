use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Person::Table)
                    .if_not_exists()
                    .col(pk_auto(Person::Id))
                    .col(uuid_uniq(Person::Uuid))
                    .col(string_len(Person::FirstName, 500).not_null())
                    .col(string_len(Person::Surname, 500).not_null())
                    .col(date(Person::DateOfBirth).not_null())
                    .col(string_len(Person::Gender, 50).not_null())
                    .col(string_len(Person::Avatar, 500).null())
                    .col(string_len(Person::CoverImage, 500).null())
                    .col(timestamp_with_time_zone(Person::CreatedAt).null())
                    .col(timestamp_with_time_zone(Person::UpdatedAt).null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Person::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Person {
    Table,
    Id,
    Uuid,
    FirstName,
    Surname,
    DateOfBirth,
    Gender,
    Avatar,
    CoverImage,
    CreatedAt,
    UpdatedAt,
}
