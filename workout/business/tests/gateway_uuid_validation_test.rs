use business::gateway::business_profile_gateway::BusinessProfileGateway;
use business::gateway::exercise_gateway::ExerciseGateway;
use business::gateway::friend_gateway::FriendGateway;
use business::gateway::person_gateway::PersonGateway;
use business::gateway::workout_gateway::WorkoutGateway;
use sea_orm::{DatabaseBackend, MockDatabase};

#[tokio::test]
async fn gateways_reject_malformed_uuid_before_querying() {
    let db = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let invalid = "not-a-uuid";

    assert!(ExerciseGateway::find_by_uuid(&db, invalid.to_string())
        .await
        .is_err());
    assert!(WorkoutGateway::find_by_uuid(&db, invalid.to_string())
        .await
        .is_err());
    assert!(PersonGateway::find_by_uuid(&db, invalid).await.is_err());
    assert!(BusinessProfileGateway::find_by_uuid(&db, invalid)
        .await
        .is_err());
    assert!(
        FriendGateway::find_all_accepted_friends_by_uuid(&db, invalid.to_string())
            .await
            .is_err()
    );
}
