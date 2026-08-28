use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;
use uuid::Uuid;

pub type RevokedTokenEntity = Model;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "revoked_token")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    #[sea_orm(unique)]
    pub jti: String,
    pub user_id: i32,
    pub token_type: String,
    pub expires_at: DateTimeUtc,
    pub created_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.uuid = Set(Uuid::new_v4());
            self.created_at = Set(chrono::Utc::now().to_utc());
        }
        Ok(self)
    }
}
