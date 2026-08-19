use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile_address::{
    BusinessProfileAddress, BusinessProfileAddressEntityMapper,
};
use crate::gateway::business_profile_address_gateway::BusinessProfileAddressGateway;
use sea_orm::DbConn;

pub struct BusinessProfileAddressUseCase;

impl BusinessProfileAddressUseCase {
    pub async fn save(
        db: &DbConn,
        domain: BusinessProfileAddress,
    ) -> Result<BusinessProfileAddress, BusinessError> {
        log::info!(
            "Adding BusinessProfileAddress for business_profile_id={}",
            domain.business_profile_id
        );
        let domain_saved = BusinessProfileAddressGateway::persist(db, domain)
            .await
            .map_err(|e| {
                log::error!("Error adding BusinessProfileAddress: {:?}", e);
                BusinessError::new(format!("Error adding BusinessProfileAddress: {:?}", e))
            })?;
        let entity = BusinessProfileAddressEntityMapper::from_active_model(domain_saved.clone());
        Ok(entity)
    }

    pub async fn delete_by_id(
        db: &DbConn,
        business_profile_address_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!(
            "Deleting BusinessProfileAddress for business_profile_address_id={}",
            business_profile_address_id
        );
        BusinessProfileAddressGateway::delete_by_id(db, business_profile_address_id)
            .await
            .map_err(|e| {
                log::error!("Error deleting BusinessProfileAddress: {:?}", e);
                BusinessError::new(format!("Error deleting BusinessProfileAddress: {:?}", e))
            })?;
        Ok(())
    }

    pub async fn delete_by_uuid(db: &DbConn, uuid: String) -> Result<(), BusinessError> {
        log::info!(
            "[BusinessProfileAddressUseCase::delete_by_uuid] Executing for uuid={}",
            uuid
        );
        BusinessProfileAddressGateway::delete_by_uuid(db, uuid.clone())
            .await
            .map_err(|error| {
                log::error!(
                    "[BusinessProfileAddressUseCase::delete_by_uuid] Failed for uuid={}: {}",
                    uuid,
                    error
                );
                BusinessError::infrastructure("Error deleting BusinessProfileAddress")
            })?;
        Ok(())
    }
}
