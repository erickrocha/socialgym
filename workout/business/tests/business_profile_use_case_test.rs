use business::commons::functions::uuid_to_string;
use business::domain::business_error::BusinessErrorKind;
use business::domain::business_profile::BusinessProfile;
use business::domain::enums::ProfileType;
use business::domain::user::User;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use entity::business_profile_address_entity::BusinessProfileAddressEntity;
use entity::business_profile_entity::BusinessProfileEntity;
use sea_orm::{DatabaseBackend, MockDatabase};
use uuid::Uuid;

fn business_profile_entity(owner_id: i32) -> BusinessProfileEntity {
    BusinessProfileEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        owner_id,
        owner_uuid: Uuid::new_v4(),
        tax_id: "12345".to_string(),
        business_name: "Gym XYZ".to_string(),
        business_type: "Gym".to_string(),
        social_name: None,
        logo: None,
        cover_image: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
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
async fn get_by_owner_id_returns_profiles_and_addresses() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![business_profile_entity(1)]])
        .append_query_results(vec![Vec::<BusinessProfileAddressEntity>::new()])
        .into_connection();

    let profiles = BusinessProfileUseCase::get_by_owner_id(&db, 1)
        .await
        .unwrap();

    assert_eq!(profiles.len(), 1);
    assert_eq!(profiles[0].business_name, "Gym XYZ");
}

#[tokio::test]
async fn delete_forbids_non_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![business_profile_entity(1)]])
        .into_connection();

    let error = BusinessProfileUseCase::delete(&db, 1, &user(2, "attacker@example.com"))
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}

#[tokio::test]
async fn update_forbids_non_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![business_profile_entity(1)]])
        .append_query_results(vec![Vec::<BusinessProfileAddressEntity>::new()])
        .into_connection();
    let mut profile = BusinessProfile::new(
        2,
        uuid_to_string(Uuid::new_v4()),
        "12345".to_string(),
        "Renamed Gym".to_string(),
        ProfileType::Professional,
        None,
    );
    profile.id = Some(1);

    let error = BusinessProfileUseCase::update(&db, profile, &user(2, "attacker@example.com"))
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}

#[tokio::test]
async fn discover_returns_empty_without_search_filters() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let profiles = BusinessProfileUseCase::discover(&db, None, None, None, None, None, 50)
        .await
        .unwrap();

    assert!(profiles.is_empty());
}

#[tokio::test]
async fn discover_ignores_out_of_range_coordinates_without_querying() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let profiles =
        BusinessProfileUseCase::discover(&db, None, None, Some(999.0), Some(999.0), None, 50)
            .await
            .unwrap();

    assert!(profiles.is_empty());
}
