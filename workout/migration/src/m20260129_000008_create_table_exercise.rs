use crate::m20260129_000003_create_table_person::Person;
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
                    .table(Exercise::Table)
                    .if_not_exists()
                    .col(pk_auto(Exercise::Id))
                    .col(uuid_uniq(Exercise::Uuid))
                    .col(string_len(Exercise::Name, 255).not_null())
                    .col(string_len(Exercise::Category, 255).not_null())
                    .col(integer(Exercise::OwnerId).not_null())
                    .col(uuid(Exercise::OwnerUuid))
                    .col(string_len(Exercise::OwnerName, 255).not_null())
                    .col(integer(Exercise::Sets).not_null())
                    .col(integer(Exercise::RepsOrDuration).not_null())
                    .col(string_len(Exercise::Description, 255).null())
                    .col(string_len(Exercise::Visibility, 50).not_null())
                    .col(timestamp_with_time_zone(Exercise::CreatedAt).null())
                    .col(timestamp_with_time_zone(Exercise::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk-exercise-owner-id")
                    .from(Exercise::Table, Exercise::OwnerId)
                    .to(Person::Table, Person::Id)
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
                    .name("fk-exercise-owner-id")
                    .table(Exercise::Table)
                    .to_owned(),
            )
            .await?;

        manager
            .drop_table(Table::drop().table(Exercise::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Exercise {
    Table,
    Id,
    Uuid,
    Name,
    OwnerId,
    OwnerUuid,
    OwnerName,
    Category,
    Sets,
    RepsOrDuration,
    Description,
    Visibility,
    CreatedAt,
    UpdatedAt,
}
