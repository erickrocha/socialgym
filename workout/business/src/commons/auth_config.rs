use std::env;

const AUTH_RULES_ENABLED: &str = "AUTH_RULES_ENABLED";
const PASSWORD_POLICY_ENABLED: &str = "PASSWORD_POLICY_ENABLED";
const LOGIN_LOCKOUT_ENABLED: &str = "LOGIN_LOCKOUT_ENABLED";
const TOKEN_REVOCATION_ENABLED: &str = "TOKEN_REVOCATION_ENABLED";

const PASSWORD_MIN_LENGTH: &str = "PASSWORD_MIN_LENGTH";
const PASSWORD_REQUIRE_UPPERCASE: &str = "PASSWORD_REQUIRE_UPPERCASE";
const PASSWORD_REQUIRE_LOWERCASE: &str = "PASSWORD_REQUIRE_LOWERCASE";
const PASSWORD_REQUIRE_DIGIT: &str = "PASSWORD_REQUIRE_DIGIT";
const PASSWORD_REQUIRE_SPECIAL: &str = "PASSWORD_REQUIRE_SPECIAL";

const LOGIN_MAX_FAILED_ATTEMPTS: &str = "LOGIN_MAX_FAILED_ATTEMPTS";
const LOGIN_LOCKOUT_DURATION_SECONDS: &str = "LOGIN_LOCKOUT_DURATION_SECONDS";

/// `AUTH_RULES_ENABLED` is the dev on/off switch: when unset or `true` (the default),
/// every feature below is active unless its own env var explicitly overrides it.
fn master_enabled() -> bool {
    env::var(AUTH_RULES_ENABLED)
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true)
}

fn resolve_flag(var_name: &str, master: bool) -> bool {
    env::var(var_name)
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(master)
}

pub fn password_policy_enabled() -> bool {
    resolve_flag(PASSWORD_POLICY_ENABLED, master_enabled())
}

pub fn login_lockout_enabled() -> bool {
    resolve_flag(LOGIN_LOCKOUT_ENABLED, master_enabled())
}

pub fn token_revocation_enabled() -> bool {
    resolve_flag(TOKEN_REVOCATION_ENABLED, master_enabled())
}

pub fn password_min_length() -> usize {
    env::var(PASSWORD_MIN_LENGTH)
        .ok()
        .and_then(|s| s.parse::<usize>().ok())
        .unwrap_or(8)
}

pub fn password_require_uppercase() -> bool {
    env::var(PASSWORD_REQUIRE_UPPERCASE)
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true)
}

pub fn password_require_lowercase() -> bool {
    env::var(PASSWORD_REQUIRE_LOWERCASE)
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true)
}

pub fn password_require_digit() -> bool {
    env::var(PASSWORD_REQUIRE_DIGIT)
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true)
}

pub fn password_require_special() -> bool {
    env::var(PASSWORD_REQUIRE_SPECIAL)
        .ok()
        .and_then(|s| s.parse::<bool>().ok())
        .unwrap_or(true)
}

pub fn login_max_failed_attempts() -> u32 {
    env::var(LOGIN_MAX_FAILED_ATTEMPTS)
        .ok()
        .and_then(|s| s.parse::<u32>().ok())
        .unwrap_or(3)
}

pub fn login_lockout_duration_seconds() -> i64 {
    env::var(LOGIN_LOCKOUT_DURATION_SECONDS)
        .ok()
        .and_then(|s| s.parse::<i64>().ok())
        .unwrap_or(900)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Mutex;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn with_env<F: FnOnce()>(vars: &[(&str, Option<&str>)], f: F) {
        let _guard = ENV_LOCK.lock().unwrap();
        for (name, value) in vars {
            match value {
                Some(v) => env::set_var(name, v),
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
            || {
                assert!(login_lockout_enabled());
            },
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
}
