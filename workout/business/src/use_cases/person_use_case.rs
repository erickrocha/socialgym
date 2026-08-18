use crate::commons::entity_mapper::EntityMapper;
use crate::commons::functions::string_to_uuid;
use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::{BusinessProfile, BusinessProfileEntityMapper};
use crate::domain::enums::ImageType;
use crate::domain::friend::{Friend, FriendEntityMapper, FriendStatus};
use crate::domain::image_storage::ImageStorage;
use crate::domain::person::{Person, PersonEntityMapper};
use crate::domain::person_address::{PersonAddress, PersonAddressEntityMapper};
use crate::domain::person_info::{PersonInfo, PersonInfoEntityMapper};
use crate::gateway::business_profile_gateway::BusinessProfileGateway;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::person_address_gateway::PersonAddressGateway;
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::person_info_gateway::PersonInfoGateway;
use crate::gateway::profile_gateway::ProfileGateway;
use crate::use_cases::common_use_case::{handle_option, handle_result};
use crate::use_cases::image_storage_use_case::ImageStorageUseCase;
use crate::use_cases::person_media_use_case::PersonMediaUseCase;
use entity;
use sea_orm::DbConn;

pub struct PersonUseCase {}

impl PersonUseCase {
    pub async fn get(db: &DbConn, person_id: i32) -> Result<Person, BusinessError> {
        log::info!("Getting person with id: {:?}", person_id);
        let person_entity = PersonGateway::find_by_id(db, person_id).await;
        let person_model = handle_option(person_entity, "Person not found")?;

        let person_info_entity = PersonInfoGateway::find_by_person_id(db, person_model.id).await;

        if person_info_entity.is_err() {
            log::info!("Person info not found for person id: {:?}", person_model.id);
            return Err(BusinessError::new("Person info not found".to_string()));
        }
        let person_info_entity = person_info_entity.unwrap();

        let mut person_domain = PersonEntityMapper::from_model(person_model);
        person_domain.person_info = person_info_entity.map(PersonInfoEntityMapper::from_model);

        if let Some(id) = person_domain.id {
            let addresses = Self::load_addresses(db, id).await;
            person_domain.addresses = addresses;

            let business_profiles = Self::load_business_profiles(db, id).await;
            person_domain.business_profiles = business_profiles;
        }

        Self::fill_images(&mut person_domain).await;

        Ok(person_domain)
    }

    async fn fill_images(person_domain: &mut Person) {
        if person_domain.cover.is_some() {
            log::info!("Person has cover image, generating pre-signed URL");
            let object_key = person_domain.cover.clone().unwrap();
            let cover_url_result =
                ImageStorageUseCase::generate_cloud_front_signed_url(object_key.as_str()).await;
            match cover_url_result {
                Ok(cover_url) => person_domain.cover = Some(cover_url),
                Err(e) => log::warn!("Error generating pre-signed URL for cover image: {:?}", e),
            }
        }
        if person_domain.object_key.is_some() {
            log::info!("Person has avatar image, generating pre-signed URL");
            let object_key = person_domain.object_key.clone().unwrap();
            let avatar_url_result =
                ImageStorageUseCase::generate_cloud_front_signed_url(object_key.as_str()).await;
            match avatar_url_result {
                Ok(avatar_url) => person_domain.avatar = Some(avatar_url),
                Err(e) => log::warn!("Error generating pre-signed URL for avatar image: {:?}", e),
            }
        }
    }

    pub async fn update(db: &DbConn, person: Person) -> Result<Person, BusinessError> {
        log::info!("Updating person: {:?}", person);
        let person_id = person.id;

        let person_info = person.person_info.clone().ok_or_else(|| {
            log::info!("Person info does not exist");
            BusinessError::new("Person info is required".to_string())
        })?;

        log::info!("Person info to update: {:?}", person_info);
        let person_info_persisted = Self::persist_person_info(db, person_info).await?;

        let person_result = PersonGateway::persist(db, person).await;
        let persisted = handle_result(person_result, "Error persisting person")?;

        let mut response_person = PersonEntityMapper::from_active_model(persisted);
        response_person.person_info = Some(PersonInfoEntityMapper::from_active_model(
            person_info_persisted,
        ));
        if let Some(person_id) = person_id {
            let addresses = Self::load_addresses(db, person_id).await;
            response_person.addresses = addresses;
        }

        log::info!("Person updated successfully");
        Ok(response_person)
    }

