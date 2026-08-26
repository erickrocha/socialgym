use sea_orm_migration::prelude::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("DROP INDEX IF EXISTS idx_person_address_location")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE person_address ADD COLUMN location_new geography(Point, 4326)",
        )
        .await?;
        db.execute_unprepared("UPDATE person_address SET location_new = location")
            .await?;
        db.execute_unprepared("ALTER TABLE person_address DROP COLUMN location")
            .await?;
        db.execute_unprepared("ALTER TABLE person_address DROP COLUMN latitude")
            .await?;
        db.execute_unprepared("ALTER TABLE person_address DROP COLUMN longitude")
            .await?;
        db.execute_unprepared(
            "ALTER TABLE person_address RENAME COLUMN location_new TO location",
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX idx_person_address_location ON person_address USING GIST (location)",
        )
        .await?;
        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        db.execute_unprepared("ALTER TABLE person_address ADD COLUMN latitude double precision")
            .await?;
        db.execute_unprepared("ALTER TABLE person_address ADD COLUMN longitude double precision")
            .await?;
        db.execute_unprepared(
            r#"UPDATE person_address
               SET latitude = ST_Y(location::geometry), longitude = ST_X(location::geometry)
               WHERE location IS NOT NULL"#,
        )
        .await?;
        db.execute_unprepared("DROP INDEX IF EXISTS idx_person_address_location")
            .await?;
        db.execute_unprepared("ALTER TABLE person_address DROP COLUMN location")
            .await?;
        db.execute_unprepared(
            r#"ALTER TABLE person_address
               ADD COLUMN location geography(Point, 4326)
               GENERATED ALWAYS AS (
                 CASE WHEN latitude IS NULL OR longitude IS NULL THEN NULL
                      ELSE ST_SetSRID(ST_MakePoint(longitude, latitude), 4326)::geography
                 END
               ) STORED"#,
        )
        .await?;
        db.execute_unprepared(
            "CREATE INDEX idx_person_address_location ON person_address USING GIST (location)",
        )
        .await?;
        Ok(())
    }
}
