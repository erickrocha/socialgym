use domain::business_error::BusinessError;
use domain::push_device::PushDevice;
use futures::TryStreamExt;
use mongodb::Database;
use mongodb::bson::{DateTime, doc};

const COLLECTION: &str = "push_devices";

pub struct PushDeviceGateway;

impl PushDeviceGateway {
    pub async fn register(
        db: &Database,
        device_uuid: &str,
        person_uuid: &str,
        platform: &str,
        registration_token: &str,
    ) -> Result<(), BusinessError> {
        let collection = db.collection::<PushDevice>(COLLECTION);
        let now = DateTime::now();

        if let Some(existing_token) = collection
            .find_one(doc! { "registrationToken": registration_token })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to find push token: {error}"))
            })?
        {
            // The device keeps one token (unique deviceUuid): drop its stale one before the
            // moved document takes the device uuid.
            collection
                .delete_many(doc! {
                    "deviceUuid": device_uuid,
                    "registrationToken": { "$ne": registration_token },
                })
                .await
                .map_err(|error| {
                    BusinessError::infrastructure(format!(
                        "failed to replace device token: {error}"
                    ))
                })?;
            collection
                .update_one(
                    doc! { "_id": existing_token.id },
                    doc! { "$set": {
                        "deviceUuid": device_uuid,
                        "personUuid": person_uuid,
                        "platform": platform,
                        "updatedAt": now,
                    } },
                )
                .await
                .map_err(|error| {
                    BusinessError::infrastructure(format!("failed to transfer push token: {error}"))
                })?;
            return Ok(());
        }

        let existing_device = collection
            .find_one(doc! { "deviceUuid": device_uuid })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to find push device: {error}"))
            })?;

        if let Some(existing_device) = existing_device {
            let update = collection
                .update_one(
                    doc! { "_id": existing_device.id },
                    doc! { "$set": {
                        "personUuid": person_uuid,
                        "platform": platform,
                        "registrationToken": registration_token,
                        "updatedAt": now,
                    } },
                )
                .await;
            if let Err(error) = update {
                if !is_duplicate_key(&error.to_string()) {
                    return Err(BusinessError::infrastructure(format!(
                        "failed to rotate push token: {error}"
                    )));
                }
                return Self::transfer_duplicate_token(
                    &collection,
                    device_uuid,
                    person_uuid,
                    platform,
                    registration_token,
                    now,
                )
                .await;
            }
            return Ok(());
        }

        let device = PushDevice {
            id: uuid::Uuid::new_v4().to_string(),
            device_uuid: device_uuid.to_string(),
            person_uuid: person_uuid.to_string(),
            platform: platform.to_string(),
            registration_token: registration_token.to_string(),
            created_at: now,
            updated_at: now,
        };
        match collection.insert_one(device).await {
            Ok(_) => Ok(()),
            Err(error) if is_duplicate_key(&error.to_string()) => {
                Self::transfer_duplicate_token(
                    &collection,
                    device_uuid,
                    person_uuid,
                    platform,
                    registration_token,
                    now,
                )
                .await
            }
            Err(error) => Err(BusinessError::infrastructure(format!(
                "failed to register push device: {error}"
            ))),
        }
    }

    async fn transfer_duplicate_token(
        collection: &mongodb::Collection<PushDevice>,
        device_uuid: &str,
        person_uuid: &str,
        platform: &str,
        registration_token: &str,
        now: DateTime,
    ) -> Result<(), BusinessError> {
        collection
            .delete_many(doc! {
                "deviceUuid": device_uuid,
                "registrationToken": { "$ne": registration_token },
            })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to replace device token: {error}"))
            })?;
        collection
            .update_one(
                doc! { "registrationToken": registration_token },
                doc! { "$set": {
                    "deviceUuid": device_uuid,
                    "personUuid": person_uuid,
                    "platform": platform,
                    "updatedAt": now,
                } },
            )
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to transfer push token: {error}"))
            })?;
        Ok(())
    }

    pub async fn remove_owned(
        db: &Database,
        device_uuid: &str,
        person_uuid: &str,
    ) -> Result<bool, BusinessError> {
        let result = db
            .collection::<PushDevice>(COLLECTION)
            .delete_one(doc! { "deviceUuid": device_uuid, "personUuid": person_uuid })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to remove push device: {error}"))
            })?;
        Ok(result.deleted_count > 0)
    }

    pub async fn find_all_for_person(
        db: &Database,
        person_uuid: &str,
    ) -> Result<Vec<PushDevice>, BusinessError> {
        db.collection::<PushDevice>(COLLECTION)
            .find(doc! { "personUuid": person_uuid })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to list push devices: {error}"))
            })?
            .try_collect()
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!("failed to read push devices: {error}"))
            })
    }

    pub async fn remove_invalid_token(
        db: &Database,
        registration_token: &str,
    ) -> Result<(), BusinessError> {
        db.collection::<PushDevice>(COLLECTION)
            .delete_one(doc! { "registrationToken": registration_token })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!(
                    "failed to remove invalid push token: {error}"
                ))
            })?;
        Ok(())
    }

    pub async fn delete_all_for_person(
        db: &Database,
        person_uuid: &str,
    ) -> Result<(), BusinessError> {
        db.collection::<PushDevice>(COLLECTION)
            .delete_many(doc! { "personUuid": person_uuid })
            .await
            .map_err(|error| {
                BusinessError::infrastructure(format!(
                    "failed to delete person push devices: {error}"
                ))
            })?;
        Ok(())
    }
}

fn is_duplicate_key(error: &str) -> bool {
    let error = error.to_ascii_lowercase();
    error.contains("e11000") || error.contains("duplicate key")
}
