use sea_orm_migration::prelude::*;

/// `person_media.mime_type` was `varchar(20)`, but the fallback for an unrecognised content
/// type is `application/octet-stream` (24 characters). The INSERT failed, the SQS consumer
/// treated it as a transient error and the message was redelivered forever without the media
/// row ever being stored. Content types can be long (`; charset=...` is stripped, vendor types
/// are not), so give the column room.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .get_connection()
            .execute_unprepared("ALTER TABLE person_media ALTER COLUMN mime_type TYPE varchar(127)")
            .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // Narrowing back would truncate or fail on long values; keep the wider column.
        let _ = manager;
        Ok(())
    }
}
