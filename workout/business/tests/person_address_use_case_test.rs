use business::domain::business_error::BusinessErrorKind;
use business::domain::person_address::PersonAddress;
use business::use_cases::person_address_use_case::PersonAddressUseCase;
use entity::person_address_entity::PersonAddressEntity;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

fn person_address_entity(person_id: i32, locality: &str) -> PersonAddressEntity {
    PersonAddressEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        person_id,
        address_line1: "456 Oak Ave".to_string(),
        address_line2: None,
        locality: locality.to_string(),
        administrative_area: "MA".to_string(),
        country_code: "USA".to_string(),
        postal_code: Some("02101".to_string()),
        current: true,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

#[tokio::test]
async fn add_person_address_persists_current_address() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![person_address_entity(1, "Santa Catarina")]])
        .into_connection();
    let address = PersonAddress::new(
        1,
        "123 Main St".to_string(),
        Some("".to_string()),
        "Santa Catarina".to_string(),
        "BR".to_string(),
        Some("88058573".to_string()),
        "BR".to_string(),
        true,
    );

    let saved = PersonAddressUseCase::add_person_address(&db, address, None, None)
        .await
        .unwrap();

    assert_eq!(saved.locality, "Santa Catarina");
    assert!(saved.current);
}

#[tokio::test]
async fn update_person_address_returns_updated_address_for_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_address_entity(1, "Boston")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![person_address_entity(1, "Boston")]])
        .into_connection();
    let mut address = PersonAddress::new(
        1,
        "456 Oak Ave".to_string(),
        None,
        "Boston".to_string(),
        "USA".to_string(),
        Some("02101".to_string()),
        "MA".to_string(),
        true,
    );
    address.id = Some(1);

    let updated = PersonAddressUseCase::update_person_address(&db, address, 1, None, None)
        .await
        .unwrap();

    assert_eq!(updated.locality, "Boston");
}

#[tokio::test]
async fn delete_person_address_forbids_another_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_address_entity(1, "Boston")]])
        .into_connection();

    let error = PersonAddressUseCase::delete_person_address(&db, 1, 2)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}
