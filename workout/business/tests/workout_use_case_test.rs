use business::domain::business_error::BusinessErrorKind;
use business::domain::enums::{Difficulty, InviteStatus, Visibility};
use business::domain::user::User;
use business::domain::workout::Workout;
use business::use_cases::workout_use_case::WorkoutUseCase;
use entity::workout_entity::WorkoutEntity;
use entity::workout_exercise_entity::WorkoutExerciseEntity;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

fn workout_entity(
    uuid: Uuid,
    owner_id: i32,
    owner_uuid: Uuid,
    status: &str,
    assigned_by_profile_id: Option<i32>,
) -> WorkoutEntity {
    WorkoutEntity {
        id: 7,
        uuid,
        name: "Assigned plan".to_string(),
        description: None,
        difficulty: "Easy".to_string(),
        muscle_group: "Full body".to_string(),
        owner_id,
        owner_uuid,
        visibility: "Private".to_string(),
        status: status.to_string(),
        assigned_by_profile_id,
        assigned_by_profile_uuid: assigned_by_profile_id.map(|_| Uuid::new_v4()),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

fn pending_workout(
    uuid: Uuid,
    owner_id: i32,
    owner_uuid: Uuid,
    assigning_profile_id: i32,
) -> WorkoutEntity {
    workout_entity(
        uuid,
        owner_id,
        owner_uuid,
        "Pending",
        Some(assigning_profile_id),
    )
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
async fn get_returns_error_when_workout_is_missing() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<WorkoutEntity>::new()])
        .into_connection();

    assert!(WorkoutUseCase::get(&db, 999).await.is_err());
}

#[test]
fn owner_uuid_check_requires_authenticated_owner() {
    assert!(WorkoutUseCase::ensure_owner_uuid("owner-uuid", "owner-uuid").is_ok());
    assert_eq!(
        WorkoutUseCase::ensure_owner_uuid("owner-uuid", "other-uuid")
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );
}

#[test]
fn private_workout_is_readable_only_by_its_owner() {
    let private = Workout {
        id: Some(1),
        uuid: Some(Uuid::new_v4().to_string()),
        owner_id: 1,
        owner_uuid: Uuid::new_v4().to_string(),
        name: "Private workout".to_string(),
        description: None,
        difficulty: Difficulty::Easy,
        muscle_group: "Chest".to_string(),
        exercises: Vec::new(),
        visibility: Visibility::Private,
        status: InviteStatus::Accepted,
        assigned_by_profile_id: None,
        assigned_by_profile_uuid: None,
        created_at: None,
        updated_at: None,
    };

    assert!(WorkoutUseCase::ensure_readable(&private, 1).is_ok());
    assert_eq!(
        WorkoutUseCase::ensure_readable(&private, 2)
            .unwrap_err()
            .kind,
        BusinessErrorKind::Forbidden
    );

    let mut public = private;
    public.visibility = Visibility::Public;
    assert!(WorkoutUseCase::ensure_readable(&public, 2).is_ok());
}

#[tokio::test]
async fn persist_marks_self_created_workout_accepted() {
    let workout_uuid = Uuid::new_v4();
    let person_uuid = Uuid::new_v4();
    let mut persisted = workout_entity(workout_uuid, 1, person_uuid, "Accepted", None);
    persisted.name = "Push ups".to_string();
    persisted.description = Some("Basic push ups".to_string());
    persisted.visibility = "Public".to_string();
    persisted.muscle_group = "Chest".to_string();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![persisted]])
        .into_connection();
    let workout = Workout {
        id: None,
        uuid: None,
        owner_id: 1,
        owner_uuid: person_uuid.to_string(),
        name: "Push ups".to_string(),
        description: Some("Basic push ups".to_string()),
        difficulty: Difficulty::Easy,
        muscle_group: "Chest".to_string(),
        exercises: Vec::new(),
        visibility: Visibility::Public,
        status: InviteStatus::Pending,
        assigned_by_profile_id: None,
        assigned_by_profile_uuid: None,
        created_at: None,
        updated_at: None,
    };

    let saved =
        WorkoutUseCase::persist(&db, workout, &actor(1, person_uuid.to_string()), None, None)
            .await
            .unwrap();

    assert_eq!(saved.name, "Push ups");
    assert_eq!(saved.status, InviteStatus::Accepted);
}

#[tokio::test]
async fn find_all_assigned_by_profile_returns_assignment_statuses() {
    let first = pending_workout(Uuid::new_v4(), 42, Uuid::new_v4(), 10);
    let mut second = pending_workout(Uuid::new_v4(), 43, Uuid::new_v4(), 10);
    second.id = 8;
    second.status = "Accepted".to_string();
    let no_exercises = Vec::<WorkoutExerciseEntity>::new();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![first, second]])
        .append_query_results(vec![no_exercises.clone(), no_exercises])
        .into_connection();

    let workouts = WorkoutUseCase::find_all_assigned_by_profile(&db, 10)
        .await
        .unwrap();

    assert_eq!(workouts.len(), 2);
    assert_eq!(workouts[1].status, InviteStatus::Accepted);
}

#[tokio::test]
async fn accept_assignment_transitions_pending_to_accepted() {
    let uuid = Uuid::new_v4();
    let pending = pending_workout(uuid, 42, Uuid::new_v4(), 10);
    let mut accepted = pending.clone();
    accepted.status = "Accepted".to_string();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![pending.clone()], vec![pending], vec![accepted]])
        .append_query_results(vec![Vec::<WorkoutExerciseEntity>::new()])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 7,
            rows_affected: 1,
        }])
        .into_connection();

    let accepted = WorkoutUseCase::accept_assignment(&db, uuid.to_string(), 42)
        .await
        .unwrap();

    assert_eq!(accepted.status, InviteStatus::Accepted);
}

#[tokio::test]
async fn accept_assignment_forbids_another_person() {
    let uuid = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![pending_workout(uuid, 42, Uuid::new_v4(), 10)]])
        .into_connection();

    let error = WorkoutUseCase::accept_assignment(&db, uuid.to_string(), 999)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}

#[tokio::test]
async fn cancel_assignment_allows_assigning_profile() {
    let uuid = Uuid::new_v4();
    let pending = pending_workout(uuid, 42, Uuid::new_v4(), 10);
    let mut cancelled = pending.clone();
    cancelled.status = "Cancelled".to_string();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![pending.clone()], vec![pending], vec![cancelled]])
        .append_query_results(vec![Vec::<WorkoutExerciseEntity>::new()])
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 7,
            rows_affected: 1,
        }])
        .into_connection();

    let cancelled = WorkoutUseCase::cancel_assignment(&db, uuid.to_string(), 10)
        .await
        .unwrap();

    assert_eq!(cancelled.status, InviteStatus::Cancelled);
}

#[tokio::test]
async fn cancel_assignment_forbids_another_profile() {
    let uuid = Uuid::new_v4();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![pending_workout(uuid, 42, Uuid::new_v4(), 10)]])
        .into_connection();

    let error = WorkoutUseCase::cancel_assignment(&db, uuid.to_string(), 99)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}

#[tokio::test]
async fn assignment_transition_requires_pending_status() {
    let uuid = Uuid::new_v4();
    let mut accepted = pending_workout(uuid, 42, Uuid::new_v4(), 10);
    accepted.status = "Accepted".to_string();
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![accepted]])
        .into_connection();

    let error = WorkoutUseCase::accept_assignment(&db, uuid.to_string(), 42)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Validation);
}
