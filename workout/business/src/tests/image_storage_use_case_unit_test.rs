use super::ImageStorageUseCase;
use crate::domain::enums::ImageType;
use crate::domain::image_storage::ImageStorage;

#[test]
fn generate_object_key_has_expected_format() {
    let extension = "-profile.jpg";
    let object_key = ImageStorageUseCase::generate_object_key(123, extension);

    assert!(object_key.starts_with("uploads/owner-123/"));
    assert!(object_key.ends_with(extension));
}

#[test]
fn generate_object_key_contains_uuid() {
    let object_key = ImageStorageUseCase::generate_object_key(456, "-cover.png");
    let parts: Vec<&str> = object_key.split('/').collect();

    assert_eq!(parts.len(), 3);
    assert_eq!(parts[0], "uploads");
    assert_eq!(parts[1], "owner-456");
    assert!(parts[2].len() > 40);
}

#[test]
fn generate_object_key_uses_unique_uuid_per_call() {
    let first = ImageStorageUseCase::generate_object_key(789, "-avatar.jpeg");
    let second = ImageStorageUseCase::generate_object_key(789, "-avatar.jpeg");

    assert_ne!(first, second);
}

#[test]
fn generate_object_key_includes_owner_id() {
    let first = ImageStorageUseCase::generate_object_key(100, "-profile.jpg");
    let second = ImageStorageUseCase::generate_object_key(200, "-profile.jpg");

    assert!(first.contains("owner-100"));
    assert!(second.contains("owner-200"));
}

#[test]
fn generate_object_key_preserves_extension() {
    for extension in [
        "-profile.jpg",
        "-cover.png",
        "-avatar.jpeg",
        "-thumbnail.gif",
    ] {
        let object_key = ImageStorageUseCase::generate_object_key(1, extension);
        assert!(object_key.ends_with(extension));
    }
}

#[test]
fn image_type_avatar_displays_expected_name() {
    assert_eq!(ImageType::Avatar.to_string(), "Avatar");
}

#[test]
fn image_type_cover_displays_expected_name() {
    assert_eq!(ImageType::Cover.to_string(), "Cover");
}

#[test]
fn generate_object_key_supports_large_owner_id() {
    let object_key = ImageStorageUseCase::generate_object_key(i32::MAX, "-profile.jpg");

    assert!(object_key.contains(&format!("owner-{}", i32::MAX)));
    assert!(object_key.starts_with("uploads/"));
}

#[test]
fn generate_object_key_supports_small_owner_id() {
    let object_key = ImageStorageUseCase::generate_object_key(1, "-profile.jpg");

    assert!(object_key.contains("owner-1"));
    assert!(object_key.starts_with("uploads/"));
}

#[test]
fn generate_object_key_supports_zero_owner_id() {
    let object_key = ImageStorageUseCase::generate_object_key(0, "-profile.jpg");

    assert!(object_key.contains("owner-0"));
    assert!(object_key.ends_with(".jpg"));
}

#[test]
fn generate_object_key_preserves_complex_extension() {
    let extension = "-profile-v2.jpg";
    let object_key = ImageStorageUseCase::generate_object_key(42, extension);

    assert!(object_key.ends_with(extension));
    assert!(object_key.contains("owner-42"));
}

#[test]
fn image_storage_new_preserves_fields() {
    let url = "https://example.com/image.jpg".to_string();
    let object_key = "uploads/owner-1/uuid.jpg".to_string();
    let storage = ImageStorage::new(url.clone(), object_key.clone(), 1);

    assert_eq!(storage.url, url);
    assert_eq!(storage.object_key, object_key);
    assert_eq!(storage.owner_id, 1);
}

#[test]
fn image_storage_implements_debug() {
    let storage = ImageStorage::new(
        "https://example.com/image.jpg".to_string(),
        "uploads/owner-1/uuid.jpg".to_string(),
        1,
    );

    assert!(format!("{storage:?}").contains("ImageStorage"));
}
