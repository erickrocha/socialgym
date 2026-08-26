use business::gateway::person_address_gateway::PersonAddressGateway;
use business::gateway::exercise_gateway::ExerciseGateway;
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
