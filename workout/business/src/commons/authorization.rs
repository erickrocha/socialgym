use crate::domain::business_error::BusinessError;

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
mod tests {
    use super::*;
    use crate::domain::business_error::BusinessErrorKind;

    #[test]
    fn owner_passes_and_everyone_else_is_forbidden() {
        assert!(ensure_owns(7, 7).is_ok());
        let error = ensure_owns(7, 8).unwrap_err();
        assert_eq!(error.kind, BusinessErrorKind::Forbidden);
    }
}
