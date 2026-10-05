use crate::infrastructure::push_provider::{PushProvider, PushSendError};
use business::gateway::mention_notification_gateway::MentionNotificationGateway;
use business::gateway::push_device_gateway::PushDeviceGateway;
use business::gateway::push_preference_gateway::{PushPreferenceError, PushPreferenceGateway};
use domain::in_app_notification::InAppNotification;
use mongodb::Database;
use mongodb::bson::DateTime;
use std::sync::Arc;
use tokio::time::{Duration, sleep};

pub fn start(db: Arc<Database>) {
    tokio::spawn(async move {
        let provider = match PushProvider::from_environment() {
            Ok(provider) => provider,
            Err(error) => {
                log::error!("ALERT: push provider configuration is invalid: {error:?}");
                return;
            }
        };

        log::info!("Push notification worker started");
        loop {
            match process_pending(&db, &provider).await {
                Ok(0) => sleep(Duration::from_secs(3)).await,
                Ok(count) => log::info!("Processed {count} push notification(s)"),
                Err(error) => {
                    log::error!("Push notification worker failed: {}", error.message);
                    sleep(Duration::from_secs(5)).await;
                }
            }
        }
    });
}

async fn process_pending(
    db: &Database,
    provider: &PushProvider,
) -> Result<usize, domain::business_error::BusinessError> {
    let gateway = MentionNotificationGateway::new(db);
    let pending = gateway.list_pending_push_notifications(20).await?;
    let mut processed = 0;

    for notification in pending {
        if !gateway.claim_push_notification(&notification.uuid).await? {
            continue;
        }
        match PushPreferenceGateway::notifications_enabled(&notification.recipient_person_uuid)
            .await
        {
            Ok(false) | Err(PushPreferenceError::NotFound) => {
                update_state(
                    db,
                    &notification,
                    "Suppressed",
                    notification.push_attempt_count,
                    None,
                    None,
                )
                .await?;
                processed += 1;
                continue;
            }
            Ok(true) => {}
            Err(PushPreferenceError::Permanent) => {
                log::error!(
                    "ALERT: permanent authorization/configuration failure querying push preference; notificationUuid={}",
                    notification.uuid
                );
                update_state(
                    db,
                    &notification,
                    "Failed",
                    notification.push_attempt_count,
                    None,
                    Some("permanent settings service authorization/configuration failure"),
                )
                .await?;
                processed += 1;
                continue;
            }
            Err(PushPreferenceError::Transient) => {
                retry_or_fail(
                    db,
                    &notification,
                    "settings service temporarily unavailable",
                )
                .await?;
                processed += 1;
                continue;
            }
        }

        let devices =
            match PushDeviceGateway::find_all_for_person(db, &notification.recipient_person_uuid)
                .await
            {
                Ok(devices) => devices,
                Err(error) => {
                    retry_or_fail(db, &notification, &error.message).await?;
                    processed += 1;
                    continue;
                }
            };
        if devices.is_empty() {
            update_state(
                db,
                &notification,
                "Suppressed",
                notification.push_attempt_count,
                None,
                None,
            )
            .await?;
            processed += 1;
            continue;
        }

        let mut completed = notification.push_completed_device_uuids.clone();
        let mut transient_error = None;
        let mut configuration_error = None;
        for device in devices {
            if completed.contains(&device.device_uuid) {
                continue;
            }
            match provider
                .send(db, &device.registration_token, &notification)
                .await
            {
                Ok(()) => completed.push(device.device_uuid),
                Err(PushSendError::InvalidToken) => {
                    PushDeviceGateway::remove_invalid_token(db, &device.registration_token).await?;
                    completed.push(device.device_uuid);
                }
                Err(PushSendError::Transient(error)) => transient_error = Some(error),
                Err(PushSendError::Configuration(error)) => {
                    configuration_error = Some(error);
                    break;
                }
            }
            update_state_with_completed(
                db,
                &notification,
                "Processing",
                notification.push_attempt_count,
                &completed,
                None,
                None,
            )
            .await?;
        }

        if let Some(error) = configuration_error {
            log::error!(
                "ALERT: permanent FCM/APNs configuration failure; notificationUuid={}: {}",
                notification.uuid,
                error
            );
            update_state_with_completed(
                db,
                &notification,
                "Failed",
                notification.push_attempt_count,
                &completed,
                None,
                Some(&error),
            )
            .await?;
        } else if let Some(error) = transient_error {
            retry_or_fail_with_completed(db, &notification, &completed, &error).await?;
        } else {
            update_state_with_completed(
                db,
                &notification,
                "Sent",
                notification.push_attempt_count,
                &completed,
                None,
                None,
            )
            .await?;
        }
        processed += 1;
    }

    Ok(processed)
}