    pub async fn add(db: &DbConn, person: Person) -> Result<Person, BusinessError> {
        log::info!("Adding person: {:?}", person);

        Self::validate_person(&person)?;

        log::info!("User is valid, adding to database");
        let person_result = PersonGateway::persist(db, person.clone()).await;
        let persisted = handle_result(person_result, "Error persisting person")?;

        let mut response_person = PersonEntityMapper::from_active_model(persisted);

        log::info!("Creating empty person info for the new person");
        let person_info = PersonInfo::new(
            response_person.id.unwrap(),
            None,
            None,
            None,
            None,
            None,
            None,
            None,
        );

        log::info!("Person info exists, proceeding to create");
        let person_info_persisted = Self::persist_person_info(db, person_info).await;

        let person_info_persisted = match person_info_persisted {
            Ok(info) => info,
            Err(error) => {
                log::info!("Error creating person info: {:?}", error.to_string());
                return Err(BusinessError::new(error.to_string()));
            }
        };
        response_person.person_info = Some(PersonInfoEntityMapper::from_active_model(
            person_info_persisted,
        ));

        log::info!("Person added successfully");
        Ok(response_person)
    }

    // Helper methods
    fn validate_person(person: &Person) -> Result<(), BusinessError> {
        if person.firstname.is_empty() || person.surname.is_empty() || person.gender.is_empty() {
            log::info!("Person and User information are required");
            return Err(BusinessError::new(
                "Person and User information are required".to_string(),
            ));
        }
        Ok(())
    }

    async fn persist_person_info(
        db: &DbConn,
        person_info: PersonInfo,
    ) -> Result<entity::person_info::ActiveModel, BusinessError> {
        let person_info_result = PersonInfoGateway::persist(db, person_info).await;
        match person_info_result {
            Ok(persisted) => {
                log::info!("Person info saved successfully: {:?}", persisted);
                Ok(persisted)
            }
            Err(error) => {
                log::info!("Error saving person info: {:?}", error.to_string());
                Err(BusinessError::new(error.to_string()))
            }
        }
    }

    async fn load_addresses(db: &DbConn, person_id: i32) -> Vec<PersonAddress> {
        let entities = PersonAddressGateway::find_all_by_person_id(db, person_id).await;
        PersonAddressEntityMapper::from_models(entities)
    }

    async fn load_business_profiles(db: &DbConn, owner_id: i32) -> Vec<BusinessProfile> {
        let entities = ProfileGateway::find_by_person_id(db, owner_id).await;
        let business_profile_ids: Vec<i32> =
            entities.iter().map(|p| p.business_profile_id).collect();
        let business_profile_models =
            BusinessProfileGateway::find_all_by_ids(db, business_profile_ids).await;
        BusinessProfileEntityMapper::from_models(business_profile_models)
    }

    pub async fn get_suggestions(db: &DbConn, person_id: i32, radius_km: f64) -> Vec<Person> {
        log::info!(
            "Attempting to find friend suggestions for person_id: {:?}",
            person_id
        );
        // Get the current person's address
        let current_address = PersonAddressGateway::get_current_address(db, person_id, true).await;
        if current_address.is_err() {
            log::info!("Person {:?} don't have current address", person_id);
            return Vec::new();
        }

        let current_address = current_address.unwrap();

        if current_address.is_none() {
            log::info!("Error getting current address for person {}", person_id);
            return Vec::new();
        }

        // Get all person IDs that have any relationship with the current person
        // This includes: pending requests (sent/received), accepted friends, rejected and canceled
        let excluded_ids = FriendGateway::find_all_related_person_ids(db, person_id)
            .await
            .unwrap_or_else(|_| Vec::new());

        let current_address = current_address.unwrap();
        let (Some(person_lat), Some(person_lon)) =
            (current_address.latitude, current_address.longitude)
        else {
            return Vec::new();
        };
        if !radius_km.is_finite() || radius_km < 0.0 {
            return Vec::new();
        }
        let nearby_addresses =
            PersonAddressGateway::find_all_within_radius(db, person_lat, person_lon, radius_km)
                .await;
        let neighbor_ids = Self::extract_neighbor_ids(
            nearby_addresses.unwrap_or_else(|_| Vec::new()),
            person_id,
            &excluded_ids,
        );
        let persons = PersonGateway::find_all_by_id_in(db, neighbor_ids).await;
        PersonEntityMapper::from_models(persons)
            .into_iter()
            .filter(|person| person.id.unwrap_or_default() != person_id)
            .collect()
    }

