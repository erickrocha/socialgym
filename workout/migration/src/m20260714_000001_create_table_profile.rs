use crate::m20260129_000003_create_table_person::Person;
use crate::m20260129_000009_create_table_business_profile::BusinessProfile;
use crate::{DbErr, DeriveMigrationName, async_trait};
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(Profile::Table)
                    .if_not_exists()
                    .col(
                        pk_auto(Profile::Id)
                            .integer()
                            .not_null()
                            .auto_increment()
                            .primary_key(),
                    )
                    .col(uuid_uniq(Profile::Uuid))
                    .col(ColumnDef::new(Profile::PersonId).integer().not_null())
                    .col(uuid(Profile::PersonUuid))
                    .col(
                        ColumnDef::new(Profile::BusinessProfileId)
                            .integer()
                            .not_null(),
                    )
                    .col(uuid(Profile::BusinessProfileUuid))
                    .col(
                        ColumnDef::new(Profile::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(Profile::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_profile_person_id")
                    .from(Profile::Table, Profile::PersonId)
                    .to(Person::Table, Person::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_profile_business_profile_id")
                    .from(Profile::Table, Profile::BusinessProfileId)
                    .to(BusinessProfile::Table, BusinessProfile::Id)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_profile_person_id")
                    .table(Profile::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_profile_business_profile_id")
                    .table(Profile::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(Profile::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum Profile {
    Table,
    Id,
    Uuid,
    PersonId,
    PersonUuid,
    BusinessProfileId,
    BusinessProfileUuid,
    CreatedAt,
    UpdatedAt,
}
