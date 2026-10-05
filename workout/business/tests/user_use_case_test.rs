use business::domain::user::User;
use business::use_cases::user_use_case::{UserUseCase, UserUseCaseError};
use chrono::{DateTime, Utc};
use entity::user_entity::UserEntity;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use std::env;
use tokio::sync::Mutex;
use uuid::Uuid;

static USER_ENV_LOCK: Mutex<()> = Mutex::const_new(());

fn clear_auth_toggle_env() {
    env::remove_var("AUTH_RULES_ENABLED");
    env::remove_var("PASSWORD_POLICY_ENABLED");
    env::remove_var("LOGIN_LOCKOUT_ENABLED");
    env::remove_var("TOKEN_REVOCATION_ENABLED");
}

fn user_entity(email: &str) -> UserEntity {
    UserEntity {
        id: 1,
        name: Some("John doe".to_string()),
        email: email.to_string(),
        password: "hashed".to_string(),
        enabled: true,
        first_login: true,
        person_id: 1,
        person_uuid: Uuid::new_v4(),
        uuid: Uuid::new_v4(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        failed_login_attempts: 0,
        locked_until: None::<DateTime<Utc>>,
        token_valid_after: None,
        deletion_requested_at: None,
        deletion_scheduled_at: None,
    }
}

#[tokio::test]
async fn add_rejects_empty_email() {
    let _guard = USER_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let user = User::new(
        Some("John doe".to_string()),
        "".to_string(),
        "password".to_string(),
        1,
        Uuid::new_v4().to_string(),
    );

    let result = UserUseCase::add(&db, user).await;

    assert!(matches!(result, Err(UserUseCaseError::InvalidInput)));
}

#[tokio::test]
async fn add_rejects_empty_password() {
    let _guard = USER_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let user = User::new(
        Some("John doe".to_string()),
        "test@example.com".to_string(),
        "".to_string(),
        1,
        Uuid::new_v4().to_string(),
    );

    let result = UserUseCase::add(&db, user).await;

    assert!(matches!(result, Err(UserUseCaseError::InvalidInput)));
}

#[tokio::test]
async fn add_persists_valid_user() {
    let _guard = USER_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![user_entity("test@example.com")]])
        .into_connection();
    let user = User::new(
        Some("John doe".to_string()),
        "test@example.com".to_string(),
        "Str0ng!Pass".to_string(),
        1,
        Uuid::new_v4().to_string(),
    );

    let added_user = UserUseCase::add(&db, user).await.unwrap();

    assert_eq!(added_user.email, "test@example.com");
}

#[tokio::test]
async fn add_rejects_weak_password_when_policy_is_enabled() {
    let _guard = USER_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let user = User::new(
        Some("John doe".to_string()),
        "test@example.com".to_string(),
        "weak".to_string(),
        1,
        Uuid::new_v4().to_string(),
    );

    let result = UserUseCase::add(&db, user).await;

    assert!(matches!(result, Err(UserUseCaseError::WeakPassword(_))));
}

#[tokio::test]
async fn add_accepts_weak_password_when_policy_is_disabled() {
    let _guard = USER_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("PASSWORD_POLICY_ENABLED", "false");
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![user_entity("weak@example.com")]])
        .into_connection();
    let user = User::new(
        Some("John doe".to_string()),
        "weak@example.com".to_string(),
        "weak".to_string(),
        1,
        Uuid::new_v4().to_string(),
    );

    let result = UserUseCase::add(&db, user).await;

    clear_auth_toggle_env();
    assert!(result.is_ok());
}