    fn extract_neighbor_ids(
        nearby_addresses: Vec<entity::person_address::Model>,
        person_id: i32,
        excluded_ids: &[i32],
    ) -> Vec<i32> {
        let mut neighbor_ids: Vec<i32> = nearby_addresses
            .into_iter()
            .filter(|address| address.person_id != person_id)
            .filter(|address| !excluded_ids.contains(&address.person_id))
            .map(|address| address.person_id)
            .collect();

        neighbor_ids.sort();
        neighbor_ids.dedup();
        neighbor_ids
    }

    async fn get_friends_by_column_and_status(
        db: &DbConn,
        column: entity::friends::Column,
        person_id: i32,
        status: FriendStatus,
    ) -> Vec<Person> {
        log::info!(
            "Attempting to find all friends for person_id: {:?}",
            person_id
        );
        let friends =
            FriendGateway::find_all_by_column_and_status(db, column, person_id, status).await;
        if friends.is_err() {
            log::info!("Person id {:?} no have friends", person_id);
            return Vec::new();
        }
        let friends = friends.unwrap();
        let friend_ids: Vec<i32> = match column {
            entity::friends::Column::PersonId => friends.into_iter().map(|f| f.friend_id).collect(),
            entity::friends::Column::FriendId => friends.into_iter().map(|f| f.person_id).collect(),
            _ => Vec::new(),
        };
        let persons = PersonGateway::find_all_by_id_in(db, friend_ids.clone()).await;
        PersonEntityMapper::from_models(persons)
    }

    pub async fn get_all_friends(db: &DbConn, person_id: i32) -> Vec<Person> {
        log::info!(
            "Attempting to find all friends for person_id: {:?}",
            person_id
        );
        // Get all accepted friend relationships where the person is involved (either direction)
        let friends = FriendGateway::find_all_accepted_friends(db, person_id).await;
        if friends.is_err() {
            log::info!("Person id {:?} has no friends", person_id);
            return Vec::new();
        }
        let friends = friends.unwrap();

        // Extract friend IDs - if person_id is the sender, get receiver; if person_id is receiver, get sender
        let friend_ids: Vec<i32> = friends
            .into_iter()
            .map(|f| {
                if f.person_id == person_id {
                    f.friend_id
                } else {
                    f.person_id
                }
            })
            .collect();

        let persons = PersonGateway::find_all_by_id_in(db, friend_ids.clone()).await;
        PersonEntityMapper::from_models(persons)
    }

    pub async fn get_all_received_requests(db: &DbConn, person_id: i32) -> Vec<Person> {
        Self::get_friends_by_column_and_status(
            db,
            entity::friends::Column::FriendId,
            person_id,
            FriendStatus::Pending,
        )
        .await
    }

    pub async fn get_all_sent_requests(db: &DbConn, person_id: i32) -> Vec<Person> {
        Self::get_friends_by_column_and_status(
            db,
            entity::friends::Column::PersonId,
            person_id,
            FriendStatus::Pending,
        )
        .await
    }

    pub async fn upload_person_image(
        db: &DbConn,
        person_id: i32,
        image_type: ImageType,
        format: String,
    ) -> Result<ImageStorage, BusinessError> {
        log::info!("Uploading person image for person_id: {:?}", person_id);

        // Determine the album name from the image type
        let album = match image_type {
            ImageType::Avatar => "avatar".to_string(),
            ImageType::Cover => "cover".to_string(),
        };

        // Use PersonMediaUseCase to generate the URL and persist the tracking record.
        // This also builds the S3 key as {person_uuid}/{album}/{new-uuid}.
        let image_storage =
            PersonMediaUseCase::generate_upload_url(db, person_id, album, format).await?;

        Ok(image_storage)
    }

    pub async fn delete_person_image(
        db: &DbConn,
        person_id: i32,
        image_type: ImageType,
    ) -> Result<(), BusinessError> {
        log::info!("Deleting person image for person_id: {:?}", person_id);
        let user_result = PersonGateway::find_by_id(db, person_id).await;
        let model = handle_option(user_result, "Person not found")?;
        let optional_object_key = match image_type {
            ImageType::Avatar => model.avatar.clone(),
            ImageType::Cover => model.cover_image.clone(),
        };
        if let Some(object_key) = optional_object_key {
            // Remove reference from person
            let mut person = PersonEntityMapper::from_model(model);
            match image_type {
                ImageType::Avatar => person.avatar = None,
                ImageType::Cover => person.cover = None,
            }
            let update_result = PersonGateway::persist(db, person).await;
            if update_result.is_err() {
                log::error!(
                    "Error updating person after image delete: {:?}",
                    update_result.err()
                );
                return Err(BusinessError::new(
                    "Failed to update person after image delete".to_string(),
                ));
            }
            // Delete from S3
            if let Err(e) = ImageStorageUseCase::delete_presigned_url(object_key.clone()).await {
                log::error!("Error deleting object from S3: {:?}", e);
            }
        }
        Ok(())
    }

