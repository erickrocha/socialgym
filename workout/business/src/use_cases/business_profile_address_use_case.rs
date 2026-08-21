use crate::commons::authorization::ensure_owns;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile_address::{
    BusinessProfileAddress, BusinessProfileAddressEntityMapper,
};
use crate::gateway::business_profile_address_gateway::BusinessProfileAddressGateway;
use crate::use_cases::business_profile_use_case::BusinessProfileUseCase;
use sea_orm::DbConn;

pub struct BusinessProfileAddressUseCase;

impl BusinessProfileAddressUseCase {
    pub async fn save(
        db: &DbConn,
        domain: BusinessProfileAddress,
        acting_person_id: i32,
    ) -> Result<BusinessProfileAddress, BusinessError> {
        log::info!(
            "Adding BusinessProfileAddress for business_profile_id={}",
            domain.business_profile_id
        );
        Self::ensure_owns_profile(db, domain.business_profile_id, acting_person_id).await?;
        if let Some(id) = domain.id {
            let existing = Self::find_by_id(db, id).await?;
            Self::ensure_owns_profile(db, existing.business_profile_id, acting_person_id).await?;
        }
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
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!(
            "Deleting BusinessProfileAddress for business_profile_address_id={}",
            business_profile_address_id
        );
        let existing = Self::find_by_id(db, business_profile_address_id).await?;
        Self::ensure_owns_profile(db, existing.business_profile_id, acting_person_id).await?;
        BusinessProfileAddressGateway::delete_by_id(db, business_profile_address_id)
            .await
            .map_err(|e| {
                log::error!("Error deleting BusinessProfileAddress: {:?}", e);
                BusinessError::new(format!("Error deleting BusinessProfileAddress: {:?}", e))
            })?;
        Ok(())
    }

    pub async fn delete_by_uuid(
        db: &DbConn,
        uuid: String,
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!(
            "[BusinessProfileAddressUseCase::delete_by_uuid] Executing for uuid={}",
            uuid
        );
        let existing = Self::find_by_uuid(db, uuid.clone()).await?;
        Self::ensure_owns_profile(db, existing.business_profile_id, acting_person_id).await?;
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

    /// Addresses inherit their authority from the profile they hang off: only
    /// the person who owns the business profile may touch them.
    async fn ensure_owns_profile(
        db: &DbConn,
        business_profile_id: i32,
        acting_person_id: i32,
    ) -> Result<(), BusinessError> {
        let profile = BusinessProfileUseCase::get_by_id(db, business_profile_id)
            .await
            .ok_or_else(|| BusinessError::not_found("Business profile not found"))?;
        ensure_owns(profile.owner_id, acting_person_id)
    }

    async fn find_by_id(db: &DbConn, id: i32) -> Result<BusinessProfileAddress, BusinessError> {
        BusinessProfileAddressGateway::find_by_id(db, id)
            .await
            .map_err(|error| {
                log::error!("[BusinessProfileAddressUseCase] Failed to load id={}: {}", id, error);
                BusinessError::infrastructure("Error loading BusinessProfileAddress")
            })?
            .map(BusinessProfileAddressEntityMapper::from_model)
            .ok_or_else(|| BusinessError::not_found("BusinessProfileAddress not found"))
    }

    async fn find_by_uuid(db: &DbConn, uuid: String) -> Result<BusinessProfileAddress, BusinessError> {
        BusinessProfileAddressGateway::find_by_uuid(db, uuid.clone())
            .await
            .map_err(|error| {
                log::error!("[BusinessProfileAddressUseCase] Failed to load uuid={}: {}", uuid, error);
                BusinessError::infrastructure("Error loading BusinessProfileAddress")
            })?
            .map(BusinessProfileAddressEntityMapper::from_model)
            .ok_or_else(|| BusinessError::not_found("BusinessProfileAddress not found"))
    }
}