async fn retry_or_fail(
    db: &Database,
    notification: &InAppNotification,
    error: &str,
) -> Result<(), domain::business_error::BusinessError> {
    retry_or_fail_with_completed(
        db,
        notification,
        &notification.push_completed_device_uuids,
        error,
    )
    .await
}

async fn retry_or_fail_with_completed(
    db: &Database,
    notification: &InAppNotification,
    completed: &[String],
    error: &str,
) -> Result<(), domain::business_error::BusinessError> {
    let attempts = notification.push_attempt_count + 1;
    if attempts >= 5 {
        log::error!(
            "ALERT: push notification exhausted five attempts; notificationUuid={}",
            notification.uuid
        );
        update_state_with_completed(
            db,
            notification,
            "Failed",
            attempts,
            completed,
            None,
            Some(error),
        )
        .await
    } else {
        let delay = retry_delay_seconds(attempts);
        let retry_at = DateTime::from_millis(DateTime::now().timestamp_millis() + delay * 1000);
        update_state_with_completed(
            db,
            notification,
            "Pending",
            attempts,
            completed,
            Some(retry_at),
            Some(error),
        )
        .await
    }
}

async fn update_state(
    db: &Database,
    notification: &InAppNotification,
    status: &str,
    attempts: i32,
    retry_at: Option<DateTime>,
    error: Option<&str>,
) -> Result<(), domain::business_error::BusinessError> {
    update_state_with_completed(
        db,
        notification,
        status,
        attempts,
        &notification.push_completed_device_uuids,
        retry_at,
        error,
    )
    .await
}

async fn update_state_with_completed(
    db: &Database,
    notification: &InAppNotification,
    status: &str,
    attempts: i32,
    completed: &[String],
    retry_at: Option<DateTime>,
    error: Option<&str>,
) -> Result<(), domain::business_error::BusinessError> {
    MentionNotificationGateway::new(db)
        .update_push_state(
            &notification.uuid,
            status,
            attempts,
            completed,
            retry_at,
            error,
        )
        .await
}

fn retry_delay_seconds(attempts: i32) -> i64 {
    2_i64.pow(attempts.clamp(1, 8) as u32).min(300)
}

#[cfg(test)]
mod tests {
    use super::{PushProvider, process_pending, retry_delay_seconds};
    use business::gateway::mention_notification_gateway::MentionNotificationGateway;
    use business::gateway::push_device_gateway::PushDeviceGateway;
    use business::proto::proto::settings::settings_service_server::{
        SettingsService, SettingsServiceServer,
    };
    use business::proto::proto::settings::{
        OwnerUuidRequest, PushPreferenceResponse, Setting, SettingIdRequest,
        SettingOwnerIdRequest,
    };
    use domain::in_app_notification::InAppNotification;
    use futures::TryStreamExt;
    use mongodb::{Client as MongoClient, bson::doc};
    use tonic::{Request, Response, Status};

    struct FakeSettingsService {
        enabled_owner_uuid: String,
        expected_header_value: String,
    }

    #[tonic::async_trait]
    impl SettingsService for FakeSettingsService {
        async fn get_by_id(
            &self,
            _: Request<SettingIdRequest>,
        ) -> Result<Response<Setting>, Status> {
            Err(Status::unimplemented("not used by push worker"))
        }

