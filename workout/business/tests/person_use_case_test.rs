use business::domain::person::Person;
use business::use_cases::person_use_case::PersonUseCase;
use chrono::NaiveDate;
use sea_orm::{DatabaseBackend, MockDatabase};

fn person(first_name: &str, surname: &str, gender: &str) -> Person {
    Person::update(
        None,
        first_name.to_string(),
        surname.to_string(),
        NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
        gender.to_string(),
        None,
        None,
        None,
        None,
        Vec::new(),
        Vec::new(),
    )
}

#[tokio::test]
async fn add_rejects_missing_first_name() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let error = PersonUseCase::add(&db, person("", "Doe", "M"))
        .await
        .unwrap_err();

    assert_eq!(error.message, "Person and User information are required");
}

#[tokio::test]
async fn add_rejects_missing_surname() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    assert!(PersonUseCase::add(&db, person("John", "", "M"))
        .await
        .is_err());
}

#[tokio::test]
async fn add_rejects_missing_gender() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    assert!(PersonUseCase::add(&db, person("John", "Doe", ""))
        .await
        .is_err());
}

#[test]
fn owner_access_allows_owner_and_rejects_another_person() {
    assert!(PersonUseCase::require_owner_access(7, 7).is_ok());
    assert!(PersonUseCase::require_owner_access(7, 8).is_err());
}

#[tokio::test]
async fn find_friends_returns_empty_without_filters() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let result = PersonUseCase::find_friends(&db, 1, None, None, None, None, 50).await;

    assert!(result.is_empty());
}

#[tokio::test]
async fn find_friends_returns_empty_for_invalid_coordinates_without_query() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    let result = PersonUseCase::find_friends(&db, 1, None, Some(200.0), Some(0.0), None, 50).await;

    assert!(result.is_empty());
}
