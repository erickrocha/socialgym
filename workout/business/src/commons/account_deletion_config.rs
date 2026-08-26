use std::env;

const ACCOUNT_DELETION_GRACE_DAYS: &str = "ACCOUNT_DELETION_GRACE_DAYS";

/// Days between an account-deletion request and permanent purge, for the
/// non-immediate ("30 days like Facebook/Instagram") path.
pub fn grace_period_days() -> i64 {
    env::var(ACCOUNT_DELETION_GRACE_DAYS)
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(30)
}
