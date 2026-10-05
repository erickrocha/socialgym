use business::domain::business_error::BusinessErrorKind;
use business::domain::person::Person;
use business::domain::user::User;
use business::use_cases::authentication::Authentication;
use business::use_cases::refresh_token::RefreshToken;
use chrono::{DateTime, NaiveDate, Utc};
use entity::person_entity::PersonEntity;
use entity::revoked_token_entity::RevokedTokenEntity;
use entity::user_entity::UserEntity;
use sea_orm::{DatabaseBackend, DbErr, MockDatabase};
use std::env;
use tokio::sync::Mutex;
use uuid::Uuid;

static TOKEN_ENV_LOCK: Mutex<()> = Mutex::const_new(());

fn clear_auth_toggle_env() {
    env::remove_var("AUTH_RULES_ENABLED");
    env::remove_var("PASSWORD_POLICY_ENABLED");
    env::remove_var("LOGIN_LOCKOUT_ENABLED");
    env::remove_var("TOKEN_REVOCATION_ENABLED");
}

fn user_entity() -> UserEntity {
    UserEntity {
        id: 1,
        name: Some("John doe".to_string()),
        email: "test@example.com".to_string(),
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

fn refresh_token(secret_suffix: &str) -> String {
    env::set_var(
        "ACCESS_TOKEN_SECRET",
        format!("test_access_{secret_suffix}"),
    );
    env::set_var(
        "REFRESH_TOKEN_SECRET",
        format!("test_refresh_{secret_suffix}"),
    );
    Authentication::generate_access_token(&domain_user(), &domain_person(), None, true)
        .refresh_token
        .unwrap()
}

#[tokio::test]
async fn execute_rotates_valid_refresh_token() {
    let _guard = TOKEN_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let refresh = refresh_token("rotation");
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity()]])
        .append_query_results(vec![Vec::<RevokedTokenEntity>::new()])
        .append_query_results(vec![vec![person_entity()]])
        .append_query_results(vec![vec![RevokedTokenEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            jti: "rotated-refresh-jti".to_string(),
            user_id: 1,
            token_type: "refresh".to_string(),
            expires_at: Utc::now(),
            created_at: Utc::now(),
        }]])
        .into_connection();

    let refreshed = RefreshToken::execute(&db, refresh).await.unwrap();

    assert_eq!(refreshed.token_type, "Bearer");
    assert!(!refreshed.access_token.is_empty());
    assert!(refreshed.refresh_token.is_some());
    clear_auth_toggle_env();
}

#[tokio::test]
async fn execute_does_not_issue_tokens_when_refresh_revocation_fails() {
    let _guard = TOKEN_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("TOKEN_REVOCATION_ENABLED", "true");
    let refresh = refresh_token("revocation-failure");
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity()]])
        .append_query_results(vec![Vec::<RevokedTokenEntity>::new()])
        .append_query_results(vec![vec![person_entity()]])
        .append_query_errors([DbErr::Custom("revocation storage unavailable".to_string())])
        .into_connection();

    let error = RefreshToken::execute(&db, refresh).await.unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Infrastructure);
    assert_eq!(error.message, "Unable to rotate refresh token");
    clear_auth_toggle_env();
}

#[tokio::test]
async fn execute_rejects_reused_refresh_token() {
    let _guard = TOKEN_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let refresh = refresh_token("reuse");
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![user_entity()]])
        .append_query_results(vec![vec![RevokedTokenEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            jti: "reused-refresh-jti".to_string(),
            user_id: 1,
            token_type: "refresh".to_string(),
            expires_at: Utc::now(),
            created_at: Utc::now(),
        }]])
        .into_connection();

    assert!(RefreshToken::execute(&db, refresh).await.is_err());
    clear_auth_toggle_env();
}

#[tokio::test]
async fn execute_rejects_invalid_token() {
    let _guard = TOKEN_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_invalid_refresh");
    env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_invalid_refresh");
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let error = RefreshToken::execute(&db, "invalid_token".to_string())
        .await
        .unwrap_err();
    assert_eq!(error.kind, BusinessErrorKind::Unauthorized);
    clear_auth_toggle_env();
}
