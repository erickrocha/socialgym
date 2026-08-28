use crate::m20260129_000001_create_table_countries::Country;
use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let insert = Query::insert()
            .into_table(Country::Table)
            .columns([
                Country::Ddi,
                Country::Name,
                Country::Acronym,
                Country::Currency,
            ])
            .values_panic(["55".into(), "Brazil".into(), "BR".into(), "BRL".into()])
            .values_panic(["1".into(), "USA".into(), "US".into(), "USD".into()])
            .values_panic(["34".into(), "Spain".into(), "ES".into(), "EUR".into()])
            .values_panic(["33".into(), "France".into(), "FR".into(), "EUR".into()])
            .values_panic(["49".into(), "Germany".into(), "DE".into(), "EUR".into()])
            .to_owned();

        manager.execute(insert).await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let delete = Query::delete()
            .from_table(Country::Table)
            .and_where(Expr::col(Country::Ddi).is_in(["55", "1", "34", "33", "49"]))
            .to_owned();

        manager.execute(delete).await
    }
}
