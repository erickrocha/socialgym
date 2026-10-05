use business::domain::person::Person;
use business::domain::user::User;
use business::use_cases::authentication::{Authentication, AuthenticationError, ValidateError};
use chrono::{DateTime, NaiveDate, Utc};
use entity::person_entity::PersonEntity;
use entity::revoked_token_entity::RevokedTokenEntity;
use entity::user_entity::UserEntity;
use sea_orm::{DatabaseBackend, MockDatabase};
use std::env;
use tokio::sync::Mutex;
use uuid::Uuid;

static AUTH_ENV_LOCK: Mutex<()> = Mutex::const_new(());

fn clear_auth_toggle_env() {
    env::remove_var("AUTH_RULES_ENABLED");
    env::remove_var("PASSWORD_POLICY_ENABLED");
    env::remove_var("LOGIN_LOCKOUT_ENABLED");
    env::remove_var("TOKEN_REVOCATION_ENABLED");
}

fn user_entity(
    email: &str,
    password_hash: &str,
    failed_login_attempts: i32,
    locked_until: Option<DateTime<Utc>>,
    token_valid_after: Option<DateTime<Utc>>,
) -> UserEntity {
    UserEntity {
        id: 1,
        name: Some("John doe".to_string()),
        email: email.to_string(),
        password: password_hash.to_string(),
        enabled: true,
        first_login: true,
        person_id: 1,
        person_uuid: Uuid::new_v4(),
        uuid: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        failed_login_attempts,
        locked_until,
        token_valid_after,
        deletion_requested_at: None,
        deletion_scheduled_at: None,
    }
}

fn person_entity() -> PersonEntity {
    PersonEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        first_name: "John".to_string(),
        surname: "Doe".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        gender: "M".to_string(),
        avatar: None,
        cover_image: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn domain_user() -> User {
    User::new(
        Some("John doe".to_string()),
        "test@example.com".to_string(),
        "password".to_string(),
        1,
        Uuid::new_v4().to_string(),
    )
}

fn domain_person() -> Person {
    Person::new(
        "John".to_string(),
        "Doe".to_string(),
        NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
        "M".to_string(),
    )
}

#[tokio::test]
async fn generate_access_token_returns_bearer_access_and_refresh_tokens() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key_for_access_token");
    env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");

    let token = Authentication::generate_access_token(&domain_user(), &domain_person(), None, true);

    assert_eq!(token.token_type, "Bearer");
    assert!(!token.access_token.is_empty());
    assert!(!token.refresh_token.unwrap().is_empty());
    assert_eq!(token.username, "test@example.com");
}

#[tokio::test]
async fn login_then_validate_accepts_valid_credentials_and_token() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_acceptance");
    env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret_acceptance");
    let hash = bcrypt::hash("CorrectPass1!", bcrypt::DEFAULT_COST).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            &hash,
            0,
            None,
            None,
        )]])
        .append_query_results(vec![vec![person_entity()]])
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            &hash,
            0,
            None,
            None,
        )]])
        .into_connection();

    let token = Authentication::execute(
        &db,
        "test@example.com".to_string(),
        "CorrectPass1!".to_string(),
    )
    .await
    .unwrap();

    assert_eq!(token.token_type, "Bearer");
    assert!(!token.access_token.is_empty());
    assert!(token.refresh_token.is_some());
    assert!(Authentication::validate(&db, token.access_token)
        .await
        .is_ok());
    clear_auth_toggle_env();
}

#[tokio::test]
async fn execute_rejects_empty_email() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let result = Authentication::execute(&db, "".to_string(), "password".to_string()).await;

    assert!(matches!(
        result,
        Err(AuthenticationError::InvalidCredentials)
    ));
}

#[tokio::test]
async fn execute_rejects_empty_password() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let result = Authentication::execute(&db, "test@example.com".to_string(), "".to_string()).await;

    assert!(matches!(
        result,
        Err(AuthenticationError::InvalidCredentials)
    ));
}

#[tokio::test]
async fn locked_account_rejects_correct_password() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
    env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");
    let hash = bcrypt::hash("CorrectPass1!", bcrypt::DEFAULT_COST).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            &hash,
            2,
            Some(Utc::now() + chrono::Duration::seconds(60)),
            None,
        )]])
        .append_query_results(vec![vec![person_entity()]])
        .into_connection();

    let result = Authentication::execute(
        &db,
        "test@example.com".to_string(),
        "CorrectPass1!".to_string(),
    )
    .await;

    assert!(matches!(
        result,
        Err(AuthenticationError::AccountLocked { .. })
    ));
}

#[tokio::test]
async fn lockout_can_be_disabled_by_configuration() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("LOGIN_LOCKOUT_ENABLED", "false");
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
    env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");
    let hash = bcrypt::hash("CorrectPass1!", bcrypt::DEFAULT_COST).unwrap();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            &hash,
            2,
            Some(Utc::now() + chrono::Duration::seconds(60)),
            None,
        )]])
        .append_query_results(vec![vec![person_entity()]])
        .into_connection();

    let result = Authentication::execute(
        &db,
        "test@example.com".to_string(),
        "CorrectPass1!".to_string(),
    )
    .await;

    clear_auth_toggle_env();
    assert!(result.is_ok());
}

#[tokio::test]
async fn validate_rejects_revoked_jti() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_revoked_jti");
    let token =
        Authentication::generate_access_token(&domain_user(), &domain_person(), None, false);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            "hashed",
            0,
            None,
            None,
        )]])
        .append_query_results(vec![vec![RevokedTokenEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            jti: "revoked-jti".to_string(),
            user_id: 1,
            token_type: "access".to_string(),
            expires_at: Utc::now(),
            created_at: Utc::now(),
        }]])
        .into_connection();

    let result = Authentication::validate(&db, token.access_token).await;

    assert!(matches!(result, Err(ValidateError::Revoked)));
}

#[tokio::test]
async fn validate_rejects_token_issued_before_revocation_watermark() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_watermark");
    let token =
        Authentication::generate_access_token(&domain_user(), &domain_person(), None, false);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            "hashed",
            0,
            None,
            Some(Utc::now() + chrono::Duration::seconds(60)),
        )]])
        .into_connection();

    let result = Authentication::validate(&db, token.access_token).await;

    assert!(matches!(result, Err(ValidateError::Revoked)));
}

#[tokio::test]
async fn validate_skips_revocation_lookup_when_disabled() {
    let _guard = AUTH_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("TOKEN_REVOCATION_ENABLED", "false");
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_disabled");
    let token =
        Authentication::generate_access_token(&domain_user(), &domain_person(), None, false);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity(
            "test@example.com",
            "hashed",
            0,
            None,
            None,
        )]])
        .into_connection();

    let result = Authentication::validate(&db, token.access_token).await;

    clear_auth_toggle_env();
    assert!(result.is_ok());
}
