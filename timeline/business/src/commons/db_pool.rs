use mongodb::{Client, Database, options::ClientOptions};

/// Connects to the database named by `DATABASE_URL` and `DATABASE_NAME`. The driver's pool is
/// unbounded by default, so it gets an explicit, env-configurable cap (`DB_POOL_MAX_SIZE`,
/// default 10): the process cannot open more connections to Mongo than intended.
pub async fn connect() -> Result<Database, Box<dyn std::error::Error + Send + Sync>> {
    let url = std::env::var("DATABASE_URL").map_err(|_| "DATABASE_URL must be set")?;
    let name = std::env::var("DATABASE_NAME").map_err(|_| "DATABASE_NAME must be set")?;
    let mut options = ClientOptions::parse(&url).await?;
    options.max_pool_size = Some(
        std::env::var("DB_POOL_MAX_SIZE")
            .ok()
            .and_then(|s| s.parse().ok())
            .unwrap_or(10),
    );
    Ok(Client::with_options(options)?.database(&name))
}

#[cfg(test)]
#[path = "../tests/db_pool_unit_test.rs"]
mod tests;
