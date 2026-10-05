use super::*;
use domain::business_error::BusinessErrorKind;

#[test]
fn owner_passes_and_everyone_else_is_forbidden() {
    assert!(ensure_owns("a", "a").is_ok());
    assert_eq!(
        ensure_owns("a", "b").unwrap_err().kind,
        BusinessErrorKind::Forbidden
    );
}
