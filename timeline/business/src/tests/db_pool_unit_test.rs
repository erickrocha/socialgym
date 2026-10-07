use super::connect;

/// One test: the cases share process environment variables, so they cannot run in parallel.
/// `ClientOptions::parse` and `Client::with_options` do not open a connection.
#[tokio::test]
async fn connect_needs_both_names_and_caps_the_pool() {
    // SAFETY: no other test in this crate reads these variables.
    unsafe {
        std::env::remove_var("DATABASE_URL");
        std::env::remove_var("DATABASE_NAME");
    }
    assert!(connect().await.unwrap_err().to_string().contains("DATABASE_URL"));

    unsafe { std::env::set_var("DATABASE_URL", "mongodb://localhost:1/x") };
    assert!(connect().await.unwrap_err().to_string().contains("DATABASE_NAME"));

    unsafe {
        std::env::set_var("DATABASE_NAME", "unit");
        std::env::set_var("DB_POOL_MAX_SIZE", "3");
    }
    assert_eq!(connect().await.expect("lazy connect").name(), "unit");

    unsafe { std::env::set_var("DATABASE_URL", "not a url") };
    assert!(connect().await.is_err());
}
