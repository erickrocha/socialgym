use business::commons::entity_mapper::EntityMapper;
use business::commons::functions::uuid_to_string;
use business::domain::business_error::BusinessErrorKind;
use business::domain::enums::{Category, Visibility};
use business::domain::exercise::{Exercise, ExerciseEntityMapper};
use business::domain::user::User;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use entity::exercise_entity::ExerciseEntity;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

fn exercise_entity(owner_id: i32, visibility: &str) -> ExerciseEntity {
    ExerciseEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        name: "Push Ups".to_string(),
        description: Some("Standard push ups".to_string()),
        owner_id,
        owner_uuid: Uuid::new_v4(),
        owner_name: "John Doe".to_string(),
        sets: 3,
        category: "Force".to_string(),
        reps_or_duration: 20,
        visibility: visibility.to_string(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

fn actor(person_id: i32, person_uuid: String) -> User {
    User::new(
        Some("Test Actor".to_string()),
        "actor@example.com".to_string(),
        "hashed".to_string(),
        person_id,
        person_uuid,
    )
}

#[tokio::test]
async fn persist_creates_exercise_for_authenticated_owner() {
    let owner_uuid = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![exercise_entity(1, "Public")]])
        .into_connection();
    let exercise = Exercise {
        id: None,
        uuid: None,
        name: "Push Ups".to_string(),
        description: Some("Standard push ups".to_string()),
        sets: 3,
        owner_id: 1,
        owner_uuid: uuid_to_string(owner_uuid),
        owner_name: "John Doe".to_string(),
        category: Category::Force,
        reps_or_duration: 20,
        visibility: Visibility::Public,
        created_at: None,
        updated_at: None,
    };

    let saved = ExerciseUseCase::persist(&db, exercise, &actor(1, owner_uuid.to_string()), None)
        .await
        .unwrap();

    assert_eq!(saved.name, "Push Ups");
    assert_eq!(saved.owner_id, 1);
}

#[tokio::test]
async fn get_returns_exercise_fields() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![exercise_entity(1, "Public")]])
        .into_connection();

    let exercise = ExerciseUseCase::get(&db, 1).await.unwrap();

    assert_eq!(exercise.name, "Push Ups");
    assert_eq!(exercise.sets, 3);
}

#[tokio::test]
async fn find_by_visibility_returns_matching_exercises() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![exercise_entity(1, "Public")]])
        .into_connection();

    let exercises = ExerciseUseCase::find_by_visibility(&db, Visibility::Public, 1)
        .await
        .unwrap();

    assert_eq!(exercises.len(), 1);
    assert_eq!(exercises[0].visibility.to_string(), "public");
}

#[tokio::test]
async fn delete_allows_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![exercise_entity(1, "Public")]])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .into_connection();

    ExerciseUseCase::delete_by_id(&db, 1, 1).await.unwrap();
}

#[tokio::test]
async fn delete_forbids_non_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![exercise_entity(1, "Public")]])
        .into_connection();

    let error = ExerciseUseCase::delete_by_id(&db, 1, 2).await.unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}

#[tokio::test]
async fn persist_uses_authenticated_owner_instead_of_client_owner() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![exercise_entity(7, "Public")]])
        .into_connection();
    let mut exercise = ExerciseEntityMapper::from_model(exercise_entity(1, "Public"));
    exercise.id = None;
    exercise.uuid = None;

    let saved =
        ExerciseUseCase::persist(&db, exercise, &actor(7, Uuid::new_v4().to_string()), None)
            .await
            .unwrap();

    assert_eq!(saved.owner_id, 7);
}

#[test]
fn private_exercise_is_only_readable_by_owner() {
    let mut exercise = ExerciseEntityMapper::from_model(exercise_entity(1, "Private"));

    assert!(ExerciseUseCase::ensure_readable(&exercise, 1).is_ok());
    assert_eq!(
        ExerciseUseCase::ensure_readable(&exercise, 2)
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );

    exercise.visibility = Visibility::Public;
    assert!(ExerciseUseCase::ensure_readable(&exercise, 2).is_ok());
}

#[test]
fn collection_rejects_private_exercises_for_another_person() {
    let public = ExerciseEntityMapper::from_model(exercise_entity(1, "Public"));
    let private = ExerciseEntityMapper::from_model(exercise_entity(1, "Private"));

    let error = ExerciseUseCase::ensure_all_readable(&[public, private], 2).unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}
