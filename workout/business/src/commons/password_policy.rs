use crate::commons::auth_config;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PasswordPolicyViolation {
    TooShort,
    MissingUppercase,
    MissingLowercase,
    MissingDigit,
    MissingSpecial,
}

/// Validates `password` against the configured policy, returning every violated rule
/// (not just the first) so the caller can report a complete list.
///
/// A no-op returning `Ok(())` when `PASSWORD_POLICY_ENABLED`/`AUTH_RULES_ENABLED` disable
/// the policy — kept here rather than at call sites so the dev bypass has one home.
pub fn validate(password: &str) -> Result<(), Vec<PasswordPolicyViolation>> {
    if !auth_config::password_policy_enabled() {
        return Ok(());
    }

    let mut violations = Vec::new();

    if password.len() < auth_config::password_min_length() {
        violations.push(PasswordPolicyViolation::TooShort);
    }
    if auth_config::password_require_uppercase() && !password.chars().any(|c| c.is_ascii_uppercase()) {
        violations.push(PasswordPolicyViolation::MissingUppercase);
    }
    if auth_config::password_require_lowercase() && !password.chars().any(|c| c.is_ascii_lowercase()) {
        violations.push(PasswordPolicyViolation::MissingLowercase);
    }
    if auth_config::password_require_digit() && !password.chars().any(|c| c.is_ascii_digit()) {
        violations.push(PasswordPolicyViolation::MissingDigit);
    }
    if auth_config::password_require_special() && !password.chars().any(|c| !c.is_ascii_alphanumeric()) {
        violations.push(PasswordPolicyViolation::MissingSpecial);
    }

    if violations.is_empty() {
        Ok(())
    } else {
        Err(violations)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_policy_enabled<F: FnOnce()>(f: F) {
        let _guard = ENV_LOCK.lock().unwrap();
        env::remove_var("AUTH_RULES_ENABLED");
        env::remove_var("PASSWORD_POLICY_ENABLED");
        f();
    }

    #[test]
    fn strong_password_passes() {
        with_policy_enabled(|| {
            assert!(validate("Str0ng!Pass").is_ok());
        });
    }

    #[test]
    fn weak_password_reports_every_violation() {
        with_policy_enabled(|| {
            let result = validate("abc");
            assert!(result.is_err());
            let violations = result.unwrap_err();
            assert!(violations.contains(&PasswordPolicyViolation::TooShort));
            assert!(violations.contains(&PasswordPolicyViolation::MissingUppercase));
            assert!(violations.contains(&PasswordPolicyViolation::MissingDigit));
            assert!(violations.contains(&PasswordPolicyViolation::MissingSpecial));
        });
    }

    #[test]
    fn policy_disabled_allows_any_password() {
        let _guard = ENV_LOCK.lock().unwrap();
        env::set_var("PASSWORD_POLICY_ENABLED", "false");
        assert!(validate("weak").is_ok());
        env::remove_var("PASSWORD_POLICY_ENABLED");
    }
}
