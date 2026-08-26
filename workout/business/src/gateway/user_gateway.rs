use crate::commons::entity_mapper::EntityMapper;
use crate::domain::user::{User, UserEntityMapper};
use chrono::{DateTime, Utc};
use entity::prelude::UserEntity as UserQuery;
use entity::user_entity as user;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set,
};

pub struct UserGateway {}

impl UserGateway {
    pub async fn persist(db: &DbConn, entity: User) -> Result<user::ActiveModel, DbErr> {
        let active_model = UserEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, entity: User) -> Result<user::UserEntity, DbErr> {
        let active_model = UserEntityMapper::build_active_model(entity);
        active_model.update(db).await
    }

    /// Sets the failed-login counter to `new_count`. Callers compute the new value
    /// themselves (usually `current + 1`) to avoid a read-then-write round trip here.
    pub async fn increment_failed_attempts(db: &DbConn, user_id: i32, new_count: i32) -> Result<user::UserEntity, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            failed_login_attempts: Set(new_count),
            ..Default::default()
        };
        active_model.update(db).await
    }

    pub async fn lock_account(db: &DbConn, user_id: i32, locked_until: DateTime<Utc>) -> Result<user::UserEntity, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            locked_until: Set(Some(locked_until)),
            ..Default::default()
        };
        active_model.update(db).await
    }

    /// Clears the failed-attempt counter and any active lock (auto-unlock / reset-on-success).
    pub async fn reset_lockout(db: &DbConn, user_id: i32) -> Result<user::UserEntity, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            failed_login_attempts: Set(0),
            locked_until: Set(None),
            ..Default::default()
        };
        active_model.update(db).await
    }

    /// Sets the revoke-all watermark: any token whose `iat` is at or before `watermark`
    /// is treated as revoked once this is set.
    pub async fn set_token_valid_after(db: &DbConn, user_id: i32, watermark: DateTime<Utc>) -> Result<user::UserEntity, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            token_valid_after: Set(Some(watermark)),
            ..Default::default()
        };
        active_model.update(db).await
    }

    /// Disables the account and schedules the cascade purge: `enabled = false`,
    /// `deletion_requested_at = now`, `deletion_scheduled_at = requested_at` (immediate)
    /// or `requested_at + grace period` (30-day path).
    pub async fn schedule_deletion(
        db: &DbConn,
        user_id: i32,
        requested_at: DateTime<Utc>,
        scheduled_at: DateTime<Utc>,
    ) -> Result<user::UserEntity, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            enabled: Set(false),
            deletion_requested_at: Set(Some(requested_at)),
            deletion_scheduled_at: Set(Some(scheduled_at)),
            ..Default::default()
        };
        active_model.update(db).await
    }

    /// Cancels a pending deletion: re-enables the account and clears both timestamps.
    pub async fn cancel_deletion(db: &DbConn, user_id: i32) -> Result<user::UserEntity, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            enabled: Set(true),
            deletion_requested_at: Set(None),
            deletion_scheduled_at: Set(None),
            ..Default::default()
        };
        active_model.update(db).await
    }

    /// Accounts whose grace period (or immediate-deletion request) has elapsed and are
    /// still disabled — i.e. not yet purged. The sweep job's entry point.
    pub async fn find_due_for_deletion(db: &DbConn, now: DateTime<Utc>) -> Result<Vec<user::UserEntity>, DbErr> {
        UserQuery::find()
            .filter(user::Column::Enabled.eq(false))
            .filter(user::Column::DeletionScheduledAt.is_not_null())
            .filter(user::Column::DeletionScheduledAt.lte(now))
            .all(db)
            .await
    }

    /// Deletes the `user` row for a person as part of the account-purge cascade.
    /// Must run after `revoked_token` rows for this user are gone (plain FK, no cascade).
    pub async fn delete_by_person_id<C: ConnectionTrait>(db: &C, person_id: i32) -> Result<sea_orm::DeleteResult, DbErr> {
        UserQuery::delete_many()
            .filter(user::Column::PersonId.eq(person_id))
            .exec(db)
            .await
    }

    pub async fn find_by_email(db: &DbConn, email: String) -> Result<Option<user::UserEntity>, DbErr> {
        UserQuery::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<user::UserEntity>, DbErr> {
        UserQuery::find()
            .filter(user::Column::Id.eq(id))
            .one(db)
            .await
    }
}
