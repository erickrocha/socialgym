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
                    .table(Workout::Table)
                    .if_not_exists()
                    .col(pk_auto(Workout::Id))
                    .col(uuid_uniq(Workout::Uuid))
                    .col(integer(Workout::OwnerId).not_null())
                    .col(uuid(Workout::OwnerUuid))
                    .col(string_len(Workout::Name, 255).not_null())
                    .col(text(Workout::Description).null())
                    .col(string_len(Workout::Difficulty, 50).not_null())
                    .col(string_len(Workout::MuscleGroup, 255).not_null())
                    .col(string_len(Workout::Visibility, 50).not_null())
                    .col(timestamp_with_time_zone(Workout::CreatedAt).null())
                    .col(timestamp_with_time_zone(Workout::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_workout_person")
                    .from_tbl(Workout::Table)
                    .from_col(Workout::OwnerId)
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
                    .name("fk_workout_person")
                    .table(Workout::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Workout::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Workout {
    Table,
    Id,
    Uuid,
    OwnerId,
    OwnerUuid,
    Name,
    Description,
    Difficulty,
    MuscleGroup,
    Visibility,
    CreatedAt,
    UpdatedAt,
}
