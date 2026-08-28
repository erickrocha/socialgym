use crate::domain::business_error::BusinessError;
use crate::gateway::business_profile_address_gateway::BusinessProfileAddressGateway;
use crate::gateway::business_profile_gateway::BusinessProfileGateway;
use crate::gateway::data_export_gateway::DataExportGateway;
use crate::gateway::friend_gateway::FriendGateway;
use crate::gateway::person_address_gateway::PersonAddressGateway;
use crate::gateway::person_gateway::PersonGateway;
use crate::gateway::person_info_gateway::PersonInfoGateway;
use crate::gateway::person_media_gateway::PersonMediaGateway;
use crate::gateway::profile_gateway::ProfileGateway;
use crate::gateway::settings_gateway::SettingsGateway;
use crate::gateway::team_member_gateway::TeamMemberGateway;
use crate::gateway::timeline_deletion_gateway::TimelineDeletionGateway;
use crate::gateway::token_revocation_gateway::TokenRevocationGateway;
use crate::gateway::user_gateway::UserGateway;
use crate::gateway::workout_gateway::WorkoutGateway;
use crate::use_cases::image_storage_use_case::ImageStorageUseCase;
use chrono::Utc;
use sea_orm::{DbConn, TransactionTrait};

pub struct AccountPurgeUseCase {}

impl AccountPurgeUseCase {
    /// Finds every account whose deletion grace period (or immediate request)
    /// has elapsed and permanently purges it, in both `timeline` and `workout`.
    /// Returns the number of accounts successfully purged this tick.
    pub async fn purge_due_accounts(db: &DbConn) -> Result<u32, BusinessError> {
        let due = UserGateway::find_due_for_deletion(db, Utc::now())
            .await
            .map_err(|e| {
                log::error!("Failed to query accounts due for deletion: {}", e);
                BusinessError::infrastructure(
                    "Failed to query accounts due for deletion".to_string(),
                )
            })?;

        let mut purged = 0u32;
        for user in due {
            let user_id = user.id;
            let person_id = user.person_id;
            let person_uuid = user.person_uuid.to_string();

            let exports = match DataExportGateway::list_for_person(db, person_id).await {
                Ok(exports) => exports,
                Err(e) => {
                    log::error!(
                        "Skipping purge for user_id={}: failed to enumerate exports: {}",
                        user_id,
                        e
                    );
                    continue;
                }
            };
            let mut export_failed = false;
            for export in exports {
                if let Some(key) = export.object_key {
                    if let Err(e) = ImageStorageUseCase::delete_presigned_url(key.clone()).await {
                        log::error!(
                            "Skipping purge for user_id={}: failed to delete export {}: {}",
                            user_id,
                            key,
                            e
                        );
                        export_failed = true;
                    }
                }
            }
            if export_failed {
                continue;
            }

            // Object storage is not covered by database cascades. Delete every
            // tracked object first and leave the account due for retry if any
            // object cannot be removed.
            let media = match PersonMediaGateway::find_all_by_person(db, person_id).await {
                Ok(media) => media,
                Err(e) => {
                    log::error!(
                        "Skipping purge for user_id={}: failed to enumerate media: {}",
                        user_id,
                        e
                    );
                    continue;
                }
            };
            let mut media_failed = false;
            for item in media {
                if let Err(e) = ImageStorageUseCase::delete_presigned_url(item.s3_key.clone()).await
                {
                    log::error!(
                        "Skipping purge for user_id={}: failed to delete S3 object {}: {}",
                        user_id,
                        item.s3_key,
                        e
                    );
                    media_failed = true;
                }
            }
            if media_failed {
                continue;
            }

            // Delete the timeline side first: if it fails, leave everything in
            // `workout` untouched so the next sweep tick retries the whole person.
            if let Err(e) = TimelineDeletionGateway::delete_person_data(&person_uuid).await {
                log::error!(
                    "Skipping purge for user_id={} (person_uuid={}): timeline deletion failed: {}",
                    user_id,
                    person_uuid,
                    e
                );
                continue;
            }

            if let Err(e) = Self::purge_workout_data(db, user_id, person_id).await {
                log::error!(
                    "Failed to purge workout data for user_id={} (person_id={}): {}",
                    user_id,
                    person_id,
                    e
                );
                continue;
            }

            log::info!("Purged account user_id={} person_id={}", user_id, person_id);
            purged += 1;
        }

        Ok(purged)
    }

    async fn purge_workout_data(
        db: &DbConn,
        user_id: i32,
        person_id: i32,
    ) -> Result<(), BusinessError> {
        // Read before opening the transaction: `find_by_owner_id` takes a plain
        // `&DbConn`, not a generic connection, so it can't run against `&txn`.
        let business_profiles = BusinessProfileGateway::find_by_owner_id(db, person_id).await;

        let txn = db.begin().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to start purge transaction: {}", e))
        })?;

        let map_err = |e: sea_orm::DbErr| {
            BusinessError::infrastructure(format!("Purge cascade failed: {}", e))
        };

        TokenRevocationGateway::delete_all_by_user_id(&txn, user_id)
            .await
            .map_err(map_err)?;

        for business_profile in business_profiles {
            BusinessProfileAddressGateway::delete_all_by_business_profile_id(
                &txn,
                business_profile.id,
            )
            .await
            .map_err(map_err)?;
            TeamMemberGateway::delete_all_by_business_profile_id(&txn, business_profile.id)
                .await
                .map_err(map_err)?;
            ProfileGateway::delete_all_by_business_profile_id(&txn, business_profile.id)
                .await
                .map_err(map_err)?;
            BusinessProfileGateway::delete_by_id(&txn, business_profile.id)
                .await
                .map_err(map_err)?;
        }

        WorkoutGateway::delete_all_by_owner_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        TeamMemberGateway::delete_all_by_person_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        ProfileGateway::delete_all_by_person_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        FriendGateway::delete_all_involving_person(&txn, person_id)
            .await
            .map_err(map_err)?;
        SettingsGateway::delete_by_person_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        PersonInfoGateway::delete_by_person_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        PersonAddressGateway::delete_all_by_person_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        UserGateway::delete_by_person_id(&txn, person_id)
            .await
            .map_err(map_err)?;
        // Cascades exercise / workout_exercise / person_media automatically.
        PersonGateway::delete_by_id(&txn, person_id)
            .await
            .map_err(map_err)?;

        txn.commit().await.map_err(|e| {
            BusinessError::infrastructure(format!("Failed to commit purge transaction: {}", e))
        })
    }
}
