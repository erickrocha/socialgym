
use business::commons::functions::parse_uuid;
use business::domain::business_error::{BusinessError, BusinessErrorKind};
use business::domain::business_profile::BusinessProfile;
use business::domain::user::User;
use tonic::{Request, Status};

/// The authenticated principal, as injected by `GrpcAuthLayer`.
pub(crate) fn require_actor<T>(request: &Request<T>) -> Result<User, Status> {
    request
        .extensions()
        .get::<User>()
        .cloned()
        .ok_or_else(|| Status::unauthenticated("missing authenticated user"))
}

/// The caller's active business profile, if any — injected by `GrpcAuthLayer`
/// alongside `User` when the JWT carries an `active_business_profile_id`.
pub(crate) fn require_active_profile<T>(request: &Request<T>) -> Option<BusinessProfile> {
    request.extensions().get::<BusinessProfile>().cloned()
}

pub(crate) fn require_person_id<T>(request: &Request<T>) -> Result<i32, Status> {
    require_actor(request).map(|user| user.person_id)
}

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