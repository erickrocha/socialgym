mod support;

use business::use_cases::image_storage_use_case::ImageStorageUseCase;
use support::{aws_env, localstack_bucket_and_queue};

fn rsa_pem() -> Option<String> {
    let out = std::process::Command::new("openssl").args(["genrsa", "-traditional", "2048"]).output().ok()?;
    out.status.success().then(|| String::from_utf8(out.stdout).ok()).flatten()
}

#[tokio::test]
#[ignore = "requires the infra/test LocalStack (S3 on :4566)"]
async fn objects_are_uploaded_signed_downloaded_and_deleted() {
    aws_env();
    let (bucket, _queue) = localstack_bucket_and_queue().await;
    let key = format!("exports/{}.zip", uuid::Uuid::new_v4());

    // export upload / download round trip, and a download link scoped to the object
    ImageStorageUseCase::upload_export(&key, b"zip-bytes".to_vec()).await.unwrap();
    assert_eq!(ImageStorageUseCase::download_object(&key).await.unwrap(), b"zip-bytes");
    let link = ImageStorageUseCase::export_download_url(&key).await.unwrap();
    assert!(link.contains(&bucket) && link.contains("X-Amz-Signature"), "{link}");
    let body = reqwest::get(&link).await.unwrap();
    assert!(body.status().is_success());
    assert_eq!(body.bytes().await.unwrap().as_ref(), b"zip-bytes");

    // pre-signed PUT URLs (album path and legacy path) accept an upload
    let album = ImageStorageUseCase::generate_presigned_url("person".into(), 7, "person-uuid", "avatar", "image/png").await.unwrap();
    assert!(album.object_key.starts_with("person/person-uuid/avatar/"));
    let put = reqwest::Client::new().put(&album.url).header("content-type", "image/png").body("png").send().await.unwrap();
    assert!(put.status().is_success());
    let legacy = ImageStorageUseCase::update_presigned_url(7, ImageStorageUseCase::generate_object_key(7, ".jpg"), "image/jpeg".into()).await.unwrap();
    assert!(legacy.object_key.starts_with("uploads/owner-7/"));
    assert_eq!(legacy.owner_id, 7);

    // delete removes the object; deleting again is not an error for S3
    ImageStorageUseCase::delete_presigned_url(key.clone()).await.unwrap();
    assert!(ImageStorageUseCase::download_object(&key).await.is_err());
    ImageStorageUseCase::delete_presigned_url(key).await.unwrap();

    // CloudFront signing is offline: it needs only the key pair, never the network
    let Some(pem) = rsa_pem() else { return };
    unsafe {
        std::env::set_var("CLOUDFRONT_KEY_PAIR_ID", "KTESTUSECASE");
        std::env::set_var("CLOUDFRONT_DOMAIN", "https://cdn.example.test");
        std::env::set_var("PRIVATE_KEY_RAW", pem.replace('\n', "\\n"));
    }
    let signed = ImageStorageUseCase::generate_cloud_front_signed_url("person/a/avatar/x").await.unwrap();
    assert!(signed.starts_with("https://cdn.example.test/person/a/avatar/x?") && signed.contains("Key-Pair-Id=KTESTUSECASE"), "{signed}");
    unsafe { std::env::set_var("PRIVATE_KEY_RAW", "not a key") };
    assert!(ImageStorageUseCase::generate_cloud_front_signed_url("x").await.is_err());
}
