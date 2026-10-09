//! Shared fixtures for use-case acceptance tests against a disposable PostgreSQL/PostGIS database.
#![allow(dead_code)]
use business::commons::authorization::ActingOwner;
use business::commons::legal_documents;
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use business::domain::exercise::{Category, Exercise, Visibility};
use business::domain::person::Person;
use business::domain::user::User;
use business::use_cases::registration_use_case::{RegistrationRequest, RegistrationUseCase};
use chrono::NaiveDate;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};

pub async fn fresh_db() -> DatabaseConnection {
    let url = std::env::var("TEST_DATABASE_URL")
        .expect("TEST_DATABASE_URL must point to the disposable workout_test database");
    let name = url
        .split('?')
        .next()
        .unwrap_or(&url)
        .rsplit('/')
        .next()
        .unwrap_or_default();
    assert_eq!(
        name, "workout_test",
        "refusing to run against a non-test database"
    );
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db
}

/// Registers a real Person + User (settings and mandatory consents included).
pub async fn register(db: &DatabaseConnection, n: u32) -> User {
    RegistrationUseCase::execute(
        db,
        RegistrationRequest {
            person: Person::new(
                format!("First{n}"),
                format!("Last{n}"),
                NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                "X".to_string(),
            ),
            email: format!("user{n}@example.test"),
            password: "Str0ng!Password".to_string(),
            language: "en".to_string(),
            terms_version: legal_documents::current_version(legal_documents::TERMS).unwrap(),
            privacy_version: legal_documents::current_version(legal_documents::PRIVACY).unwrap(),
            ip: "127.0.0.1".to_string(),
        },
    )
    .await
    .unwrap()
}

pub fn acting(user: &User) -> ActingOwner {
    ActingOwner::person(user.person_id, user.person_uuid.clone())
}

pub fn exercise(name: &str, visibility: Visibility) -> Exercise {
    Exercise {
        id: None,
        uuid: None,
        name: name.to_string(),
        description: Some("desc".to_string()),
        sets: 3,
        owner_id: 0,
        owner_uuid: String::new(),
        owner_name: "Owner".to_string(),
        category: Category::Force,
        reps_or_duration: 10,
        visibility,
        created_at: None,
        updated_at: None,
    }
}

pub fn kind(e: &BusinessError) -> &BusinessErrorKind {
    &e.kind
}

use business::domain::business_profile::BusinessProfile;
use business::domain::enums::{Difficulty, InviteStatus, ProfileType};
use business::domain::workout::Workout;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;

pub async fn business_profile(
    db: &DatabaseConnection,
    owner: &User,
    name: &str,
) -> BusinessProfile {
    BusinessProfileUseCase::add(
        db,
        BusinessProfile::new(
            0,
            String::new(),
            "12345678000199".to_string(),
            name.to_string(),
            ProfileType::Professional,
            None,
        ),
        owner,
    )
    .await
    .unwrap()
}

pub fn workout(name: &str, visibility: Visibility, exercises: Vec<Exercise>) -> Workout {
    Workout {
        id: None,
        uuid: None,
        owner_id: 0,
        owner_uuid: String::new(),
        name: name.to_string(),
        description: Some("plan".to_string()),
        difficulty: Difficulty::Easy,
        muscle_group: "legs".to_string(),
        exercises,
        visibility,
        status: InviteStatus::Accepted,
        assigned_by_profile_id: None,
        assigned_by_profile_uuid: None,
        created_at: None,
        updated_at: None,
    }
}

/// Points the AWS SDK at LocalStack (the infra/test stack) unless the environment already does.
pub fn aws_env() {
    for (k, v) in [
        ("AWS_ENDPOINT_URL", "http://localhost:4566"),
        ("AWS_REGION", "us-east-1"),
        ("AWS_ACCESS_KEY_ID", "test"),
        ("AWS_SECRET_ACCESS_KEY", "test"),
        ("AWS_WORKOUT_BUCKET", "socialgym-test-media"),
    ] {
        if std::env::var(k).is_err() {
            unsafe { std::env::set_var(k, v) };
        }
    }
}

/// Ensures the test bucket exists and creates a private queue for one test.
pub async fn localstack_bucket_and_queue() -> (String, String) {
    aws_env();
    let bucket = std::env::var("AWS_WORKOUT_BUCKET").unwrap();
    let s3 = business::gateway::aws_clients::s3_client().await;
    let _ = s3.create_bucket().bucket(&bucket).send().await;
    let sqs = business::gateway::aws_clients::sqs_client().await;
    let name = format!("uc-{}", uuid::Uuid::new_v4());
    let queue = sqs
        .create_queue()
        .queue_name(name)
        .send()
        .await
        .expect("LocalStack SQS must be reachable")
        .queue_url
        .unwrap();
    (bucket, queue)
}
