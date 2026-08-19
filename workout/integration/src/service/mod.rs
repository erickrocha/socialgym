pub mod friend_service;
pub mod person_service;
pub mod workout_service;

use business::commons::functions::parse_uuid;
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use tonic::Status;

pub(crate) fn business_status(error: BusinessError) -> Status {
    match error.kind {
        BusinessErrorKind::Validation => Status::invalid_argument(error.message),
        BusinessErrorKind::Unauthorized => Status::unauthenticated(error.message),
        BusinessErrorKind::Forbidden => Status::permission_denied(error.message),
        BusinessErrorKind::NotFound => Status::not_found(error.message),
        BusinessErrorKind::Conflict => Status::already_exists(error.message),
        BusinessErrorKind::Locked => Status::failed_precondition(error.message),
        BusinessErrorKind::Infrastructure => Status::internal(error.message),
    }
}

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
pub mod exercise_service;
pub mod settings_service;

pub mod team_member_service;
