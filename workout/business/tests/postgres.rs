use business::gateway::person_address_gateway::PersonAddressGateway;
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
              country_code, current, latitude, longitude, created_at, updated_at)
           VALUES
             (1, '10000000-0000-0000-0000-000000000001', 1, 'Center', 'Sao Paulo', 'SP', 'BR', true, -23.5505, -46.6333, now(), now()),
             (2, '10000000-0000-0000-0000-000000000002', 2, 'Near', 'Sao Paulo', 'SP', 'BR', true, -23.5405, -46.6333, now(), now()),
             (3, '10000000-0000-0000-0000-000000000003', 3, 'Far', 'Elsewhere', 'SP', 'BR', true, -22.5505, -46.6333, now(), now()),
             (4, '10000000-0000-0000-0000-000000000004', 3, 'Old', 'Sao Paulo', 'SP', 'BR', false, -23.5505, -46.6333, now(), now()),
             (5, '10000000-0000-0000-0000-000000000005', 3, 'Unknown', 'Sao Paulo', 'SP', 'BR', true, NULL, NULL, now(), now())"#,
    )
    .await
    .unwrap();

    let mut ids: Vec<i32> =
        PersonAddressGateway::find_all_within_radius(&db, -23.5505, -46.6333, 5.0)
            .await
            .unwrap()
            .into_iter()
            .map(|address| address.id)
            .collect();
    ids.sort_unstable();

    assert_eq!(ids, vec![1, 2]);
}
