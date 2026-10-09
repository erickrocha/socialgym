use business::commons::authorization::ActingOwner;
use business::domain::business_error::BusinessErrorKind;
use business::domain::enums::{Category, Difficulty, InviteStatus, Visibility};
use business::domain::exercise::Exercise;
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

fn acting(id: i32) -> ActingOwner {
    ActingOwner::person(id, Uuid::from_u128(id as u128).to_string())
}

fn workout_owned_by(owner_id: i32, visibility: Visibility) -> Workout {
    Workout {
        id: Some(1),
        uuid: Some(Uuid::new_v4().to_string()),
        owner_id,
        owner_uuid: Uuid::from_u128(owner_id as u128).to_string(),
        name: "Workout".to_string(),
        description: None,
        difficulty: Difficulty::Easy,
        muscle_group: "Chest".to_string(),
        exercises: Vec::new(),
        visibility,
        status: InviteStatus::Accepted,
        assigned_by_profile_id: None,
        assigned_by_profile_uuid: None,
        created_at: None,
        updated_at: None,
    }
}

fn exercise_owned_by(owner_id: i32, visibility: Visibility) -> Exercise {
    Exercise {
        id: Some(owner_id),
        uuid: Some(Uuid::new_v4().to_string()),
        name: "Push Ups".to_string(),
        description: None,
        sets: 3,
        owner_id,
        owner_uuid: Uuid::from_u128(owner_id as u128).to_string(),
        owner_name: "Owner".to_string(),
        category: Category::Force,
        reps_or_duration: 10,
        visibility,
        created_at: None,
        updated_at: None,
    }
}

fn empty_db() -> sea_orm::DatabaseConnection {
    MockDatabase::new(DatabaseBackend::Postgres).into_connection()
}

#[tokio::test]
async fn private_workout_is_readable_only_by_its_owner() {
    let db = empty_db();
    let private = workout_owned_by(1, Visibility::Private);

    assert!(WorkoutUseCase::ensure_readable(&db, &private, &acting(1))
        .await
        .is_ok());
    // Reported as not found, so it cannot be told apart from a missing workout.
    assert_eq!(
        WorkoutUseCase::ensure_readable(&db, &private, &acting(2))
            .await
            .unwrap_err()
            .kind,
        BusinessErrorKind::NotFound
    );
    // Same id, different identity (a Business Profile): not the owner.
    let profile = ActingOwner::profile(
        1,
        Uuid::from_u128(999).to_string(),
        5,
        Uuid::from_u128(5).to_string(),
    );
    assert!(WorkoutUseCase::ensure_readable(&db, &private, &profile)
        .await
        .is_err());

    let public = workout_owned_by(1, Visibility::Public);
    assert!(WorkoutUseCase::ensure_readable(&db, &public, &acting(2))
        .await
        .is_ok());
}

#[tokio::test]
async fn readable_by_hides_private_workouts_and_private_composed_exercises() {
    let db = empty_db();
    let mut public = workout_owned_by(1, Visibility::Public);
    public.exercises = vec![
        exercise_owned_by(1, Visibility::Public),
        exercise_owned_by(1, Visibility::Private),
    ];
    let private = workout_owned_by(1, Visibility::Private);

    // Another person sees the public workout, without the private exercise.
    let seen = WorkoutUseCase::readable_by(&db, vec![public.clone(), private.clone()], &acting(2))
        .await
        .unwrap();
    assert_eq!(seen.len(), 1);
    assert_eq!(seen[0].exercises.len(), 1);
    assert!(matches!(
        seen[0].exercises[0].visibility,
        Visibility::Public
    ));

    // The owner sees everything.
    let own = WorkoutUseCase::readable_by(&db, vec![public, private], &acting(1))
        .await
        .unwrap();
    assert_eq!(own.len(), 2);
    assert_eq!(own[0].exercises.len(), 2);
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

#[tokio::test]
async fn delete_requires_the_owner_and_reports_a_missing_row_as_not_found() {
    let owned = || workout_entity(Uuid::new_v4(), 1, Uuid::from_u128(1), "Accepted", None);
    let deleted = |rows_affected| MockExecResult {
        last_insert_id: 0,
        rows_affected,
    };

    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![owned()]])
        .append_exec_results(vec![deleted(1)])
        .into_connection();
    assert!(WorkoutUseCase::delete_by_id(&db, 7, &acting(1))
        .await
        .is_ok());

    // The row vanished between the lookup and the delete.
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![owned()]])
        .append_exec_results(vec![deleted(0)])
        .into_connection();
    let error = WorkoutUseCase::delete_by_id(&db, 7, &acting(1))
        .await
        .unwrap_err();
    assert_eq!(error.kind, BusinessErrorKind::NotFound);

    // A different identity, even with the same numeric id, is not the owner.
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![owned()]])
        .into_connection();
    let profile = ActingOwner::profile(
        1,
        Uuid::from_u128(999).to_string(),
        5,
        Uuid::from_u128(5).to_string(),
    );
    // A different identity cannot read the workout, so it is told the workout does not exist (W4).
    let error = WorkoutUseCase::delete_by_id(&db, 7, &profile)
        .await
        .unwrap_err();
    assert_eq!(error.kind, BusinessErrorKind::NotFound);
}

#[tokio::test]
async fn an_active_profile_keeps_access_to_its_own_persons_records_and_its_own() {
    let db = empty_db();
    // The caller is Person 1 acting as Business Profile 9.
    let acting = ActingOwner::profile(
        9,
        Uuid::from_u128(9).to_string(),
        1,
        Uuid::from_u128(1).to_string(),
    );

    let persons_private = workout_owned_by(1, Visibility::Private);
    let profiles_private = workout_owned_by(9, Visibility::Private);
    let someone_elses_private = workout_owned_by(2, Visibility::Private);

    assert!(
        WorkoutUseCase::ensure_readable(&db, &persons_private, &acting)
            .await
            .is_ok()
    );
    assert!(
        WorkoutUseCase::ensure_readable(&db, &profiles_private, &acting)
            .await
            .is_ok()
    );
    assert!(
        WorkoutUseCase::ensure_readable(&db, &someone_elses_private, &acting)
            .await
            .is_err()
    );

    // Listing the Person's workouts while acting as the profile still returns them all.
    let listed = WorkoutUseCase::readable_by(
        &db,
        vec![persons_private, profiles_private, someone_elses_private],
        &acting,
    )
    .await
    .unwrap();
    assert_eq!(listed.len(), 2);
}
