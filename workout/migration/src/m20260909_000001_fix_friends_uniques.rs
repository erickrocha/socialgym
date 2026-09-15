use crate::m20260129_000011_create_table_friends::Friends;
use sea_orm_migration::prelude::*;

/// `m20260129_000011` created `friends.person_uuid` / `friends.friend_uuid` with
/// `uuid_uniq`, so each column carries its own UNIQUE constraint. That means a
/// person can appear as `person_uuid` in at most one row ever (and as
/// `friend_uuid` in at most one row ever) — the second friend request anyone
/// sends or receives fails the INSERT and the controller reports it as 400.
///
/// Drop those two column-level uniques and replace them with the constraint that
/// was actually intended: one friendship row per ordered `(person_id, friend_id)`
/// pair. The reverse direction is still handled in `FriendUseCase`
/// (auto-accept / re-open), so a plain same-direction unique is enough.
#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Postgres names the constraints created by `uuid_uniq(col)` as
        // `<table>_<col>_key`.
        db.execute_unprepared(
            "ALTER TABLE friends \
                 DROP CONSTRAINT IF EXISTS friends_person_uuid_key, \
                 DROP CONSTRAINT IF EXISTS friends_friend_uuid_key",
        )
        .await?;

        // Defensively collapse any pre-existing duplicate pairs so the unique
        // index below can be created (keeps the lowest id of each pair).
        db.execute_unprepared(
            "DELETE FROM friends a \
               USING friends b \
              WHERE a.id > b.id \
                AND a.person_id = b.person_id \
                AND a.friend_id = b.friend_id",
        )
        .await?;

        manager
            .create_index(
                Index::create()
                    .name("uq_friends_person_friend")
                    .table(Friends::Table)
                    .col(Friends::PersonId)
                    .col(Friends::FriendId)
                    .unique()
                    .if_not_exists()
                    .to_owned(),
            )
            .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        // The old per-column uniques were a bug; do not recreate them.
        manager
            .drop_index(
                Index::drop()
                    .name("uq_friends_person_friend")
                    .table(Friends::Table)
                    .to_owned(),
            )
            .await?;
        Ok(())
    }
}
