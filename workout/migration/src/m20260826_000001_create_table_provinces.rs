use crate::m20260129_000001_create_table_countries::Country;
use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Province::Table)
					.if_not_exists()
					.col(pk_auto(Province::Id).integer())
					.col(string_len(Province::Name, 200).not_null())
					.col(string_len(Province::Acronym, 10).not_null())
					.col(integer(Province::CountryId).not_null())
					.to_owned(),
			)
			.await?;
		manager
			.create_foreign_key(
				ForeignKey::create()
					.name("fk_province_country")
					.from_tbl(Province::Table)
					.from_col(Province::CountryId)
					.to_tbl(Country::Table)
					.to_col(Country::Id)
					.to_owned(),
			)
			.await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_foreign_key(
				ForeignKey::drop()
					.table(Province::Table)
					.name("fk_province_country")
					.to_owned(),
			)
			.await?;
		manager
			.drop_table(Table::drop().table(Province::Table).to_owned())
			.await
	}
}

#[derive(DeriveIden)]
pub enum Province {
	Table,
	Id,
	Name,
	Acronym,
	CountryId,
}
