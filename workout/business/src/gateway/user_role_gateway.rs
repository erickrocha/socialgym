use entity::user_role_entity as role;
use sea_orm::{ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};
pub struct UserRoleGateway;
impl UserRoleGateway {
    pub async fn has_role(db: &DbConn, user_id: i32, value: &str) -> Result<bool, DbErr> {
        Ok(role::Entity::find()
            .filter(role::Column::UserId.eq(user_id))
            .filter(role::Column::Role.eq(value))
            .one(db)
            .await?
            .is_some())
    }
    pub async fn list(db: &DbConn, user_id: i32) -> Result<Vec<String>, DbErr> {
        Ok(role::Entity::find()
            .filter(role::Column::UserId.eq(user_id))
            .all(db)
            .await?
            .into_iter()
            .map(|r| r.role)
            .collect())
    }
}
