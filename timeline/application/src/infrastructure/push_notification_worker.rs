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
    use super::retry_delay_seconds;

    #[test]
    fn push_retry_delay_is_bounded() {
        assert_eq!(retry_delay_seconds(1), 2);
        assert_eq!(retry_delay_seconds(4), 16);
        assert_eq!(retry_delay_seconds(100), 256);
    }
}
