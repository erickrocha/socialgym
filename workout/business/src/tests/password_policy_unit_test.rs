use super::*;
use std::env;

fn with_policy_enabled<F: FnOnce()>(f: F) {
    let _guard = crate::commons::lock_env();
    env::remove_var("AUTH_RULES_ENABLED");
    env::remove_var("PASSWORD_POLICY_ENABLED");
    f();
}

#[test]
fn strong_password_passes() {
    with_policy_enabled(|| assert!(validate("Str0ng!Pass").is_ok()));
}

#[test]
fn weak_password_reports_every_violation() {
    with_policy_enabled(|| {
        let violations = validate("abc").unwrap_err();
        assert!(violations.contains(&PasswordPolicyViolation::TooShort));
        assert!(violations.contains(&PasswordPolicyViolation::MissingUppercase));
        assert!(violations.contains(&PasswordPolicyViolation::MissingDigit));
        assert!(violations.contains(&PasswordPolicyViolation::MissingSpecial));
    });
}

#[test]
fn policy_disabled_allows_any_password() {
    let _guard = crate::commons::lock_env();
    env::set_var("PASSWORD_POLICY_ENABLED", "false");
    assert!(validate("weak").is_ok());
    env::remove_var("PASSWORD_POLICY_ENABLED");
}
