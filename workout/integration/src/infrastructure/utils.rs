
use business::commons::authorization::ActingOwner;
use business::commons::i18n::{ErrorKey, Locale, translate};
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

/// The identity the request acts as: the Person, or the Active Business Profile.
pub(crate) fn require_acting_owner<T>(request: &Request<T>) -> Result<ActingOwner, Status> {
    let actor = require_actor(request)?;
    Ok(ActingOwner::new(&actor, require_active_profile(request).as_ref()))
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
        // A failing dependency (database, queue, object store) is retryable; REST answers 500 for it.
        BusinessErrorKind::Infrastructure => Status::unavailable(error.message),
    }
}

/// The locale the caller asked for in the `accept-language` metadata (English when absent).
pub(crate) fn locale_of<T>(request: &Request<T>) -> Locale {
    Locale::from_accept_language(request.metadata().get("accept-language").and_then(|value| value.to_str().ok()))
}

/// A status whose message is the REST error text in the caller's locale and whose `error-key` trailer is
/// the stable REST `errorKey`, so a client tells the cases apart without parsing the text.
pub(crate) fn localized_status(code: tonic::Code, key: ErrorKey, locale: Locale) -> Status {
    let mut status = Status::new(code, translate(locale, key));
    if let Ok(value) = key.as_str().parse() {
        status.metadata_mut().insert("error-key", value);
    }
    status
}

/// The status of a business failure with the message the REST form shows for `key`, in the caller's
/// locale; the code follows the kind of the error (the same table as `business_status`).
pub(crate) fn localized_business_status(error: BusinessError, key: ErrorKey, locale: Locale) -> Status {
    let code = business_status(error).code();
    localized_status(code, key, locale)
}

/// A caller that exceeded the per-IP limit (REST answers 429).
#[allow(dead_code)]
pub(crate) fn rate_limited() -> Status {
    Status::resource_exhausted("too many requests")
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
#[cfg(test)]
#[path = "../tests/status_mapping_unit_test.rs"]
mod status_tests;