    pub async fn search_persons(
        db: &DbConn,
        query: &str,
        current_user_person_id: i32,
        limit: i32,
    ) -> Vec<Person> {
        log::info!("Searching persons with query: {}", query);

        if query.trim().is_empty() {
            return Vec::new();
        }

        let limit_u64 = if limit > 0 { limit as u64 } else { 50 };

        // Search by person name (first_name, surname)
        let mut person_ids =
            PersonGateway::search_by_query(db, query, current_user_person_id, limit_u64).await;

        // Search by user email/name and get additional person_ids
        let email_person_ids =
            PersonGateway::search_by_query_with_email(db, query, current_user_person_id, limit_u64)
                .await;
        person_ids.extend(email_person_ids);

        // Remove duplicates
        person_ids.sort();
        person_ids.dedup();

        // Limit to the requested limit
        person_ids.truncate(limit as usize);

        // Fetch all persons by IDs
        let models = PersonGateway::find_all_by_id_in(db, person_ids).await;
        PersonEntityMapper::from_models(models)
    }

    pub async fn search_persons_by_uuid(
        db: &DbConn,
        query: &str,
        current_user_person_uuid: String,
        limit: i32,
    ) -> Vec<Person> {
        log::info!("Searching persons with query: {}", query);

        if query.trim().is_empty() {
            return Vec::new();
        }

        let limit_u64 = if limit > 0 { limit as u64 } else { 50 };

        // Search by person name (first_name, surname)
        let mut person_uuids = PersonGateway::search_by_query_uuid(
            db,
            query,
            string_to_uuid(&current_user_person_uuid),
            limit_u64,
        )
        .await;

        // Search by user email/name and get additional person_ids
        let email_person_ids = PersonGateway::search_by_query_with_email_uuid(
            db,
            query,
            string_to_uuid(&current_user_person_uuid),
            limit_u64,
        )
        .await;
        person_uuids.extend(email_person_ids);

        // Remove duplicates
        person_uuids.sort();
        person_uuids.dedup();

        // Limit to the requested limit
        person_uuids.truncate(limit as usize);

        // Fetch all persons by IDs
        let models = PersonGateway::find_all_by_uuid_in(db, person_uuids).await;
        PersonEntityMapper::from_models(models)
    }

    pub async fn search_mentionable_friends(
        db: &DbConn,
        person_id: i32,
        query: &str,
        limit: i32,
    ) -> Vec<Person> {
        let trimmed_query = query.trim();
        if person_id <= 0 || trimmed_query.is_empty() {
            return Vec::new();
        }

        let accepted_friendships =
            match FriendGateway::find_all_accepted_friends(db, person_id).await {
                Ok(records) => records,
                Err(e) => {
                    log::error!("Error loading accepted friends for mention search: {:?}", e);
                    return Vec::new();
                }
            };
        let accepted_friendships = FriendEntityMapper::from_models(accepted_friendships);
        let friend_ids = Self::extract_accepted_friend_ids(accepted_friendships, person_id);
        if friend_ids.is_empty() {
            return Vec::new();
        }

        let capped_limit = if limit > 0 { limit.min(50) } else { 10 };
        let limit_u64 = capped_limit as u64;

        let mut person_ids =
            PersonGateway::search_friends_by_name(db, trimmed_query, friend_ids.clone(), limit_u64)
                .await;

        let ids = PersonGateway::search_friend_ids_by_username_or_email(
            db,
            trimmed_query,
            friend_ids,
            limit_u64,
        )
        .await;
        person_ids.extend(ids);

        person_ids.sort_unstable();
        person_ids.dedup();
        person_ids.retain(|id| *id != person_id);
        person_ids.truncate(capped_limit as usize);

        if person_ids.is_empty() {
            return Vec::new();
        }

        let models = PersonGateway::find_all_by_id_in(db, person_ids).await;
        PersonEntityMapper::from_models(models)
    }

