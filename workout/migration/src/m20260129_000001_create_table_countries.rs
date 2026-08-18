use sea_orm_migration::{prelude::*, schema::*};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
	async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.create_table(
				Table::create()
					.table(Country::Table)
					.if_not_exists()
					.col(pk_auto(Country::Id).integer())
					.col(string_uniq(Country::Ddi).not_null())
					.col(string_len(Country::Name, 200).not_null())
					.col(string_len(Country::Acronym, 10).not_null())
					.col(string_len(Country::Currency, 100).not_null())
					.to_owned(),
			)
			.await?;
		Ok(())
	}

	async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
		manager
			.drop_table(Table::drop().table(Country::Table).to_owned())
			.await
	}
}

#[derive(DeriveIden)]
pub enum Country {
	Table,
	Id,
	Ddi,
	Name,
	Acronym,
	Currency
}
