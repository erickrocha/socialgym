use business::use_cases::person_use_case::PersonUseCase;
use chrono::{NaiveDate, Utc};
use entity::friends_entity::FriendsEntity;
use entity::person_entity::PersonEntity;
use entity::person_info_entity::PersonInfoEntity;
use sea_orm::{DatabaseBackend, DbErr, MockDatabase};
use uuid::Uuid;

fn person_entity(id: i32, first_name: &str) -> PersonEntity {
    PersonEntity {
        id,
        uuid: Uuid::from_u128(id as u128),
        first_name: first_name.to_string(),
        surname: "Doe".to_string(),
        date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
        gender: "M".to_string(),
        avatar: None,
        cover_image: None,
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn person_info_entity(person_id: i32) -> PersonInfoEntity {
    PersonInfoEntity {
        id: person_id,
        person_id,
        biography: Some("bio".to_string()),
        relationship: None,
        job: None,
        home_town: None,
        current_city: None,
        weight: None,
        height: None,
        uuid: Uuid::from_u128(1000 + person_id as u128),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    }
}

fn friendship(id: i32, person_id: i32, friend_id: i32) -> FriendsEntity {
    FriendsEntity {
        id,
        uuid: Uuid::from_u128(2000 + id as u128),
        person_id,
        person_uuid: Uuid::from_u128(person_id as u128),
        friend_id,
        friend_uuid: Uuid::from_u128(friend_id as u128),
        created_at: Utc::now(),
        updated_at: Utc::now(),
        status: "Accepted".to_string(),
    }
}

#[tokio::test]
async fn get_returns_not_found_when_person_is_missing() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<PersonEntity>::new()])
        .into_connection();

    let error = PersonUseCase::get(&db, 1).await.unwrap_err();

    assert_eq!(error.message, "Person not found");
}

#[tokio::test]
async fn get_fails_when_person_info_lookup_errors() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_entity(1, "John")]])
        .append_query_errors(vec![DbErr::Custom("db down".to_string())])
        .into_connection();

    let error = PersonUseCase::get(&db, 1).await.unwrap_err();

    assert_eq!(error.message, "Person info not found");
}

#[tokio::test]
async fn get_returns_person_with_info_and_no_related_data() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_entity(1, "John")]])
        .append_query_results(vec![vec![person_info_entity(1)]])
        .into_connection();

    let person = PersonUseCase::get(&db, 1).await.unwrap();

    assert_eq!(person.firstname, "John");
    assert_eq!(
        person.person_info.unwrap().biography.as_deref(),
        Some("bio")
    );
    assert!(person.addresses.is_empty());
    assert!(person.business_profiles.is_empty());
}

#[tokio::test]
async fn find_by_uuid_rejects_a_malformed_uuid() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();

    assert!(PersonUseCase::find_by_uuid(&db, "not-a-uuid".to_string())
        .await
        .is_err());
}

#[tokio::test]
async fn find_by_uuid_returns_not_found_for_unknown_uuid() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<PersonEntity>::new()])
        .into_connection();

    let error = PersonUseCase::find_by_uuid(&db, Uuid::from_u128(9).to_string())
        .await
        .unwrap_err();

    assert_eq!(error.message, "Person not found");
}

#[tokio::test]
async fn find_by_uuid_fails_when_person_info_lookup_errors() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_entity(2, "Jane")]])
        .append_query_errors(vec![DbErr::Custom("db down".to_string())])
        .into_connection();

    let error = PersonUseCase::find_by_uuid(&db, Uuid::from_u128(2).to_string())
        .await
        .unwrap_err();

    assert_eq!(error.message, "Person info not found");
}

#[tokio::test]
async fn find_by_uuid_returns_person_with_info() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![person_entity(2, "Jane")]])
        .append_query_results(vec![vec![person_info_entity(2)]])
        .into_connection();

    let person = PersonUseCase::find_by_uuid(&db, Uuid::from_u128(2).to_string())
        .await
        .unwrap();

    assert_eq!(person.firstname, "Jane");
    assert!(person.person_info.is_some());
}

#[tokio::test]
async fn update_requires_person_info() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let person = business::domain::person::Person::update(
        Some(1),
        "John".to_string(),
        "Doe".to_string(),
        NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
        "M".to_string(),
        None,
        None,
        None,
        None,
        Vec::new(),
        Vec::new(),
    );

    let error = PersonUseCase::update(&db, person).await.unwrap_err();

    assert_eq!(error.message, "Person info is required");
}

#[tokio::test]
async fn get_all_friends_is_empty_when_the_friendship_query_fails() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_errors(vec![DbErr::Custom("db down".to_string())])
        .into_connection();

    assert!(PersonUseCase::get_all_friends(&db, 1).await.is_empty());
}

#[tokio::test]
async fn get_all_friends_resolves_the_other_side_of_each_friendship() {
    // Person 1 is the sender of the first friendship and the receiver of the second.
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![friendship(1, 1, 2), friendship(2, 3, 1)]])
        .append_query_results(vec![vec![person_entity(2, "Ann"), person_entity(3, "Bob")]])
        .into_connection();

    let friends = PersonUseCase::get_all_friends(&db, 1).await;

    let names: Vec<_> = friends.iter().map(|p| p.firstname.as_str()).collect();
    assert_eq!(names, vec!["Ann", "Bob"]);
}

#[tokio::test]
async fn received_and_sent_requests_are_empty_when_nothing_is_pending() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<FriendsEntity>::new()])
        .append_query_results(vec![Vec::<FriendsEntity>::new()])
        .into_connection();

    assert!(PersonUseCase::get_all_received_requests(&db, 1)
        .await
        .is_empty());
    assert!(PersonUseCase::get_all_sent_requests(&db, 1).await.is_empty());
}