        async fn persist_settings(
            &self,
            _: Request<Setting>,
        ) -> Result<Response<Setting>, Status> {
            Err(Status::unimplemented("not used by push worker"))
        }

        async fn get_by_uuid(
            &self,
            _: Request<SettingIdRequest>,
        ) -> Result<Response<Setting>, Status> {
            Err(Status::unimplemented("not used by push worker"))
        }

        async fn get_by_owner_ids(
            &self,
            _: Request<SettingOwnerIdRequest>,
        ) -> Result<Response<Setting>, Status> {
            Err(Status::unimplemented("not used by push worker"))
        }

        async fn get_push_preference_by_owner_uuid(
            &self,
            request: Request<OwnerUuidRequest>,
        ) -> Result<Response<PushPreferenceResponse>, Status> {
            let secret = request
                .metadata()
                .get("x-internal-secret")
                .and_then(|value| value.to_str().ok());
            if secret != Some(self.expected_header_value.as_str()) {
                return Err(Status::unauthenticated("invalid internal service secret"));
            }
            Ok(Response::new(PushPreferenceResponse {
                notifications_enabled: request.into_inner().owner_uuid
                    == self.enabled_owner_uuid,
            }))
        }
    }

    #[test]
    fn push_retry_delay_is_bounded() {
        assert_eq!(retry_delay_seconds(1), 2);
        assert_eq!(retry_delay_seconds(4), 16);
        assert_eq!(retry_delay_seconds(100), 256);
    }

