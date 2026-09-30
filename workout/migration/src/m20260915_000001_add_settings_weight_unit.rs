use sea_orm_migration::prelude::*;

/// Adds an explicit, nullable weight-unit override to `settings`. `NULL`
/// means "no explicit choice yet" — clients derive the effective unit from
/// the person's current language (English -> pounds, else kilograms) until
/// they pick one here, at which point it always wins.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .add_column(ColumnDef::new(Settings::WeightUnit).string().null())
                    .to_owned(),
            )
            .await
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .alter_table(
                Table::alter()
                    .table(Settings::Table)
                    .drop_column(Settings::WeightUnit)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
pub enum Settings {
    Table,
    WeightUnit,
}
