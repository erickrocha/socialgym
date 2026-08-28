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
                    .table(DataExport::Table)
                    .if_not_exists()
                    .col(pk_auto(DataExport::Id))
                    .col(uuid_uniq(DataExport::Uuid))
                    .col(integer(DataExport::PersonId).not_null())
                    .col(string_len(DataExport::Status, 32).not_null())
                    .col(string_len(DataExport::ObjectKey, 2000).null())
                    .col(text(DataExport::Error).null())
                    .col(timestamp_with_time_zone(DataExport::CreatedAt).not_null())
                    .col(timestamp_with_time_zone(DataExport::UpdatedAt).not_null())
                    .col(timestamp_with_time_zone(DataExport::ExpiresAt).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_data_export_person")
                            .from(DataExport::Table, DataExport::PersonId)
                            .to(Person::Table, Person::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(DataExport::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum DataExport {
    Table,
    Id,
    Uuid,
    PersonId,
    Status,
    ObjectKey,
    Error,
    CreatedAt,
    UpdatedAt,
    ExpiresAt,
}