    #[tokio::test]
    #[ignore = "requires a disposable TEST_MONGO_URL MongoDB database"]
    async fn c006_push_worker_obeys_preference_and_fake_provider() {
        let database_url = std::env::var("TEST_MONGO_URL")
            .expect("TEST_MONGO_URL must point to a disposable timeline_test database");
        let client = MongoClient::with_uri_str(database_url).await.unwrap();
        let database = client.database("timeline_test");
        let enabled_owner = "c006-tc009-enabled-owner";
        let disabled_owner = "c006-tc009-disabled-owner";
        let notification_ids = vec![
            "c006-tc009-enabled-notification".to_string(),
            "c006-tc009-disabled-notification".to_string(),
            "c006-tc009-retry-notification".to_string(),
            "c006-tc009-invalid-token-notification".to_string(),
            "c006-tc009-config-error-notification".to_string(),
        ];
        let device_ids = vec![
            "c006-tc009-enabled-device".to_string(),
            "c006-tc009-disabled-device".to_string(),
            "c006-tc009-invalid-device".to_string(),
            "c006-tc009-config-device".to_string(),
        ];
        database
            .collection::<InAppNotification>("in_app_notifications")
            .delete_many(doc! { "_id": { "$in": &notification_ids } })
            .await
            .unwrap();
        database
            .collection::<domain::push_device::PushDevice>("push_devices")
            .delete_many(doc! { "deviceUuid": { "$in": &device_ids } })
            .await
            .unwrap();
        database
            .collection::<mongodb::bson::Document>("push_provider_test_deliveries")
            .delete_many(doc! { "notificationUuid": { "$in": &notification_ids } })
            .await
            .unwrap();

        let test_header_value = "test-header";
        let service_listener = std::net::TcpListener::bind("127.0.0.1:0").unwrap();
        let service_address = service_listener.local_addr().unwrap();
        drop(service_listener);
        let service = tokio::spawn(async move {
            tonic::transport::Server::builder()
                .add_service(SettingsServiceServer::new(FakeSettingsService {
                    enabled_owner_uuid: enabled_owner.to_string(),
                    expected_header_value: test_header_value.to_string(),
                }))
                .serve(service_address)
                .await
                .unwrap();
        });

        for (name, value) in [
            ("PUSH_PROVIDER_MODE", "fake"),
            ("INTERNAL_SERVICE_SECRET", test_header_value),
            ("GRPC_PROTOCOL", "http"),
            ("GRPC_HOST", "127.0.0.1"),
            ("GRPC_PORT", &service_address.port().to_string()),
            ("GRPC_USE_TLS", "false"),
        ] {
            unsafe { std::env::set_var(name, value) };
        }

        for (index, owner_uuid) in [enabled_owner, disabled_owner].into_iter().enumerate() {
            PushDeviceGateway::register(
                &database,
                &device_ids[index],
                owner_uuid,
                "android",
                &format!("c006-tc009-token-{index}"),
            )
            .await
            .unwrap();
            MentionNotificationGateway::new(&database)
                .persist_in_app_notification(InAppNotification::from_social_interaction(
                    notification_ids[index].clone(),
                    "Comment".to_string(),
                    owner_uuid.to_string(),
                    "c006-tc009-actor".to_string(),
                    "Actor".to_string(),
                    "c006-tc009-post".to_string(),
                    Some("c006-tc009-comment".to_string()),
                    "Private comment content".to_string(),
                ))
                .await
                .unwrap();
        }

        let provider = PushProvider::from_environment().unwrap();
        unsafe { std::env::remove_var("FAKE_PUSH_PROVIDER_STATUS") };
        process_pending(&database, &provider).await.unwrap();

        let notifications = database
            .collection::<InAppNotification>("in_app_notifications");
        let enabled = notifications
            .find_one(doc! { "_id": &notification_ids[0] })
            .await
            .unwrap()
            .unwrap();
        let disabled = notifications
            .find_one(doc! { "_id": &notification_ids[1] })
            .await
            .unwrap()
            .unwrap();
        assert_eq!(enabled.push_status.as_deref(), Some("Sent"));
        assert_eq!(disabled.push_status.as_deref(), Some("Suppressed"));
        assert!(!disabled.read);

        let deliveries = database
            .collection::<mongodb::bson::Document>("push_provider_test_deliveries")
            .find(doc! {})
            .await
            .unwrap()
            .try_collect::<Vec<_>>()
            .await
            .unwrap();
        let enabled_deliveries: Vec<_> = deliveries
            .iter()
            .filter(|delivery| {
                delivery.get_str("notificationUuid").ok() == Some(&notification_ids[0])
            })
            .collect();
        let disabled_deliveries: Vec<_> = deliveries
            .iter()
            .filter(|delivery| {
                delivery.get_str("notificationUuid").ok() == Some(&notification_ids[1])
            })
            .collect();
        assert_eq!(enabled_deliveries.len(), 1);
        assert!(!enabled_deliveries[0].contains_key("token"));
        assert!(disabled_deliveries.is_empty());

        MentionNotificationGateway::new(&database)
            .persist_in_app_notification(InAppNotification::from_social_interaction(
                notification_ids[2].clone(),
                "Comment".to_string(),
                enabled_owner.to_string(),
                "c006-tc009-actor".to_string(),
                "Actor".to_string(),
                "c006-tc009-post".to_string(),
                Some("c006-tc009-retry-comment".to_string()),
                "Private comment content".to_string(),
            ))
            .await
            .unwrap();
        unsafe { std::env::set_var("FAKE_PUSH_PROVIDER_STATUS", "429") };
        process_pending(&database, &provider).await.unwrap();
        unsafe { std::env::remove_var("FAKE_PUSH_PROVIDER_STATUS") };

        let retried = notifications
            .find_one(doc! { "_id": &notification_ids[2] })
            .await
            .unwrap()
            .unwrap();
        assert_eq!(retried.push_status.as_deref(), Some("Pending"));
        assert_eq!(retried.push_attempt_count, 1);
        assert!(retried.push_next_attempt_at.is_some());
        assert!(!retried.read);
        notifications
            .update_one(
                doc! { "_id": &notification_ids[2] },
                doc! { "$set": { "pushStatus": "Failed", "pushNextAttemptAt": null } },
            )
            .await
            .unwrap();

        PushDeviceGateway::register(
            &database,
            &device_ids[2],
            enabled_owner,
            "android",
            "c006-tc009-invalid-token",
        )
        .await
        .unwrap();
        MentionNotificationGateway::new(&database)
            .persist_in_app_notification(InAppNotification::from_social_interaction(
                notification_ids[3].clone(),
                "Comment".to_string(),
                enabled_owner.to_string(),
                "c006-tc009-actor".to_string(),
                "Actor".to_string(),
                "c006-tc009-post".to_string(),
                Some("c006-tc009-invalid-comment".to_string()),
                "Private comment content".to_string(),
            ))
            .await
            .unwrap();
        unsafe { std::env::set_var("FAKE_PUSH_PROVIDER_STATUS", "invalid-token") };
        process_pending(&database, &provider).await.unwrap();
        unsafe { std::env::remove_var("FAKE_PUSH_PROVIDER_STATUS") };
        assert!(
            PushDeviceGateway::find_all_for_person(&database, enabled_owner)
                .await
                .unwrap()
                .iter()
                .all(|device| device.device_uuid != device_ids[2])
        );
        let invalid_token_notification = notifications
            .find_one(doc! { "_id": &notification_ids[3] })
            .await
            .unwrap()
            .unwrap();
        assert_eq!(
            invalid_token_notification.push_status.as_deref(),
            Some("Sent")
        );
        assert!(!invalid_token_notification.read);

        PushDeviceGateway::register(
            &database,
            &device_ids[3],
            enabled_owner,
            "android",
            "c006-tc009-config-token",
        )
        .await
        .unwrap();
        MentionNotificationGateway::new(&database)
            .persist_in_app_notification(InAppNotification::from_social_interaction(
                notification_ids[4].clone(),
                "Comment".to_string(),
                enabled_owner.to_string(),
                "c006-tc009-actor".to_string(),
                "Actor".to_string(),
                "c006-tc009-post".to_string(),
                Some("c006-tc009-config-comment".to_string()),
                "Private comment content".to_string(),
            ))
            .await
            .unwrap();
        unsafe { std::env::set_var("FAKE_PUSH_PROVIDER_STATUS", "401") };
        process_pending(&database, &provider).await.unwrap();
        unsafe { std::env::remove_var("FAKE_PUSH_PROVIDER_STATUS") };
        let configuration_failure = notifications
            .find_one(doc! { "_id": &notification_ids[4] })
            .await
            .unwrap()
            .unwrap();
        assert_eq!(configuration_failure.push_status.as_deref(), Some("Failed"));
        assert_eq!(configuration_failure.push_attempt_count, 0);
        assert!(!configuration_failure.read);

        service.abort();
        let _ = service.await;
        database
            .collection::<InAppNotification>("in_app_notifications")
            .delete_many(doc! { "_id": { "$in": &notification_ids } })
            .await
            .unwrap();
        database
            .collection::<domain::push_device::PushDevice>("push_devices")
            .delete_many(doc! { "deviceUuid": { "$in": &device_ids } })
            .await
            .unwrap();
        database
            .collection::<mongodb::bson::Document>("push_provider_test_deliveries")
            .delete_many(doc! { "notificationUuid": { "$in": &notification_ids } })
            .await
            .unwrap();
        for name in [
            "PUSH_PROVIDER_MODE",
            "INTERNAL_SERVICE_SECRET",
            "GRPC_PROTOCOL",
            "GRPC_HOST",
            "GRPC_PORT",
            "GRPC_USE_TLS",
        ] {
            unsafe { std::env::remove_var(name) };
        }
    }

    #[tokio::test]
    #[ignore = "requires the disposable Workout SettingsService gRPC fixture"]
    async fn c006_live_workout_push_preference_rpc_acceptance() {
        use business::gateway::push_preference_gateway::{
            PushPreferenceError, PushPreferenceGateway,
        };

        assert!(
            PushPreferenceGateway::notifications_enabled(
                "00000000-0000-0000-0000-000000000061",
            )
            .await
            .unwrap()
        );
        assert!(
            !PushPreferenceGateway::notifications_enabled(
                "00000000-0000-0000-0000-000000000062",
            )
            .await
            .unwrap()
        );
        assert_eq!(
            PushPreferenceGateway::notifications_enabled(
                "00000000-0000-0000-0000-000000000099",
            )
            .await,
            Err(PushPreferenceError::NotFound)
        );
    }
}
