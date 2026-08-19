pub use sea_orm_migration::prelude::*;

mod m20260129_000000_enable_postgis;
mod m20260129_000001_create_table_countries;
mod m20260129_000003_create_table_person;
mod m20260129_000004_create_table_user;
mod m20260129_000005_create_table_person_address;
mod m20260129_000006_create_table_person_info;
mod m20260129_000007_create_table_workout;
mod m20260129_000008_create_table_exercise;
mod m20260129_000009_create_table_business_profile;
mod m20260129_000010_create_table_business_profile_address;
mod m20260129_000011_create_table_friends;
mod m20260306_000012_create_table_workout_exercise;
mod m20260408_000001_create_table_person_media;
mod m20260707_000001_create_table_settings;
mod m20260714_000001_create_table_profile;
mod m20260811_000001_create_table_team_members;
mod m20260813_000001_add_lockout_columns_to_user;
mod m20260813_000002_create_table_revoked_token;
mod m20260819_000001_seed_countries;

pub struct Migrator;

#[async_trait::async_trait]
impl MigratorTrait for Migrator {
    fn migrations() -> Vec<Box<dyn MigrationTrait>> {
        vec![
            Box::new(m20260129_000000_enable_postgis::Migration),
            Box::new(m20260129_000001_create_table_countries::Migration),
            Box::new(m20260129_000003_create_table_person::Migration),
            Box::new(m20260129_000004_create_table_user::Migration),
            Box::new(m20260129_000006_create_table_person_info::Migration),
            Box::new(m20260129_000005_create_table_person_address::Migration),
            Box::new(m20260129_000011_create_table_friends::Migration),
            Box::new(m20260129_000009_create_table_business_profile::Migration),
            Box::new(m20260129_000010_create_table_business_profile_address::Migration),
            Box::new(m20260129_000007_create_table_workout::Migration),
            Box::new(m20260129_000008_create_table_exercise::Migration),
            Box::new(m20260306_000012_create_table_workout_exercise::Migration),
            Box::new(m20260408_000001_create_table_person_media::Migration),
            Box::new(m20260707_000001_create_table_settings::Migration),
            Box::new(m20260714_000001_create_table_profile::Migration),
            Box::new(m20260811_000001_create_table_team_members::Migration),
            Box::new(m20260813_000001_add_lockout_columns_to_user::Migration),
            Box::new(m20260813_000002_create_table_revoked_token::Migration),
            Box::new(m20260819_000001_seed_countries::Migration),
        ]
    }
}
