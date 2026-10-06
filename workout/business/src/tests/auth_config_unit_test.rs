use super::*;
fn with_env<F: FnOnce()>(vars: &[(&str, Option<&str>)], f: F) {
    let _guard = crate::commons::lock_env();
    for (name, value) in vars {
        match value {
            Some(value) => env::set_var(name, value),
            None => env::remove_var(name),
        }
    }
    f();
    for (name, _) in vars {
        env::remove_var(name);
    }
}

#[test]
fn defaults_to_enabled_when_nothing_set() {
    with_env(
        &[
            (AUTH_RULES_ENABLED, None),
            (PASSWORD_POLICY_ENABLED, None),
            (LOGIN_LOCKOUT_ENABLED, None),
            (TOKEN_REVOCATION_ENABLED, None),
        ],
        || {
            assert!(password_policy_enabled());
            assert!(login_lockout_enabled());
            assert!(token_revocation_enabled());
        },
    );
}

#[test]
fn master_switch_disables_all_features() {
    with_env(
        &[
            (AUTH_RULES_ENABLED, Some("false")),
            (PASSWORD_POLICY_ENABLED, None),
            (LOGIN_LOCKOUT_ENABLED, None),
            (TOKEN_REVOCATION_ENABLED, None),
        ],
        || {
            assert!(!password_policy_enabled());
            assert!(!login_lockout_enabled());
            assert!(!token_revocation_enabled());
        },
    );
}

#[test]
fn per_feature_override_wins_over_master_switch() {
    with_env(
        &[
            (AUTH_RULES_ENABLED, Some("false")),
            (LOGIN_LOCKOUT_ENABLED, Some("true")),
        ],
        || assert!(login_lockout_enabled()),
    );
}

#[test]
fn numeric_defaults_are_sane() {
    with_env(
        &[
            (PASSWORD_MIN_LENGTH, None),
            (LOGIN_MAX_FAILED_ATTEMPTS, None),
            (LOGIN_LOCKOUT_DURATION_SECONDS, None),
        ],
        || {
            assert_eq!(password_min_length(), 8);
            assert_eq!(login_max_failed_attempts(), 3);
            assert_eq!(login_lockout_duration_seconds(), 900);
        },
    );
}
