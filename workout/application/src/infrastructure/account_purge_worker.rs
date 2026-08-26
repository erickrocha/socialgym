use business::sea_orm::DatabaseConnection;
use business::use_cases::account_purge_use_case::AccountPurgeUseCase;
use std::env;
use std::sync::Arc;
use tokio::time::{interval, Duration};

const ACCOUNT_PURGE_INTERVAL_SECONDS: &str = "ACCOUNT_PURGE_INTERVAL_SECONDS";

/// Spawns a background Tokio task that periodically purges accounts whose
/// deletion grace period (or immediate-deletion request) has elapsed —
/// deleting their `timeline` data first, then cascading through every
/// `workout` table that references them.
///
/// Ticks every `ACCOUNT_PURGE_INTERVAL_SECONDS` (default 900 = 15 min), which
/// also bounds how long an "immediate" deletion request takes to actually
/// complete (the account is deactivated and logged out immediately regardless;
/// only the physical data purge is delayed to this tick).
pub fn start(db: Arc<DatabaseConnection>) {
    let interval_seconds = env::var(ACCOUNT_PURGE_INTERVAL_SECONDS)
        .ok()
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(900);

    tokio::spawn(async move {
        log::info!(
            "Account purge worker starting… (interval: {}s)",
            interval_seconds
        );
        let mut ticker = interval(Duration::from_secs(interval_seconds));

        loop {
            ticker.tick().await;
            match AccountPurgeUseCase::purge_due_accounts(&db).await {
                Ok(0) => {}
                Ok(count) => log::info!("Account purge worker purged {} account(s)", count),
                Err(e) => log::error!("Account purge worker encountered an error: {:?}", e),
            }
        }
    });
}
