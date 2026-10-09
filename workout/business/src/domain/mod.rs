pub mod access_token;
pub mod address_candidate;
pub mod business_error;
pub mod business_profile;
pub mod business_profile_address;
pub mod country;
pub mod enums;
pub mod exercise;
pub mod friend;
pub mod friendship_outbox_event;
pub mod image_storage;
pub mod person;
pub mod person_address;
pub mod person_info;
pub mod person_media;
pub mod profile;
pub mod revoked_token;
pub mod settings;
pub mod team_member;
pub mod user;
pub mod workout;
pub mod workout_exercise;

/// Stored rows that use cases hand to the transports as they are (consents, data exports), re-exported
/// so `integration` does not depend on the `entity` crate.
pub mod records {
    pub use entity::consent_entity::Model as ConsentRecord;
    pub use entity::data_export_entity::Model as DataExportRecord;
}
