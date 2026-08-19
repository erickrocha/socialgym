use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::{BusinessProfile, BusinessProfileEntityMapper};
use crate::domain::business_profile_address::{
    BusinessProfileAddress, BusinessProfileAddressEntityMapper,
};
use crate::domain::enums::ImageType;
use crate::domain::image_storage::ImageStorage;
use crate::gateway::business_profile_address_gateway::BusinessProfileAddressGateway;
use crate::gateway::business_profile_gateway::BusinessProfileGateway;
use crate::use_cases::common_use_case::{handle_option};
use crate::use_cases::image_storage_use_case::ImageStorageUseCase;
use sea_orm::{DbConn};
use entity::business_profile::Model;
use crate::domain::profile::Profile;
use crate::gateway::profile_gateway::ProfileGateway;

pub struct BusinessProfileUseCase {}

impl BusinessProfileUseCase {

    pub async fn get_by_id(db: &DbConn, id: i32) -> Option<BusinessProfile> {
        log::info!("Getting business profile with id: {:?}", id);

        let business_profile = BusinessProfileGateway::find_by_id(db, id).await;

        match business_profile {
            Some(business_profile) => {
                let mut result = BusinessProfileEntityMapper::from_model(business_profile);
                Self::fill_images(&mut result).await;
                let addresses = Self::load_addresses(db, result.id.unwrap()).await;
                result.addresses = addresses;
                Some(result)
            }
            None => None,
        }
    }

    pub async fn get_by_uuid(db: &DbConn, uuid: String) -> Option<BusinessProfile> {
        log::info!("Getting business profile with uuid: {:?}", uuid);

        let business_profile = BusinessProfileGateway::find_by_uuid(db, uuid.as_str()).await;

        match business_profile {
            Ok(Some(business_profile)) => {
                let mut result = BusinessProfileEntityMapper::from_model(business_profile);
                Self::fill_images(&mut result).await;
                let addresses = Self::load_addresses(db, result.id.unwrap()).await;
                result.addresses = addresses;
                Some(result)
            }
            Ok(None) => None,
            Err(error) => {
                log::error!("Error getting business profile by uuid: {}", error);
                None
            }
        }
    }

    pub async fn get_by_owner_id(db: &DbConn,owner_id: i32) -> Result<Vec<BusinessProfile>, BusinessError> {
        log::info!("Getting business profiles for owner id: {:?}", owner_id);

        let business_profiles = BusinessProfileGateway::find_by_owner_id(db, owner_id).await;

        let result = Self::fill_business_profiles(db, business_profiles).await;

        log::info!("Successfully retrieved {} business profiles for owner id: {:?}",result.len(),owner_id);
        Ok(result)
    }

    pub async fn get_by_owner_uuid(db: &DbConn, uuid: String) -> Result<Vec<BusinessProfile>, BusinessError> {
        log::info!("Getting business profiles for owner uuid: {:?}", uuid);

        let business_profiles = BusinessProfileGateway::find_by_owner_uuid(db, uuid.as_str())
            .await
            .map_err(|e| BusinessError::new(e.to_string()))?;

        let result = Self::fill_business_profiles(db, business_profiles).await;


        log::info!("Successfully retrieved {} business profiles for owner uuid: {:?}",result.len(),uuid);
        Ok(result)
    }

    async fn fill_business_profiles(db: &DbConn, business_profiles: Vec<Model>) -> Vec<BusinessProfile> {
        let mut result = Vec::new();

        for profile in business_profiles {
            let mut business_profile = BusinessProfileEntityMapper::from_model(profile.clone());
            business_profile.addresses = Self::load_addresses(db, profile.id).await;
            Self::fill_images(&mut business_profile).await;
            result.push(business_profile);
        }
        result
    }

    async fn load_addresses(db: &DbConn, id: i32) -> Vec<BusinessProfileAddress> {
        log::info!("Loading addresses for business profile id: {:?}",id);
        let result = BusinessProfileAddressGateway::find_all_by_business_profile_id(db, id).await;
        if result.is_err() {
            log::error!("Error loading addresses for business profile {}: {:?}", id, result.err());
            return Vec::new();
        }
        let addresses = result.unwrap();
        BusinessProfileAddressEntityMapper::from_models(addresses)
    }

