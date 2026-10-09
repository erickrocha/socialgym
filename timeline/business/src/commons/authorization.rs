use domain::business_error::BusinessError;

/// Owner-scoped access guard.
///
/// Timeline resources are keyed by the owning person's uuid. Every mutating use
/// case takes the acting person uuid and routes it through here, so no caller
/// can fall back to trusting an owner id from the request body.
pub fn ensure_owns(
    resource_owner_uuid: &str,
    acting_person_uuid: &str,
) -> Result<(), BusinessError> {
    if resource_owner_uuid == acting_person_uuid {
        return Ok(());
    }
    log::warn!(
        "[ensure_owns] Denied: person_uuid={} attempted to act on a resource owned by {}",
        acting_person_uuid,
        resource_owner_uuid
    );
    Err(BusinessError::forbidden("Not the owner of this resource"))
}

#[cfg(test)]
#[path = "../tests/authorization_unit_test.rs"]
mod tests;
