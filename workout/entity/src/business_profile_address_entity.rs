use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;

pub type BusinessProfileAddressEntity = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "business_profile_address")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub business_profile_id: i32,
    pub address_line1: String,
    pub address_line2: Option<String>,
    pub locality: String,
    pub administrative_area: String,
    pub postal_code: Option<String>,
    pub country_code: String,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::business_profile_entity::Entity",
        from = "Column::BusinessProfileId",
        to = "super::business_profile_entity::Column::Id"
    )]
    BusinessProfile,
}

impl Related<super::business_profile_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::BusinessProfile.def()
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
            self.created_at = Set(chrono::Utc::now().to_utc());
        }
        self.updated_at = Set(chrono::Utc::now().to_utc());
        Ok(self)
    }
}
