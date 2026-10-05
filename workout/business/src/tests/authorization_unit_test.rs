use super::*;
use crate::domain::business_error::BusinessErrorKind;

#[test]
fn owner_passes_and_everyone_else_is_forbidden() {
    assert!(ensure_owns(7, 7).is_ok());
    let error = ensure_owns(7, 8).unwrap_err();
    assert_eq!(error.kind, BusinessErrorKind::Forbidden);
}
