use crate::m20260129_000003_create_table_person::Person;
use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Create the junction table workout_exercise
        manager
            .create_table(
                Table::create()
                    .table(PersonMedia::Table)
                    .if_not_exists()
                    .col(pk_auto(PersonMedia::Id))
                    .col(uuid_uniq(PersonMedia::Uuid))
                    .col(integer(PersonMedia::PersonId).not_null())
                    .col(uuid(PersonMedia::PersonUuid).not_null())
                    .col(string_len(PersonMedia::MimeType, 20).not_null())
                    .col(string_len(PersonMedia::Album, 200).not_null())
                    .col(string_len(PersonMedia::S3Key, 2000).not_null())
                    .col(timestamp_with_time_zone(PersonMedia::CreatedAt).null())
                    .col(timestamp_with_time_zone(PersonMedia::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        // Create foreign key to person
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_person_media_person")
                    .from_tbl(PersonMedia::Table)
                    .from_col(PersonMedia::PersonId)
                    .to_tbl(Person::Table)
                    .to_col(Person::Id)
                    .on_delete(ForeignKeyAction::Cascade)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_person_media_person")
                    .table(PersonMedia::Table)
                    .to_owned(),
            )
            .await?;

        // Drop the junction table
        manager
            .drop_table(Table::drop().table(PersonMedia::Table).to_owned())
            .await?;

        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum PersonMedia {
    Table,
    Id,
    Uuid,
    PersonId,
    PersonUuid,
    MimeType,
    Album,
    S3Key,
    CreatedAt,
    UpdatedAt,
}
