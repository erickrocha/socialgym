pub mod person_service;
pub mod friend_service;
pub mod workout_service;

use business::commons::functions::parse_uuid;
use tonic::Status;

pub(crate) fn validate_uuid(value: &str, field: &str) -> Result<(), Status> {
    parse_uuid(value)
        .map(|_| ())
        .map_err(|_| Status::invalid_argument(format!("{field} must be a valid UUID")))
}

pub(crate) fn validate_uuids(values: &[String], field: &str) -> Result<(), Status> {
    for value in values {
        validate_uuid(value, field)?;
    }
    Ok(())
}
pub mod business_profile_service;
pub mod settings_service;
pub mod exercise_service;

pub mod team_member_service;
