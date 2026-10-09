use business::commons::legal_documents;
use business::domain::person::Person;
use business::use_cases::registration_use_case::{
    RegistrationError, RegistrationRequest, RegistrationUseCase,
};
use chrono::NaiveDate;
use entity::{consent_entity, person_entity, person_info_entity, settings_entity, user_entity};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, EntityTrait};

fn registration_request(email: &str) -> RegistrationRequest {
    RegistrationRequest {
        person: Person::new(
            "Registration".to_string(),
            "Owner".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "X".to_string(),
        ),
        email: email.to_string(),
        password: "Str0ng!Password".to_string(),
        language: "en".to_string(),
        terms_version: legal_documents::current_version(legal_documents::TERMS).unwrap(),
        privacy_version: legal_documents::current_version(legal_documents::PRIVACY).unwrap(),
        ip: "127.0.0.1".to_string(),
    }
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL targeting the workout_test database"]
async fn registration_persists_profile_settings_user_and_consents_atomically() {
    let database_url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    let database_name = database_url
        .split('?')
        .next()
        .unwrap_or(&database_url)
        .rsplit('/')
        .next()
        .unwrap_or_default();
    assert_eq!(database_name, "workout_test");

    let db = Database::connect(database_url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    let registered =
        RegistrationUseCase::execute(&db, registration_request("registration@example.test"))
            .await
            .unwrap();
    let person_id = registered.person_id;

    assert_eq!(registered.email, "registration@example.test");
    assert_eq!(
        person_entity::Entity::find().all(&db).await.unwrap().len(),
        1
    );
    assert_eq!(
        person_info_entity::Entity::find()
            .all(&db)
            .await
            .unwrap()
            .len(),
        1
    );
    let settings = settings_entity::Entity::find().all(&db).await.unwrap();
    assert_eq!(settings.len(), 1);
    assert!(settings[0].notifications_enabled);
    let users = user_entity::Entity::find().all(&db).await.unwrap();
    assert_eq!(users.len(), 1);
    assert_eq!(users[0].person_id, person_id);
    let consents = consent_entity::Entity::find().all(&db).await.unwrap();
    assert_eq!(consents.len(), 2);
    assert!(consents
        .iter()
        .any(|consent| consent.document == legal_documents::TERMS));
    assert!(consents
        .iter()
        .any(|consent| consent.document == legal_documents::PRIVACY));

    let duplicate =
        RegistrationUseCase::execute(&db, registration_request("registration@example.test"))
            .await
            .unwrap_err();
    assert!(matches!(duplicate, RegistrationError::Persistence));
    assert_eq!(
        person_entity::Entity::find().all(&db).await.unwrap().len(),
        1
    );
    assert_eq!(
        person_info_entity::Entity::find()
            .all(&db)
            .await
            .unwrap()
            .len(),
        1
    );
}
