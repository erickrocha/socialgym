use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;

pub type BusinessProfileEntity = Model;

#[derive(Clone, Debug, PartialEq, Eq, DeriveEntityModel)]
#[sea_orm(table_name = "business_profile")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub owner_id: i32,
    pub owner_uuid: Uuid,
    pub tax_id: String,
    pub business_name: String,
    pub business_type: String,
    pub social_name: Option<String>,
    pub logo: Option<String>,
    pub cover_image: Option<String>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::business_profile_address_entity::Entity")]
    BusinessProfileAddress,
    #[sea_orm(has_many = "super::profile_entity::Entity")]
    Profile,
}

impl Related<super::business_profile_address_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BusinessProfileAddress.def()
    }
}

impl Related<super::profile_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Profile.def()
    }
}

#[async_trait]
impl ActiveModelBehavior for ActiveModel {
    async fn before_save<C>(mut self, _db: &C, insert: bool) -> Result<Self, DbErr>
    where
        C: ConnectionTrait,
    {
        if insert {
            self.uuid = Set(Uuid::new_v4());
            self.created_at = Set(chrono::Utc::now());
        }
        self.updated_at = Set(chrono::Utc::now());
        Ok(self)
    }
}
