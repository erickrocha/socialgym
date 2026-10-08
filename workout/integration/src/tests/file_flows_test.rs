//! C-010 task 5: the gRPC upload links (TC-007): person image, business-profile logo and cover, post
//! media. No file bytes travel over gRPC, only a short-lived pre-signed link. Handler level, against a
//! disposable PostGIS database (`TEST_DATABASE_URL`).
use super::server_support::error_key;
use crate::proto::business_profile::business_profile_service_server::BusinessProfileService;
use crate::proto::business_profile::BusinessProfileImageUploadRequest;
use crate::proto::media::media_service_server::MediaService;
use crate::proto::media::MediaUploadRequest;
use crate::proto::person::person_service_server::PersonService;
use crate::proto::person::PersonImageUploadRequest;
use crate::service::business_profile_service::GrpcBusinessProfileService;
use crate::service::media_service::GrpcMediaService;
use crate::service::person_service::GrpcPersonService;
use business::commons::i18n::ErrorKey;
use business::domain::user::User;
use business::use_cases::business_profile_use_case::BusinessProfileUseCase;
use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection};
use std::sync::Arc;
use tonic::{Code, Request};

const ALICE: &str = "00000000-0000-0000-0000-0000000000d1";
const PROFILE: &str = "30000000-0000-0000-0000-0000000000d1";

fn as_person<T>(message: T) -> Request<T> {
    let mut request = Request::new(message);
    request.extensions_mut().insert(User::new(Some("Alice".into()), "alice@example.test".into(), "hashed".into(), 1, ALICE.into()));
    request
}

async fn world() -> Arc<DatabaseConnection> {
    unsafe {
        std::env::set_var("AWS_WORKOUT_BUCKET", "c010-file-bucket");
        std::env::set_var("AWS_REGION", "us-east-1");
        std::env::set_var("AWS_ACCESS_KEY_ID", "test");
        std::env::set_var("AWS_SECRET_ACCESS_KEY", "test");
    }
    let url = std::env::var("TEST_DATABASE_URL").expect("TEST_DATABASE_URL must point to a disposable PostGIS database");
    let db = Database::connect(url).await.unwrap();
    Migrator::refresh(&db).await.unwrap();
    db.execute_unprepared(&format!(
        r#"INSERT INTO person (id, uuid, first_name, surname, date_of_birth, gender, created_at, updated_at)
             VALUES (1, '{ALICE}', 'Alice', 'Owner', '1990-01-01', 'X', now(), now());
           INSERT INTO business_profile (id, uuid, owner_id, owner_uuid, tax_id, business_name, business_type, created_at, updated_at)
             VALUES (1, '{PROFILE}', 1, '{ALICE}', '999', 'Alice Gym', 'Professional', now(), now())"#
    ))
    .await
    .unwrap();
    Arc::new(db)
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn the_upload_links_point_at_the_store_and_carry_no_file() {
    let db = world().await;

    let person = GrpcPersonService::new(db.clone())
        .get_person_image_upload_url(as_person(PersonImageUploadRequest { image_type: "avatar".into(), format: String::new() }))
        .await
        .expect("person image link")
        .into_inner();
    assert!(person.url.contains("c010-file-bucket") && person.object_key.contains(ALICE), "{} {}", person.url, person.object_key);
    assert_eq!(person.person_id, 1);

    // The logo belongs to the Active Business Profile of the token (the layer loads it).
    let profile = BusinessProfileUseCase::get_by_id(&db, 1).await.expect("profile");
    let mut request = as_person(BusinessProfileImageUploadRequest { image_type: "logo".into(), format: "png".into() });
    request.extensions_mut().insert(profile.clone());
    let logo = GrpcBusinessProfileService::new(db.clone()).get_business_profile_image_upload_url(request).await.expect("logo link").into_inner();
    assert!(logo.url.contains("c010-file-bucket") && logo.object_key.contains(PROFILE), "{}", logo.object_key);
    assert_eq!(logo.business_profile_id, 1);

    // The app stores empty strings for the image keys of a new profile; they count as no key (before the
    // fix the link could not be signed and the call failed on both transports).
    db.execute_unprepared("UPDATE business_profile SET logo = '', cover_image = '' WHERE id = 1").await.unwrap();
    for image_type in ["logo", "cover"] {
        let mut request = as_person(BusinessProfileImageUploadRequest { image_type: image_type.into(), format: "image/png".into() });
        request.extensions_mut().insert(profile.clone());
        let link = GrpcBusinessProfileService::new(db.clone()).get_business_profile_image_upload_url(request).await.expect("link for empty keys").into_inner();
        assert!(link.object_key.contains(PROFILE), "{}", link.object_key);
    }

    let media = GrpcMediaService::new(db)
        .get_post_media_upload_url(as_person(MediaUploadRequest { album: "gallery".into(), format: String::new() }))
        .await
        .expect("media link")
        .into_inner();
    assert!(media.url.contains("c010-file-bucket") && media.object_key.contains("gallery"), "{}", media.object_key);
}

#[tokio::test]
#[ignore = "requires a disposable TEST_DATABASE_URL"]
async fn a_business_profile_link_needs_an_active_profile_and_media_needs_an_album() {
    let db = world().await;

    let none = GrpcBusinessProfileService::new(db.clone())
        .get_business_profile_image_upload_url(as_person(BusinessProfileImageUploadRequest { image_type: "logo".into(), format: String::new() }))
        .await
        .unwrap_err();
    assert_eq!((none.code(), error_key(&none)), (Code::FailedPrecondition, ErrorKey::BusinessProfilePreSignedUrlNotGenerated.as_str()), "REST answers 400 for the same condition");

    let no_album = GrpcMediaService::new(db)
        .get_post_media_upload_url(as_person(MediaUploadRequest { album: String::new(), format: "image/png".into() }))
        .await
        .unwrap_err();
    assert_eq!((no_album.code(), error_key(&no_album)), (Code::InvalidArgument, ErrorKey::RequiredParameterMissing.as_str()));
}
