use business::gateway::person_address_gateway::PersonAddressGateway;
use business::gateway::exercise_gateway::ExerciseGateway;
use business::commons::legal_documents;
use business::use_cases::consent_use_case::ConsentUseCase;
use business::use_cases::friend_use_case::FriendUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database};

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn radius_search_uses_current_indexable_geography_points() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    db.execute_unprepared(
        r#"INSERT INTO person
             (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
           VALUES
             (1, '00000000-0000-0000-0000-000000000001', 'Center', 'Person', '1990-01-01', 'X', now(), now()),
             (2, '00000000-0000-0000-0000-000000000002', 'Near', 'Person', '1990-01-01', 'X', now(), now()),
             (3, '00000000-0000-0000-0000-000000000003', 'Far', 'Person', '1990-01-01', 'X', now(), now());
           INSERT INTO person_address
             (id, uuid, person_id, address_line1, locality, administrative_area,
              country_code, current, location, created_at, updated_at)
           VALUES
             (1, '10000000-0000-0000-0000-000000000001', 1, 'Center', 'Sao Paulo', 'SP', 'BR', true, ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)::geography, now(), now()),
             (2, '10000000-0000-0000-0000-000000000002', 2, 'Near', 'Sao Paulo', 'SP', 'BR', true, ST_SetSRID(ST_MakePoint(-46.6333, -23.5405), 4326)::geography, now(), now()),
             (3, '10000000-0000-0000-0000-000000000003', 3, 'Far', 'Elsewhere', 'SP', 'BR', true, ST_SetSRID(ST_MakePoint(-46.6333, -22.5505), 4326)::geography, now(), now()),
             (4, '10000000-0000-0000-0000-000000000004', 3, 'Old', 'Sao Paulo', 'SP', 'BR', false, ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)::geography, now(), now()),
             (5, '10000000-0000-0000-0000-000000000005', 3, 'Unknown', 'Sao Paulo', 'SP', 'BR', true, NULL, now(), now())"#,
    )
    .await
    .unwrap();

    let mut ids: Vec<i32> = PersonAddressGateway::find_all_within_radius(&db, 1, 5.0)
        .await
        .unwrap()
        .into_iter()
        .map(|address| address.id)
        .collect();
    ids.sort_unstable();

    assert_eq!(ids, vec![1, 2]);

    db.execute_unprepared(
        r#"INSERT INTO exercise
             (id, uuid, name, category, owner_id, owner_uuid, owner_name,
              sets, reps_or_duration, description, visibility, created_at, updated_at)
           VALUES
             (1, '20000000-0000-0000-0000-000000000001', 'Push Ups', 'Force',
              1, '00000000-0000-0000-0000-000000000001', 'Center Person',
              3, 12, NULL, 'Public', now(), now())"#,
    )
    .await
    .unwrap();

    let exercise = ExerciseGateway::find_by_uuid(
        &db,
        "20000000-0000-0000-0000-000000000001".to_string(),
    )
    .await
    .unwrap();
    assert_eq!(exercise.map(|model| model.id), Some(1));
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn c002_profile_owner_and_health_consent_acceptance() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    db.execute_unprepared(
        r#"INSERT INTO person
             (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
           VALUES
             (1, '00000000-0000-0000-0000-000000000011', 'Profile', 'Owner', '1990-01-01', 'X', now(), now());
           INSERT INTO person_info
             (id, uuid, person_id, weight, height, created_at, updated_at)
           VALUES
             (1, '10000000-0000-0000-0000-000000000011', 1, 80.0, 180.0, now(), now())"#,
    )
    .await
    .unwrap();

    let person = PersonUseCase::get(&db, 1).await.unwrap();
    assert_eq!(person.id, Some(1));
    assert!(PersonUseCase::require_owner_access(1, 1).is_ok());
    assert!(PersonUseCase::require_owner_access(1, 2).is_err());

    assert!(ConsentUseCase::require_current(&db, 1, legal_documents::HEALTH_DATA)
        .await
        .is_err());

    db.execute_unprepared(
        r#"INSERT INTO consent
             (uuid, person_id, document, version, accepted_at, ip)
           VALUES
             ('20000000-0000-0000-0000-000000000011', 1, 'health_data', '1.0.0', now(), '127.0.0.1')"#,
    )
    .await
    .unwrap();

    assert!(ConsentUseCase::require_current(&db, 1, legal_documents::HEALTH_DATA)
        .await
        .is_ok());
}

  #[tokio::test]
  #[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
  async fn c003_friendship_lifecycle_and_owner_scope_acceptance() {
    let database_url = std::env::var("TEST_DATABASE_URL")
      .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    db.execute_unprepared(
      r#"INSERT INTO person
         (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
         VALUES
         (1, '00000000-0000-0000-0000-000000000021', 'Sender', 'Person', '1990-01-01', 'X', now(), now()),
         (2, '00000000-0000-0000-0000-000000000022', 'Receiver', 'Person', '1990-01-01', 'X', now(), now()),
         (3, '00000000-0000-0000-0000-000000000023', 'Unrelated', 'Person', '1990-01-01', 'X', now(), now())"#,
    )
    .await
    .unwrap();

    let pending = FriendUseCase::send_friend_request(&db, 1, 2).await.unwrap();
    assert_eq!(pending.status.as_str(), "Pending");
    assert!(FriendUseCase::accept_friend_request(&db, 2, 1)
      .await
      .is_ok());
    assert!(FriendUseCase::ensure_accepted_friend(&db, 1, 2)
      .await
      .is_ok());
    assert!(FriendUseCase::ensure_accepted_friend(&db, 1, 3)
      .await
      .is_err());

    let friends = FriendUseCase::find_all_friend(&db, 1).await.unwrap();
    assert_eq!(friends.len(), 1);
    assert_eq!(friends[0].friend_id, 2);

    FriendUseCase::remove_friend(&db, 1, 2).await.unwrap();
    assert!(FriendUseCase::find_all_friend(&db, 1)
      .await
      .unwrap()
      .is_empty());
  }

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn c004_workout_exercise_visibility_acceptance() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    db.execute_unprepared(
        r#"INSERT INTO person
             (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
           VALUES
             (1, '00000000-0000-0000-0000-000000000031', 'Workout', 'Owner', '1990-01-01', 'X', now(), now()),
             (2, '00000000-0000-0000-0000-000000000032', 'Other', 'Person', '1990-01-01', 'X', now(), now());
           INSERT INTO workout
             (id, uuid, owner_id, owner_uuid, name, description, difficulty, muscle_group, visibility, status, created_at, updated_at)
           VALUES
             (1, '10000000-0000-0000-0000-000000000031', 1, '00000000-0000-0000-0000-000000000031', 'Public Workout', 'Tracked workout', 'Easy', 'Chest', 'public', 'Accepted', now(), now());
           INSERT INTO exercise
             (id, uuid, name, category, owner_id, owner_uuid, owner_name, sets, reps_or_duration, description, visibility, created_at, updated_at)
           VALUES
             (1, '20000000-0000-0000-0000-000000000031', 'Private Exercise', 'Force', 1, '00000000-0000-0000-0000-000000000031', 'Workout Owner', 3, 10, 'Private', 'private', now(), now());
           INSERT INTO workout_exercise
             (id, uuid, workout_id, exercise_id, order_index, created_at, updated_at)
           VALUES
             (1, '30000000-0000-0000-0000-000000000031', 1, 1, 0, now(), now())"#,
    )
    .await
    .unwrap();

    let workout = WorkoutUseCase::get(&db, 1).await.unwrap();
    assert_eq!(workout.name, "Public Workout");
    let exercises = ExerciseUseCase::find_all_by_workout_id(&db, 1).await.unwrap();
    assert_eq!(exercises.len(), 1);
    assert!(ExerciseUseCase::ensure_all_readable(&exercises, 1).is_ok());
    assert!(ExerciseUseCase::ensure_all_readable(&exercises, 2).is_err());
}
