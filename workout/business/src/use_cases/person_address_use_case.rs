use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::person_address::{PersonAddress, PersonAddressEntityMapper};
use crate::gateway::person_address_gateway::PersonAddressGateway;
use sea_orm::DbConn;

pub struct PersonAddressUseCase {}

impl PersonAddressUseCase {
    pub async fn add_person_address(
        db: &DbConn,
        mut address: PersonAddress,
    ) -> Result<PersonAddress, BusinessError> {
        log::info!("Adding person address: {:?}", address);

        let update_result =
            PersonAddressGateway::set_all_addresses_not_current(db, address.person_id).await;
        if let Err(error) = update_result {
            log::error!(
                "Error updating existing addresses to non-current: {:?}",
                error.to_string()
            );
            return Err(BusinessError::new(error.to_string()));
        }

        address.current = true;

        let address_result = PersonAddressGateway::persist(db, address).await;
        match address_result {
            Ok(persisted) => {
                log::info!("Person address saved successfully");
                Ok(PersonAddressEntityMapper::from_active_model(persisted))
            }
            Err(error) => {
                log::error!("Error saving person address: {:?}", error.to_string());
                Err(BusinessError::new(error.to_string()))
            }
        }
    }

    pub async fn delete_person_address(
        db: &DbConn,
        person_address_id: i32,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting person address with id: {:?}", person_address_id);
        let delete_result = PersonAddressGateway::delete(db, person_address_id).await;
        match delete_result {
            Ok(_) => {
                log::info!("Person address deleted successfully");
                Ok(())
            }
            Err(error) => {
                log::error!("Error deleting person address: {:?}", error.to_string());
                Err(BusinessError::new(error.to_string()))
            }
        }
    }

    pub async fn update_person_address(
        db: &DbConn,
        address: PersonAddress,
    ) -> Result<PersonAddress, BusinessError> {
        log::info!("Updating person address: {:?}", address);

        let update_result = PersonAddressGateway::persist(db, address).await;
        match update_result {
            Ok(updated) => {
                log::info!("Person address updated successfully");
                Ok(PersonAddressEntityMapper::from_active_model(updated))
            }
            Err(error) => {
                log::error!("Error updating person address: {:?}", error.to_string());
                Err(BusinessError::new(error.to_string()))
            }
        }
    }
}
