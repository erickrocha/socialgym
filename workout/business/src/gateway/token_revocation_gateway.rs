use crate::commons::entity_mapper::EntityMapper;
use crate::domain::revoked_token::{RevokedToken, RevokedTokenMapper};
use chrono::NaiveDateTime;
use entity::prelude::RevokedToken as RevokedTokenQuery;
use entity::revoked_token;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

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
}
