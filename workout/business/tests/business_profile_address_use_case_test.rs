use business::domain::business_error::BusinessErrorKind;
use business::domain::business_profile_address::BusinessProfileAddress;
use business::use_cases::business_profile_address_use_case::BusinessProfileAddressUseCase;
use entity::business_profile_address_entity::BusinessProfileAddressEntity;
use entity::business_profile_entity::BusinessProfileEntity;
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

#[tokio::test]
async fn save_forbids_non_owner_of_business_profile() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![business_profile(1)]])
        .append_query_results(vec![Vec::<BusinessProfileAddressEntity>::new()])
        .into_connection();
    let address = BusinessProfileAddress::new(
        1,
        "1 Main St".to_string(),
        None,
        "Boston".to_string(),
        "MA".to_string(),
        Some("02101".to_string()),
        "USA".to_string(),
    );

    let error = BusinessProfileAddressUseCase::save(&db, address, 2, Some(42.3601), Some(-71.0589))
        .await
        .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}
