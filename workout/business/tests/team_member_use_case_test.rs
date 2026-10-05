use business::commons::functions::uuid_to_string;
use business::domain::business_error::BusinessErrorKind;
use business::use_cases::team_member_use_case::TeamMemberUseCase;
use chrono::NaiveDate;
use entity::business_profile_entity::BusinessProfileEntity;
use entity::person_entity::PersonEntity;
use entity::team_member_entity::TeamMemberEntity;
use sea_orm::{DatabaseBackend, MockDatabase};
use uuid::Uuid;

fn business_profile(owner_id: i32) -> BusinessProfileEntity {
    BusinessProfileEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        owner_id,
        owner_uuid: Uuid::new_v4(),
        tax_id: "12345".to_string(),
        business_name: "Gym XYZ".to_string(),
        business_type: "Professional".to_string(),
        social_name: None,
        logo: None,
        cover_image: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    }
}

fn person() -> PersonEntity {
    PersonEntity {
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

fn team_member(business_profile_id: i32, person_id: i32, status: &str) -> TeamMemberEntity {
    TeamMemberEntity {
        id: 1,
        uuid: Uuid::new_v4(),
        business_profile_id,
        business_profile_uuid: Uuid::new_v4(),
        person_id,
        person_uuid: Uuid::new_v4(),
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
        status: status.to_string(),
    }
}

#[tokio::test]
async fn find_roster_returns_profile_owner_and_accepted_members() {
    let profile = business_profile(1);
    let profile_uuid = uuid_to_string(profile.uuid);
    let owner = person();
    let owner_uuid = uuid_to_string(owner.uuid);
    let member = team_member(1, 2, "Accepted");
    let member_uuid = uuid_to_string(member.person_uuid);
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![profile.clone()]])
        .append_query_results(vec![vec![owner]])
        .append_query_results(vec![vec![member]])
        .into_connection();

    let roster = TeamMemberUseCase::find_roster(&db, &profile_uuid)
        .await
        .unwrap();

    assert_eq!(roster.business_profile_id, profile.id);
    assert_eq!(roster.owner_person_uuid, owner_uuid);
    assert_eq!(roster.accepted_member_person_uuids, vec![member_uuid]);
}

#[tokio::test]
async fn ensure_accepted_member_allows_accepted_status() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![team_member(1, 2, "Accepted")]])
        .into_connection();

    TeamMemberUseCase::ensure_accepted_member(&db, 1, 2)
        .await
        .unwrap();
}

#[tokio::test]
async fn ensure_accepted_member_forbids_pending_status() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![team_member(1, 3, "Pending")]])
        .into_connection();

    let error = TeamMemberUseCase::ensure_accepted_member(&db, 1, 3)
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}
