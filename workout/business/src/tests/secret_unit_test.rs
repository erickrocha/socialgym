use super::constant_time_eq;

#[test]
fn equal_secrets_match_and_any_difference_does_not() {
    assert!(constant_time_eq("c010-internal-secret", "c010-internal-secret"));
    assert!(!constant_time_eq("c010-internal-secret", "c010-internal-secreT"), "last byte differs");
    assert!(!constant_time_eq("c010-internal-secret", "C010-internal-secret"), "first byte differs");
    assert!(!constant_time_eq("secret", "secret-and-more"), "different lengths");
    assert!(!constant_time_eq("", "x"));
    assert!(constant_time_eq("", ""), "callers reject an empty configured secret before comparing");
}
