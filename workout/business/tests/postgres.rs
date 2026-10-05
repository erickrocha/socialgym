use business::gateway::person_address_gateway::PersonAddressGateway;
use business::gateway::exercise_gateway::ExerciseGateway;
use business::commons::legal_documents;
use business::domain::enums::ProfileType;
use business::domain::user::User;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use business::use_cases::consent_use_case::ConsentUseCase;
use business::use_cases::friend_use_case::FriendUseCase;
use business::gateway::aws_clients::sqs_client;
use business::use_cases::friendship_outbox_publisher_use_case::FriendshipOutboxPublisherUseCase;
use business::use_cases::person_use_case::PersonUseCase;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;
use log::{Level, LevelFilter, Log, Metadata, Record};
use migration::{Migrator, MigratorTrait};
use sea_orm::{ColumnTrait, ConnectionTrait, Database, EntityTrait, QueryFilter, QueryOrder, Statement};
use std::sync::Mutex;

struct AcceptanceLogCapture;

static ACCEPTANCE_LOGS: Mutex<Vec<String>> = Mutex::new(Vec::new());

impl Log for AcceptanceLogCapture {
  fn enabled(&self, metadata: &Metadata<'_>) -> bool {
    metadata.level() <= Level::Error
  }

  fn log(&self, record: &Record<'_>) {
    if self.enabled(record.metadata()) {
      ACCEPTANCE_LOGS
        .lock()
        .unwrap()
        .push(record.args().to_string());
    }
  }

  fn flush(&self) {}
}

static ACCEPTANCE_LOGGER: AcceptanceLogCapture = AcceptanceLogCapture;

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

