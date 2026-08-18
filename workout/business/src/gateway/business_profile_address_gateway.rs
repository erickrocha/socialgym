use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_profile_address::{
    BusinessProfileAddress, BusinessProfileAddressEntityMapper,
};
use entity::business_profile_address;
use entity::prelude::BusinessProfileAddress as BusinessProfileAddressQuery;
use sea_orm::{ActiveModelTrait, ColumnTrait, DbConn, DbErr, EntityTrait, QueryFilter};

pub struct BusinessProfileAddressGateway {}

impl BusinessProfileAddressGateway {
    pub async fn find_all_by_business_profile_id(db: &DbConn,business_profile_id: i32) -> Result<Vec<business_profile_address::Model>, DbErr> {
        BusinessProfileAddressQuery::find()
            .filter(
                business_profile_address::Column::BusinessProfileId.eq(business_profile_id),
            )
            .all(db)
            .await
    }

    pub async fn persist(db: &DbConn,domain: BusinessProfileAddress) -> Result<business_profile_address::ActiveModel, DbErr> {
        let active_model = BusinessProfileAddressEntityMapper::build_active_model(domain);
        active_model.save(db).await
    }

    pub async fn find_by_id(db: &DbConn,id: i32) -> Result<Option<business_profile_address::Model>, DbErr> {
        BusinessProfileAddressQuery::find()
            .filter(business_profile_address::Column::Id.eq(id))
            .one(db)
            .await
    }
    
    pub async fn delete_by_id(db: &DbConn, id: i32) -> Result<(), DbErr> {
        let address = BusinessProfileAddressQuery::find_by_id(id).one(db).await?;
        if let Some(address) = address {
            let active_model: business_profile_address::ActiveModel = address.into();
            active_model.delete(db).await?;
        }
        Ok(())
    }
}
