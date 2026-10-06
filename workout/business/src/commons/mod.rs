pub mod account_deletion_config;
pub mod auth_config;
pub mod authorization;
pub mod db_pool;
pub mod entity_mapper;
pub mod functions;
pub mod gateway;
pub mod legal_documents;
pub mod password_policy;

/// Serializes unit tests that read or write process environment variables (auth and
/// password-policy flags): the test harness runs tests in parallel, so unsynchronized
/// `set_var`/`remove_var` calls made other tests see half-configured settings.
#[cfg(test)]
pub(crate) fn lock_env() -> std::sync::MutexGuard<'static, ()> {
    static ENV_LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
    ENV_LOCK.lock().unwrap_or_else(|poisoned| poisoned.into_inner())
}
