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
                    .table(Consent::Table)
                    .if_not_exists()
                    .col(pk_auto(Consent::Id))
                    .col(uuid_uniq(Consent::Uuid))
                    .col(integer(Consent::PersonId).not_null())
                    .col(string_len(Consent::Document, 64).not_null())
                    .col(string_len(Consent::Version, 64).not_null())
                    .col(timestamp_with_time_zone(Consent::AcceptedAt).not_null())
                    .col(string_len(Consent::Ip, 45).not_null())
                    .col(timestamp_with_time_zone(Consent::RevokedAt).null())
                    .foreign_key(
                        ForeignKey::create()
                            .name("fk_consent_person")
                            .from(Consent::Table, Consent::PersonId)
                            .to(Person::Table, Person::Id)
                            .on_delete(ForeignKeyAction::Cascade),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_consent_person_document_active")
                    .table(Consent::Table)
                    .col(Consent::PersonId)
                    .col(Consent::Document)
                    .col(Consent::RevokedAt)
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(Table::drop().table(Consent::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
pub enum Consent {
    Table,
    Id,
    Uuid,
    PersonId,
    Document,
    Version,
    AcceptedAt,
    Ip,
    RevokedAt,
}
