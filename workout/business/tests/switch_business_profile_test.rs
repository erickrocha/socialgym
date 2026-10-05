use business::commons::functions::uuid_to_string;
use business::domain::user::User;
use business::use_cases::switch_business_profile::{
    SwitchBusinessProfile, SwitchBusinessProfileError,
};
use chrono::{NaiveDate, Utc};
use entity::business_profile_entity::BusinessProfileEntity;
use entity::person_entity::PersonEntity;
use sea_orm::{DatabaseBackend, MockDatabase};
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

fn business_profile(owner_id: i32) -> BusinessProfileEntity {
    BusinessProfileEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        owner_id,
        owner_uuid: Uuid::new_v4(),
        tax_id: "12345".to_string(),
        business_name: "Gym XYZ".to_string(),
        business_type: "Professional".to_string(),
        social_name: None,
        logo: None,
        cover_image: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn person(id: i32) -> PersonEntity {
    PersonEntity {
        id,
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

fn user(person_id: i32, email: &str) -> User {
    User::new(
        None,
        email.to_string(),
        "hash".to_string(),
        person_id,
        uuid_to_string(Uuid::new_v4()),
    )
}

#[tokio::test]
async fn activate_forbids_person_who_does_not_own_profile() {
    let _guard = TOKEN_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    let profile = business_profile(1);
    let profile_uuid = uuid_to_string(profile.uuid);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![profile]])
        .append_query_results(vec![vec![person(2)]])
        .into_connection();

    let result = SwitchBusinessProfile::activate(
        &db,
        &user(2, "attacker@example.com"),
        profile_uuid,
        "some-jti".to_string(),
        Utc::now().timestamp() + 3600,
    )
    .await;

    assert!(matches!(result, Err(SwitchBusinessProfileError::Forbidden)));
}

#[tokio::test]
async fn activate_succeeds_for_profile_owner() {
    let _guard = TOKEN_ENV_LOCK.lock().await;
    clear_auth_toggle_env();
    env::set_var("TOKEN_REVOCATION_ENABLED", "false");
    env::set_var("ACCESS_TOKEN_SECRET", "test_secret_switch_activate");
    env::set_var(
        "REFRESH_TOKEN_SECRET",
        "test_refresh_secret_switch_activate",
    );
    let profile = business_profile(1);
    let profile_uuid = uuid_to_string(profile.uuid);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![profile]])
        .append_query_results(vec![vec![person(1)]])
        .into_connection();

    let result = SwitchBusinessProfile::activate(
        &db,
        &user(1, "owner@example.com"),
        profile_uuid,
        "some-jti".to_string(),
        Utc::now().timestamp() + 3600,
    )
    .await;

    clear_auth_toggle_env();
    assert!(result.is_ok());
}
