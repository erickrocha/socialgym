use crate::commons::account_deletion_config;
use crate::domain::business_error::BusinessError;
use crate::gateway::user_gateway::UserGateway;
use crate::use_cases::token_revocation::TokenRevocation;
use chrono::{DateTime, Utc};
use sea_orm::DbConn;

pub struct AccountDeletionStatus {
	pub requested_at: DateTime<Utc>,
	pub scheduled_at: DateTime<Utc>,
}

pub struct AccountDeletionUseCase {}

impl AccountDeletionUseCase {
	/// Disables the account and schedules the cascade purge — immediately (picked
	/// up on the sweep's next tick) or after the grace period (default 30 days).
	/// All current sessions are revoked right away either way.
	pub async fn request_deletion(
		db: &DbConn,
		user_id: i32,
		immediate: bool,
	) -> Result<AccountDeletionStatus, BusinessError> {
		let requested_at = Utc::now();
		let scheduled_at = if immediate {
			requested_at
		} else {
			requested_at + chrono::Duration::days(account_deletion_config::grace_period_days())
		};

		log::info!(
			"Scheduling account deletion for user_id={} at {} (immediate={})",
			user_id,
			scheduled_at,
			immediate
		);

		UserGateway::schedule_deletion(db, user_id, requested_at, scheduled_at)
			.await
			.map_err(|e| {
				log::error!("Failed to schedule deletion for user_id={}: {}", user_id, e);
				BusinessError::infrastructure("Failed to schedule account deletion".to_string())
			})?;

		TokenRevocation::revoke_all_for_user(db, user_id).await;

		Ok(AccountDeletionStatus { requested_at, scheduled_at })
	}

	/// Cancels a pending deletion: re-enables the account and clears both
	/// timestamps. Only meaningful while the row still exists — once the sweep
	/// purges it, there's nothing left to cancel (and the user row is gone).
	pub async fn cancel_deletion(db: &DbConn, user_id: i32) -> Result<(), BusinessError> {
		let user = UserGateway::find_by_id(db, user_id)
			.await
			.map_err(|e| {
				log::error!("Failed to load user_id={} for deletion cancellation: {}", user_id, e);
				BusinessError::infrastructure("Failed to cancel account deletion".to_string())
			})?
			.ok_or_else(|| BusinessError::not_found("User not found".to_string()))?;

		if user.deletion_scheduled_at.is_none() {
			return Err(BusinessError::validation(
				"There is no pending account deletion to cancel".to_string(),
			));
		}

		UserGateway::cancel_deletion(db, user_id).await.map_err(|e| {
			log::error!("Failed to cancel deletion for user_id={}: {}", user_id, e);
			BusinessError::infrastructure("Failed to cancel account deletion".to_string())
		})?;

		log::info!("Cancelled pending account deletion for user_id={}", user_id);
		Ok(())
	}
}
