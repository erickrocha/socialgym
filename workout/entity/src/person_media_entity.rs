use sea_orm::entity::prelude::*;
use sea_orm::prelude::async_trait::async_trait;
use sea_orm::Set;

pub type PersonMediaEntity = Model;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel)]
#[sea_orm(table_name = "person_media")]
pub struct Model {
    #[sea_orm(primary_key)]
    pub id: i32,
    #[sea_orm(unique)]
    pub uuid: Uuid,
    pub person_id: i32,
    pub person_uuid: Uuid,
    pub mime_type: String,
    pub album: String,
    pub s3_key: String,
    pub created_at: DateTimeUtc,
    pub updated_at: DateTimeUtc,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(
        belongs_to = "super::person_entity::Entity",
        from = "Column::PersonId",
        to = "super::person_entity::Column::Id"
    )]
    Person,
}

impl Related<super::person_entity::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Person.def()
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