    fn extract_accepted_friend_ids(friendships: Vec<Friend>, person_id: i32) -> Vec<i32> {
        let mut friend_ids: Vec<i32> = friendships
            .into_iter()
            .filter_map(|f| {
                if f.person_id == person_id {
                    Some(f.friend_id)
                } else if f.friend_id == person_id {
                    Some(f.person_id)
                } else {
                    None
                }
            })
            .filter(|id| *id != person_id)
            .collect();

        friend_ids.sort_unstable();
        friend_ids.dedup();
        friend_ids
    }

    pub async fn find_by_uuid(db: &DbConn, uuid: String) -> Result<Person, BusinessError> {
        log::info!("Finding person by UUID: {:?}", uuid);
        let person_entity = PersonGateway::find_by_uuid(db, uuid.as_str()).await;
        let person_model = handle_option(person_entity, "Person not found")?;

        let person_info_entity = PersonInfoGateway::find_by_person_id(db, person_model.id).await;

        if person_info_entity.is_err() {
            log::info!("Person info not found for person id: {:?}", person_model.id);
            return Err(BusinessError::new("Person info not found".to_string()));
        }
        let person_info_entity = person_info_entity.unwrap();

        let mut person_domain = PersonEntityMapper::from_model(person_model);
        person_domain.person_info = person_info_entity.map(PersonInfoEntityMapper::from_model);
        if let Some(id) = person_domain.id {
            let addresses = Self::load_addresses(db, id).await;
            person_domain.addresses = addresses;

            // Load profiles if exist
            let business_profiles = Self::load_business_profiles(db, id).await;
            person_domain.business_profiles = business_profiles;
        }

        Self::fill_images(&mut person_domain).await;

        Ok(person_domain)
    }
}

#[cfg(test)]
mod tests {
    use super::PersonUseCase;
    use crate::commons::functions::string_to_uuid;
    use crate::domain::friend::{Friend, FriendStatus};
    use chrono::Utc;
    use entity::{friends, person_address};
    use serde::de::Unexpected::Option;

    fn make_address(id: i32, person_id: i32) -> person_address::Model {
        person_address::Model {
            id,
            uuid: string_to_uuid(format!("address-{id}").as_str()),
            person_id,
            address_line1: "".to_string(),
            address_line2: None,
            locality: "".to_string(),
            administrative_area: "Florianópolis".to_string(),
            postal_code: Some("88058573".to_string()),
            country_code: "BR".to_string(),
            current: true,
            latitude: Some(0.0),
            longitude: Some(0.0),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        }
    }

    fn make_friendship(id: i32, person_id: i32, friend_id: i32) -> Friend {
        Friend {
            id: Some(id),
            uuid: Some(format!("friendship-{id}").as_str().to_string()),
            person_id,
            friend_id,
            created_at: Some(Utc::now().naive_utc()),
            updated_at: Some(Utc::now().naive_utc()),
            status: FriendStatus::Accepted,
            person_uuid: format!("person-{person_id}"),
            friend_uuid: format!("person-{friend_id}"),
        }
    }

    #[test]
    fn extract_neighbor_ids_excludes_current_person_by_person_id() {
        let me = 1;
        let addresses = vec![make_address(99, 1), make_address(100, 2)];

        let ids = PersonUseCase::extract_neighbor_ids(addresses, me, &[]);
        assert_eq!(ids, vec![2]);
    }

    #[test]
    fn extract_neighbor_ids_excludes_related_people() {
        let addresses = vec![make_address(1, 2), make_address(2, 3), make_address(3, 4)];

        let ids = PersonUseCase::extract_neighbor_ids(addresses, 1, &[3]);
        assert_eq!(ids, vec![2, 4]);
    }

    #[test]
    fn extract_neighbor_ids_dedups_multiple_addresses_same_person() {
        let addresses = vec![make_address(1, 2), make_address(2, 2), make_address(3, 3)];

        let ids = PersonUseCase::extract_neighbor_ids(addresses, 1, &[]);
        assert_eq!(ids, vec![2, 3]);
    }

    #[test]
    fn extract_accepted_friend_ids_keeps_both_directions_and_dedups() {
        let me = 1;
        let friendships = vec![
            make_friendship(1, 1, 2),
            make_friendship(2, 3, 1),
            make_friendship(3, 1, 2),
            make_friendship(4, 1, 1),
        ];

        let ids = PersonUseCase::extract_accepted_friend_ids(friendships, me);
        assert_eq!(ids, vec![2, 3]);
    }
}
