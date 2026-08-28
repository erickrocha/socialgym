use business::sea_orm::{ActiveModelTrait, ColumnTrait, Database, EntityTrait, QueryFilter, Set};
use entity::{user_entity, user_role_entity};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenvy::dotenv().ok();
    let mut args = std::env::args().skip(1);
    let user_uuid = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("usage: grant_role <user-uuid> <role>"))?;
    let role = args
        .next()
        .ok_or_else(|| anyhow::anyhow!("usage: grant_role <user-uuid> <role>"))?;
    if role != "moderator" {
        anyhow::bail!("unsupported role: {role}");
    }
    let uuid = uuid::Uuid::parse_str(&user_uuid)?;
    let database_url = std::env::var("DATABASE_URL")?;
    let db = Database::connect(database_url).await?;
    let user = user_entity::Entity::find()
        .filter(user_entity::Column::Uuid.eq(uuid))
        .one(&db)
        .await?
        .ok_or_else(|| anyhow::anyhow!("user not found"))?;
    user_role_entity::ActiveModel {
        user_id: Set(user.id),
        role: Set(role.clone()),
        ..Default::default()
    }
    .insert(&db)
    .await?;
    println!("granted role {role} to {user_uuid}");
    Ok(())
}
