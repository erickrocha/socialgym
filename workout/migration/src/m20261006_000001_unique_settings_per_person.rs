use sea_orm_migration::prelude::*;

/// `settings.person_id` had no unique constraint, so a repeated POST (or a retried
/// bootstrap) stored several settings rows for one person and lookups by owner
/// returned an arbitrary one. Keep one row per person — the most recently updated,
/// highest id on a tie — and enforce it from now on.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared(
            "DELETE FROM settings a \
               USING settings b \
              WHERE a.person_id = b.person_id \
                AND (a.updated_at < b.updated_at \
                     OR (a.updated_at = b.updated_at AND a.id < b.id))",
        )
        .await?;
        db.execute_unprepared(
            "CREATE UNIQUE INDEX IF NOT EXISTS uq_settings_person ON settings (person_id)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("DROP INDEX IF EXISTS uq_settings_person")
            .await?;
        Ok(())
    }
}
