use business::domain::person_info::PersonInfo;
use business::use_cases::person_info_use_case::PersonInfoUseCase;
use entity::person_info_entity::PersonInfoEntity;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

fn person_info_entity(biography: &str) -> PersonInfoEntity {
    PersonInfoEntity {
        id: 1,
        person_id: 1,
        biography: Some(biography.to_string()),
        relationship: None,
        job: None,
        home_town: None,
        current_city: None,
        weight: None,
        height: None,
        uuid: Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

#[tokio::test]
async fn get_returns_person_info_for_existing_person() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_info_entity("Test bio")]])
        .into_connection();

    let person_info = PersonInfoUseCase::get(&db, 1).await.unwrap();

    assert_eq!(person_info.id, Some(1));
    assert_eq!(person_info.biography.as_deref(), Some("Test bio"));
}

#[tokio::test]
async fn get_returns_not_found_when_person_info_is_missing() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<PersonInfoEntity>::new()])
        .into_connection();

    let error = PersonInfoUseCase::get(&db, 999).await.unwrap_err();

    assert_eq!(error.message, "PersonInfo not found");
}

#[tokio::test]
async fn update_persists_and_returns_changed_biography() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_info_entity("Test bio")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![person_info_entity("Updated bio")]])
        .into_connection();
    let mut payload = PersonInfo::new(
        1,
        Some("Test bio".to_string()),
        None,
        None,
        None,
        None,
        None,
        None,
    );
    payload.biography = Some("Updated bio".to_string());

    let updated = PersonInfoUseCase::update(&db, 1, payload).await.unwrap();

    assert_eq!(updated.biography.as_deref(), Some("Updated bio"));
}
