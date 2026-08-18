use crate::commons::entity_mapper::EntityMapper;
use crate::domain::user::{User, UserEntityMapper};
use chrono::{DateTime, Utc};
use entity::prelude::User as UserQuery;
use entity::user;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter, Set};

pub struct UserGateway {}

impl UserGateway {
    pub async fn persist(db: &DbConn, entity: User) -> Result<user::ActiveModel, DbErr> {
        let active_model = UserEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn update(db: &DbConn, entity: User) -> Result<user::Model, DbErr> {
        let active_model = UserEntityMapper::build_active_model(entity);
        active_model.update(db).await
    }

    /// Sets the failed-login counter to `new_count`. Callers compute the new value
    /// themselves (usually `current + 1`) to avoid a read-then-write round trip here.
    pub async fn increment_failed_attempts(db: &DbConn, user_id: i32, new_count: i32) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            failed_login_attempts: Set(new_count),
            ..Default::default()
        };
        active_model.update(db).await
    }

    pub async fn lock_account(db: &DbConn, user_id: i32, locked_until: DateTime<Utc>) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            locked_until: Set(Some(locked_until)),
            ..Default::default()
        };
        active_model.update(db).await
    }

    /// Clears the failed-attempt counter and any active lock (auto-unlock / reset-on-success).
    pub async fn reset_lockout(db: &DbConn, user_id: i32) -> Result<user::Model, DbErr> {
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
    pub async fn set_token_valid_after(db: &DbConn, user_id: i32, watermark: DateTime<Utc>) -> Result<user::Model, DbErr> {
        let active_model = user::ActiveModel {
            id: Set(user_id),
            token_valid_after: Set(Some(watermark)),
            ..Default::default()
        };
        active_model.update(db).await
    }

    pub async fn find_by_email(db: &DbConn, email: String) -> Result<Option<user::Model>, DbErr> {
        UserQuery::find()
            .filter(user::Column::Email.eq(email))
            .one(db)
            .await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Result<Option<user::Model>, DbErr> {
        UserQuery::find()
            .filter(user::Column::Id.eq(id))
            .one(db)
            .await
    }
}
