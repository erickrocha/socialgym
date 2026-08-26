#[cfg(feature = "mock")]
mod tests {
    use business::commons::functions::uuid_to_string;
    use business::domain::enums::{Category, Difficulty, Visibility};
    use business::commons::entity_mapper::EntityMapper;
use business::domain::business_error::BusinessErrorKind;
use business::domain::exercise::{Exercise, ExerciseEntityMapper};
    use business::domain::person::Person;
    use business::domain::person_address::PersonAddress;
    use business::domain::person_info::PersonInfo;
    use business::domain::user::User;
    use business::domain::workout::Workout;
    use business::gateway::business_profile_gateway::BusinessProfileGateway;
    use business::gateway::exercise_gateway::ExerciseGateway;
    use business::gateway::friend_gateway::FriendGateway;
    use business::gateway::person_gateway::PersonGateway;
    use business::gateway::workout_gateway::WorkoutGateway;
    use business::use_cases::authentication::{Authentication, AuthenticationError, ValidateError};
    use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
    use business::use_cases::exercise_use_case::ExerciseUseCase;
    use business::use_cases::friend_use_case::FriendUseCase;
    use business::use_cases::logout_use_case::LogoutUseCase;
    use business::use_cases::person_address_use_case::PersonAddressUseCase;
    use business::use_cases::person_info_use_case::PersonInfoUseCase;
    use business::use_cases::person_use_case::PersonUseCase;
    use business::use_cases::refresh_token::RefreshToken;
    use business::use_cases::user_use_case::{UserUseCase, UserUseCaseError};
    use business::use_cases::workout_use_case::WorkoutUseCase;
    use chrono::{DateTime, NaiveDate, Utc};

    fn exercise_entity_owned_by(owner_id: i32) -> entity::exercise_entity::ExerciseEntity {
        entity::exercise_entity::ExerciseEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            name: "Push Ups".to_string(),
            description: Some("Standard push ups".to_string()),
            owner_id,
            owner_uuid: Uuid::new_v4(),
            owner_name: "John Doe".to_string(),
            sets: 3,
            category: "Force".to_string(),
            reps_or_duration: 20,
            visibility: "Public".to_string(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    fn person_address_entity_owned_by(
        person_id: i32,
    ) -> entity::person_address_entity::PersonAddressEntity {
        entity::person_address_entity::PersonAddressEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            person_id,
            address_line1: "456 Oak Ave".to_string(),
            address_line2: None,
            locality: "Boston".to_string(),
            administrative_area: "MA".to_string(),
            country_code: "USA".to_string(),
            postal_code: Some("02101".to_string()),
            current: true,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }

    #[tokio::test]
    async fn test_person_address_use_case_delete_denied_for_non_owner() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![person_address_entity_owned_by(1)]])
            .into_connection();

        let result = PersonAddressUseCase::delete_person_address(&db, 1, 2).await;

