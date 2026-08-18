use migration::{Migrator, MigratorTrait};
use sea_orm_migration::sea_orm::{ConnectionTrait, Database, DbBackend, Statement};

#[async_std::test]
#[ignore = "requires a dedicated TEST_DATABASE_URL PostgreSQL/PostGIS database"]
async fn fresh_schema_uses_native_postgres_and_postgis_types() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to a disposable PostgreSQL/PostGIS database");
    let db = Database::connect(database_url).await.unwrap();

    // `fresh` asks SeaORM to drop every visible table, including PostGIS-owned
    // `spatial_ref_sys`. Use the migration-aware refresh path instead.
    Migrator::up(&db, None).await.unwrap();
    Migrator::refresh(&db).await.unwrap();

    let postgis = db
        .query_one_raw(Statement::from_string(
            DbBackend::Postgres,
            "SELECT extversion FROM pg_extension WHERE extname = 'postgis'",
        ))
        .await
        .unwrap()
        .expect("postgis extension should exist");
    let _: String = postgis.try_get("", "extversion").unwrap();

    let columns = db
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"SELECT column_name, data_type, udt_name
               FROM information_schema.columns
               WHERE table_name IN ('person', 'person_address', 'user')
                 AND column_name IN ('uuid', 'created_at', 'location')"#,
        ))
        .await
        .unwrap();
    let column_types: Vec<(String, String, String)> = columns
        .into_iter()
        .map(|row| {
            (
                row.try_get("", "column_name").unwrap(),
                row.try_get("", "data_type").unwrap(),
                row.try_get("", "udt_name").unwrap(),
            )
        })
        .collect();
    assert!(
        column_types
            .iter()
            .any(|(_, data_type, udt)| data_type == "uuid" || udt == "uuid")
    );
    assert!(
        column_types
            .iter()
            .any(|(_, data_type, _)| data_type == "timestamp with time zone")
    );
    assert!(
        column_types
            .iter()
            .any(|(name, _, udt)| name == "location" && udt == "geography")
    );

    let indexes = db
        .query_all_raw(Statement::from_string(
            DbBackend::Postgres,
            r#"SELECT indexname FROM pg_indexes
               WHERE indexname IN ('idx_person_address_location',
                                   'idx_business_profile_address_location')"#,
        ))
        .await
        .unwrap();
    assert_eq!(indexes.len(), 2);
}
