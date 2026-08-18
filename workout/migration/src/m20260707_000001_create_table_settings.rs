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
                    .table(Settings::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Settings::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(uuid_uniq(Settings::Uuid))
                    .col(ColumnDef::new(Settings::PersonId).integer().not_null())
                    .col(uuid(Settings::PersonUuid))
                    .col(ColumnDef::new(Settings::Language).string().not_null())
                    .col(ColumnDef::new(Settings::Theme).string().not_null())
                    .col(
                        ColumnDef::new(Settings::NotificationsEnabled)
                            .boolean()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(Settings::ContextMenuPosition)
                            .string()
                            .not_null(),
                    )
                    .col(ColumnDef::new(Settings::HomePage).string().not_null())
                    .col(
                        ColumnDef::new(Settings::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Settings::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_settings_person")
                    .from_tbl(Settings::Table)
                    .from_col(Settings::PersonId)
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
                    .name("fk_settings_person")
                    .table(Settings::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Settings::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Settings {
    Table,
    Id,
    Uuid,
    PersonId,
    PersonUuid,
    Language,
    Theme,
    NotificationsEnabled,
    ContextMenuPosition,
    HomePage,
    CreatedAt,
    UpdatedAt,
}
