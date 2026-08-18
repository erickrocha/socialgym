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
                    .table(PersonAddress::Table)
                    .if_not_exists()
                    .col(pk_auto(PersonAddress::Id).integer())
                    .col(uuid_uniq(PersonAddress::Uuid))
                    .col(integer(PersonAddress::PersonId).not_null())
                    .col(string_len_null(PersonAddress::AddressLine1, 500))
                    .col(string_len_null(PersonAddress::AddressLine2, 500))
                    .col(string_len_null(PersonAddress::Locality, 500))
                    .col(string_len_null(PersonAddress::AdministrativeArea, 500))
                    .col(string_len_null(PersonAddress::PostalCode, 20))
                    .col(string_len_null(PersonAddress::CountryCode, 2))
                    .col(boolean(PersonAddress::Current).not_null().default(true))
                    .col(double_null(PersonAddress::Latitude))
                    .col(double_null(PersonAddress::Longitude))
                    .col(timestamp_with_time_zone(PersonAddress::CreatedAt).null())
                    .col(timestamp_with_time_zone(PersonAddress::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;
        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_person_address_person")
                    .from_tbl(PersonAddress::Table)
                    .from_col(PersonAddress::PersonId)
                    .to_tbl(Person::Table)
                    .to_col(Person::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE person_address
                   ADD COLUMN location geography(Point, 4326)
                   GENERATED ALWAYS AS (
                     CASE WHEN latitude IS NULL OR longitude IS NULL THEN NULL
                          ELSE ST_SetSRID(ST_MakePoint(longitude, latitude), 4326)::geography
                     END
                   ) STORED"#,
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                "CREATE INDEX idx_person_address_location ON person_address USING GIST (location)",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .table(PersonAddress::Table)
                    .name("fk_person_address_person")
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(PersonAddress::Table).to_owned())
            .await
    }
}

#[derive(DeriveIden)]
enum PersonAddress {
    Table,
    Id,
    Uuid,
    PersonId,
    AddressLine1,
    AddressLine2,
    Locality,
    AdministrativeArea,
    PostalCode,
    CountryCode,
    Current,
    Latitude,
    Longitude,
    CreatedAt,
    UpdatedAt,
}
