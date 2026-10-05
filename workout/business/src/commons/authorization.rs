use crate::domain::business_error::BusinessError;
use crate::domain::business_profile::BusinessProfile;
use crate::domain::user::User;

/// Owner-scoped access guard.
///
/// Authority in this domain is delegated by consent, never implied: a principal
/// may only act on a resource it owns. Every mutating use case takes the acting
/// person id and routes it through here, so no caller (HTTP or gRPC) can forget
/// the check.
pub fn ensure_owns(resource_owner_id: i32, acting_person_id: i32) -> Result<(), BusinessError> {
    if resource_owner_id == acting_person_id {
        return Ok(());
    }
    log::warn!(
        "[ensure_owns] Denied: person_id={} attempted to act on a resource owned by person_id={}",
        acting_person_id,
        resource_owner_id
    );
    Err(BusinessError::forbidden("Not the owner of this resource"))
}

#[cfg(test)]
#[path = "../tests/authorization_unit_test.rs"]
mod tests;

/// The identity a request acts as: the caller's Person, or the Active Business
/// Profile when one is active. Owned records store only that id/uuid, so ownership
/// must match on the uuid as well; a Person id and a Business Profile id can be
/// the same number but their uuids never are.
#[derive(Debug, Clone, PartialEq)]
pub struct ActingOwner {
    pub id: i32,
    pub uuid: String,
}

impl ActingOwner {
    pub fn new(actor: &User, active_profile: Option<&BusinessProfile>) -> Self {
        Self {
            id: active_profile.and_then(|p| p.id).unwrap_or(actor.person_id),
            uuid: active_profile
                .and_then(|p| p.uuid.clone())
                .unwrap_or_else(|| actor.person_uuid.clone()),
        }
    }

    pub fn owns(&self, owner_id: i32, owner_uuid: &str) -> bool {
        self.id == owner_id && self.uuid == owner_uuid
    }
}

/// Like [`ensure_owns`], for records that store the owner's id and uuid.
pub fn ensure_owns_as(
    resource_owner_id: i32,
    resource_owner_uuid: &str,
    acting: &ActingOwner,
) -> Result<(), BusinessError> {
    if acting.owns(resource_owner_id, resource_owner_uuid) {
        return Ok(());
    }
    log::warn!(
        "[ensure_owns_as] Denied: acting id={} on a resource owned by id={}",
        acting.id,
        resource_owner_id
    );
    Err(BusinessError::forbidden("Not the owner of this resource"))
}
