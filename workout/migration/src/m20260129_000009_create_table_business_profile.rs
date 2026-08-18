use crate::m20260129_000003_create_table_person::Person;
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
                    .table(BusinessProfile::Table)
                    .if_not_exists()
                    .col(pk_auto(BusinessProfile::Id))
                    .col(uuid_uniq(BusinessProfile::Uuid))
                    .col(integer(BusinessProfile::OwnerId).not_null())
                    .col(uuid(BusinessProfile::OwnerUuid))
                    .col(string_len(BusinessProfile::TaxId, 100).not_null())
                    .col(string_len(BusinessProfile::BusinessName, 255).not_null())
                    .col(string_len(BusinessProfile::BusinessType, 100).not_null())
                    .col(string_len(BusinessProfile::SocialName, 255).null())
                    .col(string_len(BusinessProfile::Logo, 500).null())
                    .col(string_len(BusinessProfile::CoverImage, 500).null())
                    .col(timestamp_with_time_zone(BusinessProfile::CreatedAt).null())
                    .col(string_len(BusinessProfile::CreatedBy, 100).null())
                    .col(timestamp_with_time_zone(BusinessProfile::UpdatedAt).null())
                    .col(string_len(BusinessProfile::UpdatedBy, 100).null())
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_business_profile_person")
                    .from_tbl(BusinessProfile::Table)
                    .from_col(BusinessProfile::OwnerId)
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
                    .name("fk_business_profile_person")
                    .table(BusinessProfile::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(BusinessProfile::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum BusinessProfile {
    Table,
    Id,
    Uuid,
    OwnerId,
    OwnerUuid,
    TaxId,
    BusinessName,
    BusinessType,
    SocialName,
    Logo,
    CoverImage,
    CreatedAt,
    CreatedBy,
    UpdatedAt,
    UpdatedBy,
}
