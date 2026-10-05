use business::use_cases::logout_use_case::LogoutUseCase;
use entity::revoked_token_entity::RevokedTokenEntity;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use uuid::Uuid;

#[tokio::test]
async fn execute_persists_revoked_token() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_exec_results(vec![MockExecResult {
            last_insert_id: 1,
            rows_affected: 1,
        }])
        .append_query_results(vec![vec![RevokedTokenEntity {
            id: 1,
            uuid: Uuid::new_v4(),
            jti: "some-jti".to_string(),
            user_id: 1,
            token_type: "access".to_string(),
            expires_at: chrono::Utc::now(),
            created_at: chrono::Utc::now(),
        }]])
        .into_connection();

    let expiration = chrono::Utc::now().timestamp() + 3600;
    let result = LogoutUseCase::execute(&db, 1, "some-jti".to_string(), expiration, None).await;

    assert!(result.is_ok());
}
