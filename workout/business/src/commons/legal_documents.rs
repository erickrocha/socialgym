use std::env;

pub const TERMS: &str = "terms";
pub const PRIVACY: &str = "privacy";
pub const HEALTH_DATA: &str = "health_data";

pub fn current_version(document: &str) -> Option<String> {
    let (key, default) = match document {
        TERMS => ("TERMS_VERSION", "1.0.0"),
        PRIVACY => ("PRIVACY_VERSION", "1.0.0"),
        HEALTH_DATA => ("HEALTH_DATA_CONSENT_VERSION", "1.0.0"),
        _ => return None,
    };
    Some(env::var(key).unwrap_or_else(|_| default.to_string()))
}

pub fn is_current(document: &str, version: &str) -> bool {
    current_version(document).as_deref() == Some(version)
}
