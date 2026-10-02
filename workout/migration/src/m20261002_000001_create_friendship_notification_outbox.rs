use sea_orm_migration::prelude::*;
use sea_orm_migration::schema::{integer, pk_auto, uuid_uniq};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .create_table(
                Table::create()
                    .table(FriendshipNotificationOutbox::Table)
                    .if_not_exists()
                    .col(pk_auto(FriendshipNotificationOutbox::Id))
                    .col(uuid_uniq(FriendshipNotificationOutbox::EventUuid))
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::FriendshipUuid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::EventType)
                            .string_len(40)
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::ActorPersonUuid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::RecipientPersonUuid)
                            .uuid()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::OccurredAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(integer(FriendshipNotificationOutbox::AttemptCount).default(0))
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::NextAttemptAt)
                            .timestamp_with_time_zone()
                            .not_null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::LastError)
                            .text()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::PublishedAt)
                            .timestamp_with_time_zone()
                            .null(),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::CreatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .col(
                        ColumnDef::new(FriendshipNotificationOutbox::UpdatedAt)
                            .timestamp_with_time_zone()
                            .not_null()
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;

        manager
            .create_index(
                Index::create()
                    .name("idx_friendship_outbox_pending")
                    .table(FriendshipNotificationOutbox::Table)
                    .col(FriendshipNotificationOutbox::PublishedAt)
                    .col(FriendshipNotificationOutbox::NextAttemptAt)
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        manager
            .drop_table(
                Table::drop()
                    .table(FriendshipNotificationOutbox::Table)
                    .to_owned(),
            )
            .await
    }
}

#[derive(DeriveIden)]
enum FriendshipNotificationOutbox {
    Table,
    Id,
    EventUuid,
    FriendshipUuid,
    EventType,
    ActorPersonUuid,
    RecipientPersonUuid,
    OccurredAt,
    AttemptCount,
    NextAttemptAt,
    LastError,
    PublishedAt,
    CreatedAt,
    UpdatedAt,
}