        assert_eq!(result.unwrap_err().kind, BusinessErrorKind::Forbidden);
    }

    /// The authenticated principal that owner-scoped use cases act on behalf of.
    fn actor(person_id: i32, person_uuid: String) -> User {
        User::new(
            Some("Test Actor".to_string()),
            "actor@example.com".to_string(),
            "hashed".to_string(),
            person_id,
            person_uuid,
        )
    }

    use sea_orm::DbBackend;
    use std::env;
    use std::sync::Mutex;
    use uuid::Uuid;

    /// Serializes tests that mutate the AUTH_RULES_ENABLED/*_ENABLED toggles, since
    /// `#[tokio::test]`s run concurrently and share process-wide env vars.
    static AUTH_ENV_LOCK: Mutex<()> = Mutex::new(());

    fn clear_auth_toggle_env() {
        env::remove_var("AUTH_RULES_ENABLED");
        env::remove_var("PASSWORD_POLICY_ENABLED");
        env::remove_var("LOGIN_LOCKOUT_ENABLED");
        env::remove_var("TOKEN_REVOCATION_ENABLED");
    }

    #[tokio::test]
    async fn uuid_gateways_reject_malformed_strings_before_querying() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();
        let invalid = "not-a-uuid";

        assert!(ExerciseGateway::find_by_uuid(&db, invalid.to_string()).await.is_err());
        assert!(WorkoutGateway::find_by_uuid(&db, invalid.to_string()).await.is_err());
        assert!(PersonGateway::find_by_uuid(&db, invalid).await.is_err());
        assert!(BusinessProfileGateway::find_by_uuid(&db, invalid).await.is_err());
        assert!(FriendGateway::find_all_accepted_friends_by_uuid(&db, invalid.to_string())
            .await
            .is_err());
    }

    #[allow(clippy::too_many_arguments)]
    fn mock_user_model(
        email: &str,
        password_hash: &str,
        failed_login_attempts: i32,
        locked_until: Option<DateTime<Utc>>,
        token_valid_after: Option<DateTime<Utc>>,
    ) -> entity::user_entity::UserEntity {
        entity::user_entity::UserEntity {
            id: 1,
            name: Some("John doe".to_string()),
            email: email.to_string(),
            password: password_hash.to_string(),
            enabled: true,
            first_login: true,
            person_id: 1,
            person_uuid: Uuid::new_v4(),
            uuid: Uuid::new_v4(),
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
            failed_login_attempts,
            locked_until,
            token_valid_after,
            deletion_requested_at: None,
            deletion_scheduled_at: None,
        }
    }

    fn mock_person_model() -> entity::person_entity::PersonEntity {
        entity::person_entity::PersonEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            first_name: "John".to_string(),
            surname: "Doe".to_string(),
            date_of_birth: NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            gender: "M".to_string(),
            avatar: None,
            cover_image: None,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        }
    }
    // ========================
    // User Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_user_use_case_add_empty_email() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let user = User::new(
            Some("John doe".to_string()),
            "".to_string(),
            "password".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let result = UserUseCase::add(&db, user).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserUseCaseError::InvalidInput
        ));
    }

    #[tokio::test]
    async fn test_user_use_case_add_empty_password() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let result = UserUseCase::add(&db, user).await;

        assert!(result.is_err());
        assert!(matches!(
            result.unwrap_err(),
            UserUseCaseError::InvalidInput
        ));
    }

    #[tokio::test]
    async fn test_user_use_case_add_valid_user() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_user_model(
                "test@example.com",
                "hashed",
                0,
                None,
                None,
            )]])
            .into_connection();

        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "Str0ng!Pass".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let result = UserUseCase::add(&db, user).await;

        assert!(result.is_ok());
        let added_user = result.unwrap();
        assert_eq!(added_user.email, "test@example.com");
    }

    #[tokio::test]
    async fn test_user_use_case_add_weak_password_rejected() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "weak".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let result = UserUseCase::add(&db, user).await;

        assert!(matches!(result, Err(UserUseCaseError::WeakPassword(_))));
    }

    #[tokio::test]
    async fn test_user_use_case_add_password_policy_disabled_via_env() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        env::set_var("PASSWORD_POLICY_ENABLED", "false");

        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![mock_user_model(
                "weak@example.com",
                "hashed",
                0,
                None,
                None,
            )]])
            .into_connection();

        let user = User::new(
            Some("John doe".to_string()),
            "weak@example.com".to_string(),
            "weak".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let result = UserUseCase::add(&db, user).await;

        clear_auth_toggle_env();
        assert!(result.is_ok());
    }

    // ========================
    // Person Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_person_use_case_add_empty_firstname() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let person = Person::update(
            None,
            "".to_string(),
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

        let result = PersonUseCase::add(&db, person).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "Person and User information are required"
        );
    }

    #[tokio::test]
    async fn test_person_use_case_add_empty_surname() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let person = Person::update(
            None,
            "John".to_string(),
            "".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            "M".to_string(),
            None,
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
        );

        let result = PersonUseCase::add(&db, person).await;

        assert!(result.is_err());
    }

    #[tokio::test]
    async fn test_person_use_case_add_empty_gender() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let person = Person::update(
            None,
            "John".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 15).unwrap(),
            "".to_string(),
            None,
            None,
            None,
            None,
            Vec::new(),
            Vec::new(),
        );

        let result = PersonUseCase::add(&db, person).await;

        assert!(result.is_err());
    }

    // Note: Full update tests require integration testing with real database
    // Validation tests above cover the main error cases

    // ========================
    // Person Info Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_person_info_use_case_get_valid_id() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::person_info_entity::PersonInfoEntity {
                id: 1,
                person_id: 1,
                biography: Some("Test bio".to_string()),
                relationship: None,
                job: None,
                home_town: None,
                current_city: None,
                weight: None,
                height: None,
                uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = PersonInfoUseCase::get(&db, 1).await;

        assert!(result.is_ok());
        let person_info = result.unwrap();
        assert_eq!(person_info.id, Some(1));
        assert_eq!(person_info.biography, Some("Test bio".to_string()));
    }

    #[tokio::test]
    async fn test_person_info_use_case_get_not_found() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results::<entity::person_info_entity::PersonInfoEntity, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
            .into_connection();

        let result = PersonInfoUseCase::get(&db, 999).await;

        assert!(result.is_err());
        assert_eq!(result.unwrap_err().message, "PersonInfo not found");
    }

    #[tokio::test]
    async fn test_person_info_use_case_update_valid() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::person_info_entity::PersonInfoEntity {
                id: 1,
                person_id: 1,
                biography: Some("Test bio".to_string()),
                relationship: None,
                job: None,
                home_town: None,
                current_city: None,
                weight: None,
                height: None,
                uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::person_info_entity::PersonInfoEntity {
                id: 1,
                person_id: 1,
                biography: Some("Updated bio".to_string()),
                relationship: None,
                job: None,
                home_town: None,
                current_city: None,
                weight: None,
                height: None,
                uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let mut person_info = PersonInfo::new(
            1,
            Some("Test bio".to_string()),
            None,
            None,
            None,
            None,
            None,
            None,
        );
        person_info.biography = Some("Updated bio".to_string());

        let result = PersonInfoUseCase::update(&db, 1, person_info).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.biography, Some("Updated bio".to_string()));
    }

    // ========================
    // Person Address Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_person_address_use_case_add_person_address() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::person_address_entity::PersonAddressEntity {
                id: 1,
                person_id: 1,
                address_line1: "123 Main St".to_string(),
                address_line2: Some("".to_string()),
                administrative_area: "".to_string(),
                country_code: "BR".to_string(),
                postal_code: Some("88058573".to_string()),
                current: true,
                locality: "Santa Catarina".to_string(),
                uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let address = PersonAddress::new(
            1,
            "123 Main St".to_string(),
            Some("".to_string()),
            "Santa Catarina".to_string(),
            "BR".to_string(),
            Some("88058573".to_string()),
            "BR".to_string(),
            true,
        );

        let result = PersonAddressUseCase::add_person_address(&db, address, None, None).await;

        assert!(result.is_ok());
        let saved_address = result.unwrap();
        assert_eq!(saved_address.locality, "Santa Catarina".to_string());
        assert_eq!(saved_address.current, true);
    }

    // Note: Delete operations are best tested with integration tests
    // The address update test below covers the main use case

    #[tokio::test]
    async fn test_person_address_use_case_update_person_address() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![person_address_entity_owned_by(1)]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::person_address_entity::PersonAddressEntity {
                id: 1,
                uuid: Uuid::new_v4(),
                person_id: 1,
                address_line1: "456 Oak Ave".to_string(),
                address_line2: None,
                locality: "Boston".to_string(),
                administrative_area: "MA".to_string(),
                country_code: "USA".to_string(),
                postal_code: Some("02101".to_string()),
                current: true,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let mut address = PersonAddress::new(
            1,
            "456 Oak Ave".to_string(),
            None,
            "Boston".to_string(),
            "USA".to_string(),
            Some("02101".to_string()),
            "MA".to_string(),
            true,
        );
        address.id = Some(1);

        let result =
            PersonAddressUseCase::update_person_address(&db, address, 1, None, None).await;

        assert!(result.is_ok());
        let updated = result.unwrap();
        assert_eq!(updated.locality, "Boston".to_string());
    }

    // ========================
    // Friend Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_friend_use_case_send_friend_request_same_person() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let result = FriendUseCase::send_friend_request(&db, 1, 1).await;

        assert!(result.is_err());
        assert_eq!(
            result.unwrap_err().message,
            "Cannot send friend request to yourself"
        );
    }

    #[tokio::test]
    async fn test_friend_use_case_send_friend_request_valid() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![Vec::<entity::friends_entity::FriendsEntity>::new()])
            .append_query_results(vec![vec![entity::person_entity::PersonEntity {
                id: 1,
                uuid: Uuid::new_v4(),
                first_name: "John".to_string(),
                surname: "Doe".to_string(),
                date_of_birth: chrono::NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
                gender: "M".to_string(),
                avatar: None,
                cover_image: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_query_results(vec![vec![entity::person_entity::PersonEntity {
                id: 2,
                uuid: Uuid::new_v4(),
                first_name: "Jane".to_string(),
                surname: "Doe".to_string(),
                date_of_birth: chrono::NaiveDate::from_ymd_opt(1992, 5, 20).unwrap(),
                gender: "F".to_string(),
                avatar: None,
                cover_image: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::friends_entity::FriendsEntity {
                id: 1,
                person_id: 1,
                friend_id: 2,
                status: "Pending".to_string(),
                uuid: Uuid::new_v4(),
                person_uuid: Uuid::new_v4(),
                friend_uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = FriendUseCase::send_friend_request(&db, 1, 2).await;

        assert!(result.is_ok());
        let friend = result.unwrap();
        assert_eq!(friend.person_id, 1);
        assert_eq!(friend.friend_id, 2);
        assert_eq!(friend.status.as_str(), "Pending");
    }

    #[tokio::test]
    async fn test_friend_use_case_accept_friend_request() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::friends_entity::FriendsEntity {
                id: 1,
                person_id: 1,
                friend_id: 2,
                status: "Pending".to_string(),
                uuid: Uuid::new_v4(),
                person_uuid: Uuid::new_v4(),
                friend_uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::friends_entity::FriendsEntity {
                id: 1,
                person_id: 1,
                friend_id: 2,
                status: "Accepted".to_string(),
                uuid: Uuid::new_v4(),
                person_uuid: Uuid::new_v4(),
                friend_uuid: Uuid::new_v4(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = FriendUseCase::accept_friend_request(&db, 1, 2).await;

        assert!(result.is_ok());
        let accepted = result.unwrap();
        assert_eq!(accepted.status.as_str(), "Accepted");
    }

    #[tokio::test]
    async fn test_friend_use_case_deny_friend_request() {
        let uuid = Uuid::new_v4();
        let person_uuid = Uuid::new_v4();
        let friend_uuid = Uuid::new_v4();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::friends_entity::FriendsEntity {
                id: 1,
                person_id: 1,
                friend_id: 2,
                status: "Pending".to_string(),
                uuid: uuid.clone(),
                person_uuid: person_uuid.clone(),
                friend_uuid: friend_uuid.clone(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::friends_entity::FriendsEntity {
                id: 1,
                person_id: 1,
                friend_id: 2,
                status: "Rejected".to_string(),
                uuid,
                person_uuid,
                friend_uuid,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = FriendUseCase::deny_friend_request(&db, 1, 2).await;

        assert!(result.is_ok());
        let denied = result.unwrap();
        assert_eq!(denied.status.as_str(), "Rejected");
    }

    // ========================
    // Workout Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_workout_use_case_get_not_found() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results::<entity::workout_entity::WorkoutEntity, Vec<_>, Vec<Vec<_>>>(vec![vec![]])
            .into_connection();

        let result = WorkoutUseCase::get(&db, 999).await;

        assert!(result.is_err());
    }

    // Note: Complex workout queries with exercises are better tested with integration tests

    #[tokio::test]
    async fn test_workout_use_case_add_valid_workout() {
        let uuid = Uuid::new_v4();
        let person_uuid = Uuid::new_v4();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::workout_entity::WorkoutEntity {
                id: 1,
                uuid: uuid.clone(),
                name: "Push ups".to_string(),
                description: Some("Basic push ups".to_string()),
                difficulty: "Easy".to_string(),
                muscle_group: "Chest".to_string(),
                owner_id: 1,
                owner_uuid: person_uuid.clone(),
                visibility: "Public".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let workout = Workout {
            id: None,
            uuid: None,
            owner_id: 1,
            owner_uuid: uuid_to_string(person_uuid),
            name: "Push ups".to_string(),
            description: Some("Basic push ups".to_string()),
            difficulty: Difficulty::Easy,
            muscle_group: "Chest".to_string(),
            exercises: Vec::new(),
            visibility: Visibility::Public,
            created_at: None,
            updated_at: None,
        };

        let result = WorkoutUseCase::persist(
            &db,
            workout,
            &actor(1, uuid_to_string(person_uuid)),
            None,
            None,
        )
        .await;

        assert!(result.is_ok());
        let saved = result.unwrap();
        assert_eq!(saved.name, "Push ups");
    }

    // ========================
    // Exercise Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_exercise_use_case_add_valid_exercise() {
        let uuid = Uuid::new_v4();
        let owner_uuid = Uuid::new_v4();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::exercise_entity::ExerciseEntity {
                id: 1,
                uuid: uuid.clone(),
                name: "Push Ups".to_string(),
                description: Some("Standard push ups".to_string()),
                owner_id: 1,
                owner_uuid: owner_uuid.clone(),
                owner_name: "John Doe".to_string(),
                sets: 3,
                category: "Force".to_string(),
                reps_or_duration: 20,
                visibility: "Public".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let exercise = Exercise {
            id: None,
            uuid: None,
            name: "Push Ups".to_string(),
            description: Some("Standard push ups".to_string()),
            sets: 3,
            owner_id: 1,
            owner_uuid: uuid_to_string(owner_uuid),
            owner_name: "John Doe".to_string(),
            category: Category::Force,
            reps_or_duration: 20,
            visibility: Visibility::Public,
            created_at: None,
            updated_at: None,
        };

        let result =
            ExerciseUseCase::persist(&db, exercise, &actor(1, uuid_to_string(owner_uuid)), None)
                .await;

        assert!(result.is_ok());
        let saved = result.unwrap();
        assert_eq!(saved.name, "Push Ups");
        assert_eq!(saved.owner_id, 1);
    }

    #[tokio::test]
    async fn test_exercise_use_case_get_valid_exercise() {
        let uuid = Uuid::new_v4();
        let owner_uuid = Uuid::new_v4();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::exercise_entity::ExerciseEntity {
                id: 1,
                uuid: uuid.clone(),
                name: "Push Ups".to_string(),
                description: Some("Standard push ups".to_string()),
                owner_id: 1,
                owner_uuid: owner_uuid.clone(),
                owner_name: "John Doe".to_string(),
                sets: 3,
                category: "Force".to_string(),
                reps_or_duration: 20,
                visibility: "Public".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = ExerciseUseCase::get(&db, 1).await;

        assert!(result.is_ok());
        let exercise = result.unwrap();
        assert_eq!(exercise.name, "Push Ups");
        assert_eq!(exercise.sets, 3);
    }

    #[tokio::test]
    async fn test_exercise_use_case_find_by_visibility_public() {
        let uuid = Uuid::new_v4();
        let owner_uuid = Uuid::new_v4();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::exercise_entity::ExerciseEntity {
                id: 1,
                uuid: uuid.clone(),
                name: "Push Ups".to_string(),
                description: Some("Standard push ups".to_string()),
                owner_id: 1,
                owner_uuid: owner_uuid.clone(),
                owner_name: "John Doe".to_string(),
                sets: 3,
                category: "Force".to_string(),
                reps_or_duration: 20,
                visibility: "Public".to_string(),
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = ExerciseUseCase::find_by_visibility(&db, Visibility::Public, 1).await;

        assert!(result.is_ok());
        let exercises = result.unwrap();
        assert_eq!(exercises.len(), 1);
        assert_eq!(exercises[0].visibility.to_string(), "public");
    }

    #[tokio::test]
    async fn test_exercise_use_case_delete_exercise() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![exercise_entity_owned_by(1)]])
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .into_connection();

        let result = ExerciseUseCase::delete_by_id(&db, 1, 1).await;

        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_exercise_use_case_delete_denied_for_non_owner() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![exercise_entity_owned_by(1)]])
            .into_connection();

        // person 2 asking to delete person 1's exercise
        let result = ExerciseUseCase::delete_by_id(&db, 1, 2).await;

        assert_eq!(result.unwrap_err().kind, BusinessErrorKind::Forbidden);
    }

    #[tokio::test]
    async fn test_exercise_use_case_persist_ignores_client_supplied_owner() {
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![exercise_entity_owned_by(7)]])
            .into_connection();

        let mut exercise = ExerciseEntityMapper::from_model(exercise_entity_owned_by(1));
        exercise.id = None;
        exercise.uuid = None;
        // client claims to be person 1; the actor is person 7
        let saved = ExerciseUseCase::persist(
            &db,
            exercise,
            &actor(7, uuid_to_string(Uuid::new_v4())),
            None,
        )
        .await
        .unwrap();

        assert_eq!(saved.owner_id, 7);
    }

    #[tokio::test]
    async fn test_exercise_use_case_private_exercise_not_readable_by_others() {
        let mut private = ExerciseEntityMapper::from_model(exercise_entity_owned_by(1));
        private.visibility = Visibility::Private;

        assert!(ExerciseUseCase::ensure_readable(&private, 1).is_ok());
        assert_eq!(
            ExerciseUseCase::ensure_readable(&private, 2).unwrap_err().kind,
            BusinessErrorKind::Forbidden
        );
        // public exercises stay readable by anyone
        private.visibility = Visibility::Public;
        assert!(ExerciseUseCase::ensure_readable(&private, 2).is_ok());
    }

    // ========================
    // Business Profile Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_business_profile_use_case_get_by_owner_id() {
        let uuid = Uuid::new_v4();
        let owner_uuid = Uuid::new_v4();
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![entity::business_profile_entity::BusinessProfileEntity {
                id: 1,
                uuid: uuid.clone(),
                owner_id: 1,
                owner_uuid: owner_uuid.clone(),
                tax_id: "12345".to_string(),
                business_name: "Gym XYZ".to_string(),
                business_type: "Gym".to_string(),
                social_name: None,
                logo: None,
                cover_image: None,
                created_at: chrono::Utc::now(),
                updated_at: chrono::Utc::now(),
            }]])
            .append_query_results::<entity::business_profile_address_entity::BusinessProfileAddressEntity, Vec<_>, Vec<Vec<_>>>(
                vec![vec![]],
            )
            .into_connection();

        let result = BusinessProfileUseCase::get_by_owner_id(&db, 1).await;

        assert!(result.is_ok());
        let profiles = result.unwrap();
        assert_eq!(profiles.len(), 1);
        assert_eq!(profiles[0].business_name, "Gym XYZ");
    }

    // ========================
    // Authentication Use Case Tests
    // ========================

    #[tokio::test]
    async fn test_authentication_generate_access_token() {
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key_for_access_token");
        std::env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");
        let uuid = Uuid::new_v4();
        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "password".to_string(),
            1,
            uuid_to_string(uuid),
        );
        let person = Person::new(
            "John".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "M".to_string(),
        );

        let token = Authentication::generate_access_token(&user, &person, None, true);

        assert_eq!(token.token_type, "Bearer");
        assert!(!token.access_token.is_empty());
        assert!(!token.refresh_token.unwrap().is_empty());
        assert_eq!(token.username, "test@example.com");
    }

    #[tokio::test]
    async fn test_authentication_execute_invalid_email() {
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let result = Authentication::execute(&db, "".to_string(), "password".to_string()).await;

        assert!(matches!(
            result,
            Err(AuthenticationError::InvalidCredentials)
        ));
    }

    #[tokio::test]
    async fn test_authentication_execute_invalid_password() {
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let result =
            Authentication::execute(&db, "test@example.com".to_string(), "".to_string()).await;

        assert!(matches!(
            result,
            Err(AuthenticationError::InvalidCredentials)
        ));
    }

    // ========================
    // Login Lockout Tests
    // ========================

    #[tokio::test]
    async fn test_authentication_locked_account_rejects_correct_password() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
        std::env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");

        let hash = bcrypt::hash("CorrectPass1!", bcrypt::DEFAULT_COST).unwrap();
        let locked_until = Utc::now() + chrono::Duration::seconds(60);
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![mock_user_model(
                "test@example.com",
                &hash,
                2,
                Some(locked_until),
                None,
            )]])
            .append_query_results(vec![vec![mock_person_model()]])
            .into_connection();

        let result = Authentication::execute(
            &db,
            "test@example.com".to_string(),
            "CorrectPass1!".to_string(),
        )
        .await;

        assert!(matches!(
            result,
            Err(AuthenticationError::AccountLocked { .. })
        ));
    }

    #[tokio::test]
    async fn test_authentication_lockout_bypassed_when_disabled() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        env::set_var("LOGIN_LOCKOUT_ENABLED", "false");
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
        std::env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");

        let hash = bcrypt::hash("CorrectPass1!", bcrypt::DEFAULT_COST).unwrap();
        let locked_until = Utc::now() + chrono::Duration::seconds(60);
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![mock_user_model(
                "test@example.com",
                &hash,
                2,
                Some(locked_until),
                None,
            )]])
            .append_query_results(vec![vec![mock_person_model()]])
            .into_connection();

        let result = Authentication::execute(
            &db,
            "test@example.com".to_string(),
            "CorrectPass1!".to_string(),
        )
        .await;

        clear_auth_toggle_env();
        assert!(result.is_ok());
    }

    // ========================
    // Token Revocation Tests
    // ========================

    #[tokio::test]
    async fn test_authentication_validate_revoked_jti_rejected() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_revoked_jti");

        let person = business::domain::person::Person::new(
            "John".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "M".to_string(),
        );
        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "password".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let token = Authentication::generate_access_token(&user, &person, None, false);

        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![mock_user_model(
                "test@example.com",
                "hashed",
                0,
                None,
                None,
            )]])
            .append_query_results(vec![vec![entity::revoked_token_entity::RevokedTokenEntity {
                id: 1,
                uuid: Uuid::new_v4(),
                jti: "revoked-jti".to_string(),
                user_id: 1,
                token_type: "access".to_string(),
                expires_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let result = Authentication::validate(&db, token.access_token).await;

        assert!(matches!(result, Err(ValidateError::Revoked)));
    }

    #[tokio::test]
    async fn test_authentication_validate_revoked_by_watermark() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_watermark");

        let person = business::domain::person::Person::new(
            "John".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "M".to_string(),
        );
        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "password".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let token = Authentication::generate_access_token(&user, &person, None, false);

        // watermark set to the future guarantees it is after this token's `iat`
        let watermark = Utc::now() + chrono::Duration::seconds(60);
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![mock_user_model(
                "test@example.com",
                "hashed",
                0,
                None,
                Some(watermark),
            )]])
            .into_connection();

        let result = Authentication::validate(&db, token.access_token).await;

        assert!(matches!(result, Err(ValidateError::Revoked)));
    }

    #[tokio::test]
    async fn test_authentication_validate_revocation_disabled_skips_db_check() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();
        env::set_var("TOKEN_REVOCATION_ENABLED", "false");
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_disabled");

        let person = business::domain::person::Person::new(
            "John".to_string(),
            "Doe".to_string(),
            NaiveDate::from_ymd_opt(1990, 1, 1).unwrap(),
            "M".to_string(),
        );
        let user = User::new(
            Some("John doe".to_string()),
            "test@example.com".to_string(),
            "password".to_string(),
            1,
            Uuid::new_v4().to_string(),
        );
        let token = Authentication::generate_access_token(&user, &person, None, false);

        // No revoked_token query result appended: if the code queried it anyway
        // despite being disabled, the mock DB would panic on the missing result.
        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_query_results(vec![vec![mock_user_model(
                "test@example.com",
                "hashed",
                0,
                None,
                None,
            )]])
            .into_connection();

        let result = Authentication::validate(&db, token.access_token).await;

        clear_auth_toggle_env();
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_logout_use_case_revokes_token() {
        let _guard = AUTH_ENV_LOCK.lock().unwrap();
        clear_auth_toggle_env();

        let db = sea_orm::MockDatabase::new(DbBackend::Postgres)
            .append_exec_results(vec![sea_orm::MockExecResult {
                last_insert_id: 1,
                rows_affected: 1,
            }])
            .append_query_results(vec![vec![entity::revoked_token_entity::RevokedTokenEntity {
                id: 1,
                uuid: Uuid::new_v4(),
                jti: "some-jti".to_string(),
                user_id: 1,
                token_type: "access".to_string(),
                expires_at: chrono::Utc::now(),
                created_at: chrono::Utc::now(),
            }]])
            .into_connection();

        let exp = Utc::now().timestamp() + 3600;
        let result = LogoutUseCase::execute(&db, 1, "some-jti".to_string(), exp, None).await;

        assert!(result.is_ok());
    }

    // ========================
    // Refresh Token Use Case Tests
    // ========================

    // Note: Token generation tests are better with integration tests
    // JWT validation requires proper token setup

    #[tokio::test]
    async fn test_refresh_token_invalid_token() {
        std::env::set_var("ACCESS_TOKEN_SECRET", "test_secret_key");
        std::env::set_var("REFRESH_TOKEN_SECRET", "test_refresh_secret");

        let db = sea_orm::MockDatabase::new(DbBackend::Postgres).into_connection();

        let result = RefreshToken::execute(&db, "invalid_token".to_string()).await;

        assert!(result.is_err());
    }
}
