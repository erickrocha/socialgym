use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::string_to_uuid;
use crate::domain::business_profile::{BusinessProfile, BusinessProfileEntityMapper};
use entity::business_profile::{ActiveModel, Model};
use entity::prelude::BusinessProfile as BusinessProfileQuery;
use sea_orm::{
    ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, PaginatorTrait, QueryFilter,
};

pub struct BusinessProfileGateway {}

impl BusinessProfileGateway {
    pub async fn persist(db: &DbConn, entity: BusinessProfile) -> Result<ActiveModel, DbErr> {
        let active_model = BusinessProfileEntityMapper::build_active_model(entity);
        active_model.save(db).await
    }

    pub async fn find_by_id(db: &DbConn, id: i32) -> Option<Model> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile::Column::Id.eq(id))
            .one(db)
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: &str) -> Option<Model> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile::Column::Uuid.eq(string_to_uuid(uuid)))
            .one(db)
            .await
            .unwrap_or(None)
    }

    pub async fn find_by_owner_id(db: &DbConn, owner_id: i32) -> Vec<Model> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile::Column::OwnerId.eq(owner_id))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn find_by_owner_uuid(db: &DbConn, owner_uuid: &str) -> Vec<Model> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile::Column::OwnerUuid.eq(string_to_uuid(owner_uuid)))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }

    pub async fn count_by_owner_id(db: &DbConn, owner_id: i32) -> u64 {
        BusinessProfileQuery::find()
            .filter(entity::business_profile::Column::OwnerId.eq(owner_id))
            .count(db)
            .await
            .unwrap_or(0)
    }

    pub async fn find_all_by_ids(db: &DbConn, ids: Vec<i32>) -> Vec<Model> {
        BusinessProfileQuery::find()
            .filter(entity::business_profile::Column::Id.is_in(ids))
            .all(db)
            .await
            .unwrap_or_else(|_| Vec::new())
    }
}
