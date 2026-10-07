use crate::commons::authorization::ensure_owns;
use crate::commons::functions::{is_valid_coordinate, uuid_to_string};
use crate::domain::user::User;
use crate::commons::entity_mapper::EntityMapper;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::{BusinessProfile, BusinessProfileEntityMapper};
use crate::domain::business_profile_address::{
    BusinessProfileAddress, BusinessProfileAddressEntityMapper,
};
use crate::domain::enums::{ImageType, ProfileType};
use crate::domain::image_storage::ImageStorage;
use crate::gateway::business_profile_address_gateway::BusinessProfileAddressGateway;
use crate::gateway::business_profile_gateway::BusinessProfileGateway;
use crate::gateway::team_member_gateway::TeamMemberGateway;
use crate::use_cases::common_use_case::{handle_option};
use crate::use_cases::image_storage_use_case::ImageStorageUseCase;
use sea_orm::{DbConn, TransactionTrait};
use entity::business_profile_entity::BusinessProfileEntity;
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

    /// Combined "professional discovery" search: a name/social-name text
    /// query, a location filter (an explicit lat/long point + radius), a
    /// `business_type` filter (e.g. "Professional"), or any combination.
    /// Filters combine with AND; at least one of text/location must be
    /// supplied, or nothing is returned — this is a public marketplace
    /// search, not a listing, so an empty query never yields "everything".
    #[allow(clippy::too_many_arguments)]
    pub async fn discover(
        db: &DbConn,
        query: Option<String>,
        business_type: Option<ProfileType>,
        latitude: Option<f64>,
        longitude: Option<f64>,
        radius_km: Option<f64>,
        limit: i32,
    ) -> Result<Vec<BusinessProfile>, BusinessError> {
        let trimmed_query = query.as_deref().map(str::trim).filter(|q| !q.is_empty());
        let search_point = match (latitude, longitude) {
            (Some(lat), Some(lon)) if is_valid_coordinate(lat, lon) => Some((lat, lon)),
            _ => None,
        };

        if trimmed_query.is_none() && search_point.is_none() {
            return Ok(Vec::new());
        }

        let business_type_str = business_type.map(|t| t.to_string());
        let limit_u64 = if limit > 0 { limit as u64 } else { 50 };

        // With a location the text hits are intersected with the nearby profiles, so
        // they must not be cut to `limit` first (a profile matching both could be
        // dropped); the final `limit` is applied after the intersection.
        // ponytail: text matches are bounded at TEXT_SEARCH_CAP when combined with a location.
        const TEXT_SEARCH_CAP: u64 = 1000;
        let text_limit = if search_point.is_some() { TEXT_SEARCH_CAP } else { limit_u64 };
        let query_ids: Option<Vec<i32>> = if let Some(text) = trimmed_query {
            Some(
                BusinessProfileGateway::search_by_query(
                    db,
                    text,
                    business_type_str.as_deref(),
                    text_limit,
                )
                .await,
            )
        } else {
            None
        };

        let location_ids: Option<Vec<i32>> = if let Some((lat, lon)) = search_point {
            // The search radius is bounded so discovery cannot sweep the whole table.
            const MAX_RADIUS_KM: f64 = 500.0;
            let radius = radius_km
                .filter(|r| r.is_finite() && *r >= 0.0)
                .unwrap_or(200.0)
                .min(MAX_RADIUS_KM);
            let nearby = BusinessProfileAddressGateway::find_all_within_radius_of_point(
                db, lat, lon, radius,
            )
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Discovery search failed: {}", e)))?;
            let mut ids: Vec<i32> = nearby
                .into_iter()
                .map(|address| address.business_profile_id)
                .collect();
            if let Some(business_type_str) = business_type_str.as_deref() {
                let allowed =
                    BusinessProfileGateway::find_all_ids_by_business_type(db, business_type_str)
                        .await;
                ids.retain(|id| allowed.contains(id));
            }
            ids.sort();
            ids.dedup();
            Some(ids)
        } else {
            None
        };

        // Combine: both filters supplied -> intersect (AND); only one supplied -> use it as-is.
        let mut candidate_ids = match (query_ids, location_ids) {
            (Some(query_ids), Some(location_ids)) => query_ids
                .into_iter()
                .filter(|id| location_ids.contains(id))
                .collect::<Vec<i32>>(),
            (Some(query_ids), None) => query_ids,
            (None, Some(location_ids)) => location_ids,
            (None, None) => Vec::new(),
        };

        let limit = if limit > 100 {
            100
        } else if limit < 1 {
            50
        } else {
            limit
        };
        candidate_ids.truncate(limit as usize);

        if candidate_ids.is_empty() {
            return Ok(Vec::new());
        }

        let business_profiles = BusinessProfileGateway::find_all_by_ids(db, candidate_ids).await;
        let result = Self::fill_business_profiles(db, business_profiles).await;
        Ok(result)
    }

    async fn fill_business_profiles(db: &DbConn, business_profiles: Vec<BusinessProfileEntity>) -> Vec<BusinessProfile> {
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
                log::info!("Pre-signed URL generated for business_profile_id={}", id);
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
                    // `from_model` keeps the stored logo key in `object_key` and leaves
                    // `logo` empty; carry it over so a cover upload does not clear the logo.
                    business_profile.logo = business_profile.object_key.clone();
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

    /// Create a business profile owned by `actor`. A client-supplied owner id is
    /// ignored: ownership is always the authenticated person.
    pub async fn add(
        db: &DbConn,
        mut domain: BusinessProfile,
        actor: &User,
    ) -> Result<BusinessProfile, BusinessError> {
        log::info!("Adding new business profile for owner_id: {}", actor.person_id);
        domain.owner_id = actor.person_id;
        domain.owner_uuid = actor.person_uuid.clone();
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

    pub async fn update(
        db: &DbConn,
        mut domain: BusinessProfile,
        actor: &User,
    ) -> Result<BusinessProfile, BusinessError> {
        log::info!("Updating business profile for owner_id: {:?}", domain.owner_id);
        let id = domain
            .id
            .ok_or_else(|| BusinessError::validation("Business profile id is required"))?;
        let existing = Self::get_by_id(db, id)
            .await
            .ok_or_else(|| BusinessError::not_found("Business profile not found"))?;
        ensure_owns(existing.owner_id, actor.person_id)?;
        domain.owner_id = existing.owner_id;
        domain.owner_uuid = existing.owner_uuid.clone();
        // The uuid and the stored image keys are not editable here: reads return signed
        // image URLs, and writing a client payload back would replace the keys with them.
        let stored = BusinessProfileGateway::find_by_id(db, id)
            .await
            .ok_or_else(|| BusinessError::not_found("Business profile not found"))?;
        domain.uuid = Some(uuid_to_string(stored.uuid));
        domain.logo = stored.logo;
        domain.cover_image = stored.cover_image;
        let updated_profile = BusinessProfileGateway::persist(db, domain)
            .await
            .map_err(|e| {
                log::error!("Error updating business profile: {:?}", e);
                BusinessError::new(format!("Error updating business profile: {:?}", e))
            })?;
        let entity = BusinessProfileEntityMapper::from_active_model(updated_profile);
        Ok(entity)
    }

    /// Permanently deletes a business profile owned by `actor`, cascading its
    /// addresses, team memberships, and profile mapping in a single
    /// transaction. Object storage is best-effort: a failed image cleanup is
    /// logged, not fatal, since orphaned S3 objects are recoverable and
    /// blocking deletion on them would strand the user's request.
    pub async fn delete(db: &DbConn, id: i32, actor: &User) -> Result<(), BusinessError> {
        log::info!("Deleting business profile id: {:?}", id);
        let existing = BusinessProfileGateway::find_by_id(db, id)
            .await
            .ok_or_else(|| BusinessError::not_found("Business profile not found"))?;
        let existing = BusinessProfileEntityMapper::from_model(existing);
        ensure_owns(existing.owner_id, actor.person_id)?;

        for object_key in [existing.object_key.clone(), existing.cover_image.clone()]
            .into_iter()
            .flatten()
        {
            if let Err(e) = ImageStorageUseCase::delete_presigned_url(object_key.clone()).await {
                log::error!(
                    "Error deleting object {} for business profile {}: {:?}",
                    object_key,
                    id,
                    e
                );
            }
        }

        let txn = db.begin().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to start delete transaction: {}", e))
        })?;
        let map_err = |e: sea_orm::DbErr| {
            BusinessError::infrastructure(format!("Delete cascade failed: {}", e))
        };

        BusinessProfileAddressGateway::delete_all_by_business_profile_id(&txn, id)
            .await
            .map_err(map_err)?;
        TeamMemberGateway::delete_all_by_business_profile_id(&txn, id)
            .await
            .map_err(map_err)?;
        ProfileGateway::delete_all_by_business_profile_id(&txn, id)
            .await
            .map_err(map_err)?;
        BusinessProfileGateway::delete_by_id(&txn, id)
            .await
            .map_err(map_err)?;

        txn.commit().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to commit delete transaction: {}", e))
        })
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
