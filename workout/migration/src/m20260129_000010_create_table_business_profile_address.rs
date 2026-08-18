use crate::m20260129_000009_create_table_business_profile::BusinessProfile;
use crate::{DeriveMigrationName, async_trait};
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
                    .table(BusinessProfileAddress::Table)
                    .if_not_exists()
                    .col(pk_auto(BusinessProfileAddress::Id))
                    .col(uuid_uniq(BusinessProfileAddress::Uuid))
                    .col(integer(BusinessProfileAddress::BusinessProfileId).not_null())
                    .col(string_len(BusinessProfileAddress::AddressLine1, 500).not_null())
                    .col(string_len(BusinessProfileAddress::AddressLine2, 200).null())
                    .col(string_len(BusinessProfileAddress::Locality, 200).null())
                    .col(string_len(BusinessProfileAddress::AdministrativeArea, 200).null())
                    .col(string_len(BusinessProfileAddress::PostalCode, 10).null())
                    .col(string_len(BusinessProfileAddress::CountryCode, 10).null())
                    .col(double(BusinessProfileAddress::Latitude).null())
                    .col(double(BusinessProfileAddress::Longitude).null())
                    .col(timestamp_with_time_zone(BusinessProfileAddress::CreatedAt).null())
                    .col(timestamp_with_time_zone(BusinessProfileAddress::UpdatedAt).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_business_profile_address_business_profile")
                    .from_tbl(BusinessProfileAddress::Table)
                    .from_col(BusinessProfileAddress::BusinessProfileId)
                    .to_tbl(BusinessProfile::Table)
                    .to_col(BusinessProfile::Id)
                    .to_owned(),
            )
            .await?;
        manager
            .get_connection()
            .execute_unprepared(
                r#"ALTER TABLE business_profile_address
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
                "CREATE INDEX idx_business_profile_address_location ON business_profile_address USING GIST (location)",
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_business_profile_address_business_profile")
                    .table(BusinessProfileAddress::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(
                Table::drop()
                    .table(BusinessProfileAddress::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum BusinessProfileAddress {
    Table,
    Id,
    Uuid,
    BusinessProfileId,
    AddressLine1,
    AddressLine2,
    Locality,
    AdministrativeArea,
    PostalCode,
    CountryCode,
    Latitude,
    Longitude,
    CreatedAt,
    UpdatedAt,
}
