use crate::commons::entity_mapper::EntityMapper;
use crate::domain::revoked_token::{RevokedToken, RevokedTokenMapper};
use chrono::NaiveDateTime;
use entity::prelude::RevokedTokenEntity as RevokedTokenQuery;
use entity::revoked_token_entity as revoked_token;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, ConnectionTrait, DbConn, DbErr, DeleteResult, EntityTrait,
    QueryFilter,
};

pub struct TokenRevocationGateway {}

impl TokenRevocationGateway {
    pub async fn revoke(db: &DbConn, jti: String, user_id: i32, token_type: &str, expires_at: NaiveDateTime) -> Result<(), DbErr> {
        let revoked = RevokedToken::new(jti, user_id, token_type.to_string(), expires_at);
        let active_model = RevokedTokenMapper::build_active_model(revoked);
        active_model.save(db).await?;
        Ok(())
    }

    pub async fn is_revoked(db: &DbConn, jti: &str) -> Result<bool, DbErr> {
        let found = RevokedTokenQuery::find()
            .filter(revoked_token::Column::Jti.eq(jti))
            .one(db)
            .await?;
        Ok(found.is_some())
    }

    /// Bulk-deletes the revoked-token audit rows for a user — must run before the
    /// `user` row itself is deleted (plain FK, no cascade) in an account-purge cascade.
    pub async fn delete_all_by_user_id<C: ConnectionTrait>(db: &C, user_id: i32) -> Result<DeleteResult, DbErr> {
        RevokedTokenQuery::delete_many()
            .filter(revoked_token::Column::UserId.eq(user_id))
            .exec(db)
            .await
    }
}