    pub async fn upload_business_profile_image(db: &DbConn, id: i32,uuid: String,image_type: ImageType,format: String) -> Result<ImageStorage, BusinessError> {
        log::info!("Uploading image for business profile: {:?}",id);
        let business_profile_result = BusinessProfileGateway::find_by_id(db, id).await;
        let business_profile_model = handle_option(business_profile_result, "Business profile not found")?;
        let optional_object_key = match image_type {
            ImageType::Avatar => business_profile_model.logo,
            ImageType::Cover => business_profile_model.cover_image,
        };
        let s3_result = match optional_object_key {
            Some(key) => ImageStorageUseCase::update_presigned_url(id, key, format).await,
            None => ImageStorageUseCase::generate_presigned_url("business_profile".to_string(),id,uuid.as_str(),image_type.to_string().as_str(),format.as_str()).await
        };

        match s3_result {
            Ok(image_storage) => {
                log::info!("Pre-signed URL generated successfully: {:?}",image_storage.url);
                let business_profile_entity = BusinessProfileGateway::find_by_id(db, id).await;
                if business_profile_entity.is_none() {
                    log::error!("Business profile with id {} not found", id);
                    return Err(BusinessError::new("Business profile not found".to_string()));
                }

                let business_profile_option = business_profile_entity.unwrap();
                let mut business_profile = BusinessProfileEntityMapper::from_model(business_profile_option);
                if image_type == ImageType::Avatar {
                    business_profile.logo = Some(image_storage.object_key.clone());
                } else {
                    business_profile.cover_image = Some(image_storage.object_key.clone());
                }

                let update_result = BusinessProfileGateway::persist(db, business_profile).await;
                if update_result.is_err() {
                    log::error!("Error updating business profile with id {}: {:?}", id, update_result.err());
                    return Err(BusinessError::new("Failed to update business profile with image key".to_string()));
                }
                Ok(image_storage)
            }
            Err(e) => {
                log::error!("Error generating pre-signed URL: {:?}", e);
                Err(BusinessError::new(format!("Error generating pre-signed URL: {:?}", e)))
            }
        }
    }

    pub async fn add(db: &DbConn, domain: BusinessProfile) -> Result<BusinessProfile, BusinessError> {
        log::info!("Adding new business profile for owner_id: {:?}", domain.owner_id);
        let added_profile = BusinessProfileGateway::persist(db, domain)
            .await
            .map_err(|e| {
                log::error!("Error adding business profile: {:?}", e);
                BusinessError::new(format!("Error adding business profile: {:?}", e))
            })?;
        let entity = BusinessProfileEntityMapper::from_active_model(added_profile);
        let profile = Profile::new(entity.owner_id,entity.owner_uuid.clone(),entity.id.unwrap(),entity.uuid.clone().unwrap());
        ProfileGateway::persist(db, profile).await.map_err(|e| {
            log::error!("Error adding profile: {:?}", e);
            BusinessError::new(format!("Error adding profile: {:?}", e))
        })?;
        Ok(entity)
    }

    pub async fn update(db: &DbConn, domain: BusinessProfile) -> Result<BusinessProfile, BusinessError> {
        log::info!("Updating business profile for owner_id: {:?}", domain.owner_id);
        let updated_profile = BusinessProfileGateway::persist(db, domain)
            .await
            .map_err(|e| {
                log::error!("Error updating business profile: {:?}", e);
                BusinessError::new(format!("Error updating business profile: {:?}", e))
            })?;
        let entity = BusinessProfileEntityMapper::from_active_model(updated_profile);
        Ok(entity)
    }
    async fn fill_images(business_profile: &mut BusinessProfile) {
        if business_profile.cover_image.is_some() {
            log::info!("Business profile has cover image, generating pre-signed URL");
            let object_key = business_profile.cover_image.clone().unwrap();
            let cover_url_result = ImageStorageUseCase::generate_cloud_front_signed_url(object_key.as_str()).await;
            match cover_url_result {
                Ok(cover_url) => business_profile.cover_image = Some(cover_url),
                Err(e) => log::warn!("Error generating pre-signed URL for cover image: {:?}", e),
            }
        }
        if business_profile.object_key.is_some() {
            log::info!("Business profile has avatar image, generating pre-signed URL");
            let object_key = business_profile.object_key.clone().unwrap();
            let logo_result = ImageStorageUseCase::generate_cloud_front_signed_url(object_key.as_str()).await;
            match logo_result {
                Ok(avatar_url) => business_profile.logo = Some(avatar_url),
                Err(e) => log::warn!("Error generating pre-signed URL for avatar image: {:?}", e),
            }
        }
    }
}