#[tokio::test]
#[ignore = "requires disposable TEST_DATABASE_URL PostgreSQL/PostGIS and LocalStack SQS"]
async fn c006_friendship_outbox_publishes_fifo_in_order() {
  use sea_orm::{ActiveModelTrait, Set};

  log::set_logger(&ACCEPTANCE_LOGGER).expect("acceptance logger is not already installed");
  log::set_max_level(LevelFilter::Error);
  ACCEPTANCE_LOGS.lock().unwrap().clear();

  let database_url = std::env::var("TEST_DATABASE_URL")
    .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
  let queue_url = std::env::var("AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL")
    .expect("AWS_FRIENDSHIP_NOTIFICATION_QUEUE_URL must point to the test FIFO queue");
  let db = Database::connect(database_url).await.unwrap();
  Migrator::refresh(&db).await.unwrap();
  let client = sqs_client().await;
  let mut queue_ready = false;
  for _ in 0..30 {
    if client
      .get_queue_url()
      .queue_name("social-notification-events.fifo")
      .send()
      .await
      .is_ok()
    {
      queue_ready = true;
      break;
    }
    tokio::time::sleep(std::time::Duration::from_secs(1)).await;
  }
  assert!(queue_ready, "LocalStack friendship FIFO queue did not become ready");

  db.execute_unprepared(
    r#"INSERT INTO person
       (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
       VALUES
       (1, '00000000-0000-0000-0000-000000000061', 'Sender', 'Person', '1990-01-01', 'X', now(), now()),
       (2, '00000000-0000-0000-0000-000000000062', 'Receiver', 'Person', '1990-01-01', 'X', now(), now());
       INSERT INTO "user"
         (id, uuid, name, email, password, first_login, enabled, person_id, person_uuid, created_at, updated_at)
       VALUES
         (1, '10000000-0000-0000-0000-000000000061', 'Sender Person', 'c006-sender@example.test', 'unused', false, true, 1, '00000000-0000-0000-0000-000000000061', now(), now());
       INSERT INTO settings
         (id, uuid, person_id, person_uuid, language, theme, notifications_enabled, context_menu_position, home_page, created_at, updated_at)
       VALUES
         (1, '20000000-0000-0000-0000-000000000061', 1, '00000000-0000-0000-0000-000000000061', 'en', 'light', true, 'left', 'feed', now(), now()),
         (2, '20000000-0000-0000-0000-000000000062', 2, '00000000-0000-0000-0000-000000000062', 'en', 'light', false, 'left', 'feed', now(), now());
       INSERT INTO consent (uuid, person_id, document, version, accepted_at, ip)
       VALUES
         ('30000000-0000-0000-0000-000000000061', 1, 'terms', '1.0.0', now(), '127.0.0.1'),
         ('30000000-0000-0000-0000-000000000062', 1, 'privacy', '1.0.0', now(), '127.0.0.1')"#,
  )
  .await
  .unwrap();

  let pending = FriendUseCase::send_friend_request(&db, 1, 2).await.unwrap();
  FriendUseCase::accept_friend_request(&db, 2, 1)
    .await
    .unwrap();
  let friendship_uuid = pending.uuid.unwrap();
  let events = entity::friendship_notification_outbox_entity::Entity::find()
    .filter(
      entity::friendship_notification_outbox_entity::Column::FriendshipUuid
        .eq(uuid::Uuid::parse_str(&friendship_uuid).unwrap()),
    )
    .order_by_asc(entity::friendship_notification_outbox_entity::Column::Id)
    .all(&db)
    .await
    .unwrap();
  assert_eq!(events.len(), 2);
  assert_eq!(events[0].event_type, "friend_request_created");
  assert_eq!(events[1].event_type, "friend_request_accepted");

  // Independent of the queue's name, so a dedicated acceptance queue can be used.
  let (queue_base, queue_name) = queue_url.rsplit_once('/').unwrap();
  let unavailable_queue_url = format!("{queue_base}/missing-{queue_name}");
  for expected_attempt in 1..=5 {
    assert_eq!(
      FriendshipOutboxPublisherUseCase::publish_pending(
        &db,
        &client,
        &unavailable_queue_url,
      )
      .await
      .unwrap(),
      0
    );
    let failed = entity::friendship_notification_outbox_entity::Entity::find()
      .filter(
        entity::friendship_notification_outbox_entity::Column::EventUuid
          .eq(events[0].event_uuid),
      )
      .one(&db)
      .await
      .unwrap()
      .unwrap();
    assert_eq!(failed.attempt_count, expected_attempt);
    assert!(failed.last_error.is_some());
    assert!(failed.published_at.is_none());

    let mut retry: entity::friendship_notification_outbox_entity::ActiveModel = failed.into();
    retry.next_attempt_at = Set(chrono::Utc::now() - chrono::Duration::seconds(1));
    retry.update(&db).await.unwrap();
  }
  assert!(ACCEPTANCE_LOGS.lock().unwrap().iter().any(|message| {
    message.contains("ALERT: friendship event has failed to publish five consecutive times")
      && message.contains(&events[0].event_uuid.to_string())
  }));

  let accepted_event = entity::friendship_notification_outbox_entity::Entity::find()
    .filter(
      entity::friendship_notification_outbox_entity::Column::EventUuid
        .eq(events[1].event_uuid),
    )
    .one(&db)
    .await
    .unwrap()
    .unwrap();
  assert_eq!(accepted_event.attempt_count, 0);
  assert!(accepted_event.published_at.is_none());

  let published = FriendshipOutboxPublisherUseCase::publish_pending(
    &db,
    &client,
    &queue_url,
  )
  .await
  .unwrap();
  assert_eq!(published, 2);

  let published_events = entity::friendship_notification_outbox_entity::Entity::find()
    .filter(
      entity::friendship_notification_outbox_entity::Column::FriendshipUuid
        .eq(uuid::Uuid::parse_str(&friendship_uuid).unwrap()),
    )
    .order_by_asc(entity::friendship_notification_outbox_entity::Column::Id)
    .all(&db)
    .await
    .unwrap();
  assert!(published_events.iter().all(|event| event.published_at.is_some()));
  assert_eq!(published_events[0].attempt_count, 5);

  let messages = client
    .receive_message()
    .queue_url(&queue_url)
    .max_number_of_messages(10)
    .wait_time_seconds(1)
    .send()
    .await
    .unwrap()
    .messages
    .unwrap_or_default();
  // Only this run's friendship: the queue may still hold events left by an earlier run.
  let event_types: Vec<String> = messages
    .iter()
    .filter_map(|message| message.body())
    .map(|body| serde_json::from_str::<serde_json::Value>(body).unwrap())
    .filter(|event| event["friendshipUuid"].as_str() == Some(friendship_uuid.as_str()))
    .map(|event| event["eventType"].as_str().unwrap().to_string())
    .collect();
  assert_eq!(
    event_types,
    vec!["friend_request_created", "friend_request_accepted"]
  );

  for message in messages {
    if let Some(receipt_handle) = message.receipt_handle() {
      client
        .delete_message()
        .queue_url(&queue_url)
        .receipt_handle(receipt_handle)
        .send()
        .await
        .unwrap();
    }
  }
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn c006_rejected_and_cancelled_friendships_do_not_emit_transition_events() {
  let database_url = std::env::var("TEST_DATABASE_URL")
    .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
  let db = Database::connect(database_url).await.unwrap();
  Migrator::refresh(&db).await.unwrap();

  db.execute_unprepared(
    r#"INSERT INTO person
       (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
       VALUES
       (1, '00000000-0000-0000-0000-000000000071', 'Reject', 'Sender', '1990-01-01', 'X', now(), now()),
       (2, '00000000-0000-0000-0000-000000000072', 'Reject', 'Receiver', '1990-01-01', 'X', now(), now()),
       (3, '00000000-0000-0000-0000-000000000073', 'Cancel', 'Sender', '1990-01-01', 'X', now(), now()),
       (4, '00000000-0000-0000-0000-000000000074', 'Cancel', 'Receiver', '1990-01-01', 'X', now(), now())"#,
  )
  .await
  .unwrap();

  let rejected = FriendUseCase::send_friend_request(&db, 1, 2)
    .await
    .unwrap();
  FriendUseCase::deny_friend_request(&db, 2, 1)
    .await
    .unwrap();
  let cancelled = FriendUseCase::send_friend_request(&db, 3, 4)
    .await
    .unwrap();
  FriendUseCase::cancel_friend_request(&db, 3, 4)
    .await
    .unwrap();

  for friendship_uuid in [rejected.uuid.unwrap(), cancelled.uuid.unwrap()] {
    let events = entity::friendship_notification_outbox_entity::Entity::find()
      .filter(
        entity::friendship_notification_outbox_entity::Column::FriendshipUuid
          .eq(uuid::Uuid::parse_str(&friendship_uuid).unwrap()),
      )
      .all(&db)
      .await
      .unwrap();
    assert_eq!(events.len(), 1);
    assert_eq!(events[0].event_type, "friend_request_created");
  }
}

fn business_profile_owner_actor(person_id: i32, person_uuid: &str) -> User {
    User::update(
        None,
        None,
        None,
        format!("owner{person_id}@example.com"),
        "irrelevant".to_string(),
        true,
        false,
        person_id,
        person_uuid.to_string(),
    )
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn c007_business_profile_delete_cascades_addresses_and_team_members() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    db.execute_unprepared(
        r#"INSERT INTO person
             (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
           VALUES
             (1, '00000000-0000-0000-0000-000000000041', 'Owner', 'Person', '1990-01-01', 'X', now(), now()),
             (2, '00000000-0000-0000-0000-000000000042', 'Other', 'Person', '1990-01-01', 'X', now(), now()),
             (3, '00000000-0000-0000-0000-000000000043', 'Team', 'Member', '1990-01-01', 'X', now(), now());
           INSERT INTO business_profile
             (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
           VALUES
             (1, '10000000-0000-0000-0000-000000000041', 1, '00000000-0000-0000-0000-000000000041', '123', 'Owner Gym', 'Professional', now(), now());
           INSERT INTO business_profile_address
             (id, uuid, business_profile_id, address_line1, locality, administrative_area, country_code, created_at, updated_at)
           VALUES
             (1, '20000000-0000-0000-0000-000000000041', 1, 'Main St', 'Sao Paulo', 'SP', 'BR', now(), now());
           INSERT INTO team_members
             (id, uuid, business_profile_id, business_profile_uuid, person_id, person_uuid, status, created_at, updated_at)
           VALUES
             (1, '30000000-0000-0000-0000-000000000041', 1, '10000000-0000-0000-0000-000000000041', 3, '00000000-0000-0000-0000-000000000043', 'Accepted', now(), now());
           INSERT INTO profile
             (id, uuid, person_id, person_uuid, business_profile_id, business_profile_uuid, created_at, updated_at)
           VALUES
             (1, '40000000-0000-0000-0000-000000000041', 1, '00000000-0000-0000-0000-000000000041', 1, '10000000-0000-0000-0000-000000000041', now(), now())"#,
    )
    .await
    .unwrap();

    // Non-owner cannot delete.
    let non_owner = business_profile_owner_actor(2, "00000000-0000-0000-0000-000000000042");
    assert!(BusinessProfileUseCase::delete(&db, 1, &non_owner).await.is_err());

    // Owner deletion cascades every related row in a single transaction.
    let owner = business_profile_owner_actor(1, "00000000-0000-0000-0000-000000000041");
    BusinessProfileUseCase::delete(&db, 1, &owner).await.unwrap();

    assert!(BusinessProfileUseCase::get_by_id(&db, 1).await.is_none());

    for (table, sql) in [
        ("business_profile_address", "SELECT COUNT(*) FROM business_profile_address WHERE business_profile_id = 1"),
        ("team_members", "SELECT COUNT(*) FROM team_members WHERE business_profile_id = 1"),
        ("profile", "SELECT COUNT(*) FROM profile WHERE business_profile_id = 1"),
    ] {
        let row = db
            .query_one_raw(Statement::from_string(
                sea_orm::DatabaseBackend::Postgres,
                sql,
            ))
            .await
            .unwrap()
            .unwrap();
        let count: i64 = row.try_get("", "count").unwrap();
        assert_eq!(count, 0, "{table} rows should be cascade-deleted");
    }
}

#[tokio::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn c007_business_profile_discover_combines_text_and_location_filters() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    db.execute_unprepared(
        r#"INSERT INTO person
             (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
           VALUES
             (1, '00000000-0000-0000-0000-000000000051', 'Owner', 'One', '1990-01-01', 'X', now(), now()),
             (2, '00000000-0000-0000-0000-000000000052', 'Owner', 'Two', '1990-01-01', 'X', now(), now()),
             (3, '00000000-0000-0000-0000-000000000053', 'Owner', 'Three', '1990-01-01', 'X', now(), now());
           -- Matches text query and is near the search point.
           INSERT INTO business_profile
             (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
           VALUES
             (1, '10000000-0000-0000-0000-000000000051', 1, '00000000-0000-0000-0000-000000000051', '111', 'Downtown Fitness Studio', 'Professional', now(), now()),
             (2, '10000000-0000-0000-0000-000000000052', 2, '00000000-0000-0000-0000-000000000052', '222', 'Downtown Fitness Studio', 'Company', now(), now()),
             (3, '10000000-0000-0000-0000-000000000053', 3, '00000000-0000-0000-0000-000000000053', '333', 'Far Away Studio', 'Professional', now(), now());
           INSERT INTO business_profile_address
             (id, uuid, business_profile_id, address_line1, locality, administrative_area, country_code, created_at, updated_at, location)
           VALUES
             (1, '20000000-0000-0000-0000-000000000051', 1, 'Near St', 'Sao Paulo', 'SP', 'BR', now(), now(), ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)::geography),
             (2, '20000000-0000-0000-0000-000000000052', 2, 'Near St', 'Sao Paulo', 'SP', 'BR', now(), now(), ST_SetSRID(ST_MakePoint(-46.6333, -23.5505), 4326)::geography),
             (3, '20000000-0000-0000-0000-000000000053', 3, 'Far St', 'Elsewhere', 'SP', 'BR', now(), now(), ST_SetSRID(ST_MakePoint(-46.6333, -22.5505), 4326)::geography)"#,
    )
    .await
    .unwrap();

    // Text query alone: matches both "Downtown Fitness Studio" profiles regardless of location.
    let by_text = BusinessProfileUseCase::discover(
        &db,
        Some("Downtown".to_string()),
        None,
        None,
        None,
        None,
        50,
    )
    .await
    .unwrap();
    let mut text_ids: Vec<i32> = by_text.iter().filter_map(|p| p.id).collect();
    text_ids.sort();
    assert_eq!(text_ids, vec![1, 2]);

    // Text + business_type filter narrows to the Professional one.
    let by_text_and_type = BusinessProfileUseCase::discover(
        &db,
        Some("Downtown".to_string()),
        Some(ProfileType::Professional),
        None,
        None,
        None,
        50,
    )
    .await
    .unwrap();
    assert_eq!(
        by_text_and_type
            .iter()
            .filter_map(|p| p.id)
            .collect::<Vec<i32>>(),
        vec![1]
    );

    // Text + location: intersects to only the nearby "Downtown" profiles, excluding the far one.
    let by_text_and_location = BusinessProfileUseCase::discover(
        &db,
        Some("Studio".to_string()),
        None,
        Some(-23.5505),
        Some(-46.6333),
        Some(5.0),
        50,
    )
    .await
    .unwrap();
    let mut combined_ids: Vec<i32> = by_text_and_location.iter().filter_map(|p| p.id).collect();
    combined_ids.sort();
    assert_eq!(combined_ids, vec![1, 2]);

    // No text and no location: never returns everything.
    let empty = BusinessProfileUseCase::discover(&db, None, None, None, None, None, 50)
        .await
        .unwrap();
    assert!(empty.is_empty());
}
