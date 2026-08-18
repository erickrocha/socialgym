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
                    .table(TeamMembers::Table)
                    .if_not_exists()
                    .col(pk_auto(TeamMembers::Id))
                    .col(uuid_uniq(TeamMembers::Uuid))
                    .col(integer(TeamMembers::BusinessProfileId).not_null())
                    .col(uuid(TeamMembers::BusinessProfileUuid).not_null())
                    .col(integer(TeamMembers::PersonId).not_null())
                    .col(uuid(TeamMembers::PersonUuid).not_null())
                    .col(
                        string_len(TeamMembers::Status, 20)
                            .not_null()
                            .default("Pending"),
                    )
                    .to_owned()
                    .col(
                        ColumnDef::new(TeamMembers::CreatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(TeamMembers::UpdatedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("team_members_business_profile_id")
                    .from(TeamMembers::Table, TeamMembers::BusinessProfileId)
                    .to(BusinessProfile::Table, BusinessProfile::Id)
                    .to_owned(),
            )
            .await?;

        manager
            .create_foreign_key(
                ForeignKey::create()
                    .name("fk_team_members_person")
                    .from(TeamMembers::Table, TeamMembers::PersonId)
                    .to(Person::Table, Person::Id)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("team_members_business_profile_id")
                    .table(TeamMembers::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_foreign_key(
                ForeignKey::drop()
                    .name("fk_team_members_person")
                    .table(TeamMembers::Table)
                    .to_owned(),
            )
            .await?;
        manager
            .drop_table(Table::drop().table(TeamMembers::Table).to_owned())
            .await?;
        Ok(())
    }
}

#[derive(DeriveIden)]
pub enum TeamMembers {
    Table,
    Id,
    Uuid,
    BusinessProfileId,
    BusinessProfileUuid,
    PersonId,
    PersonUuid,
    CreatedAt,
    UpdatedAt,
    Status,
}
