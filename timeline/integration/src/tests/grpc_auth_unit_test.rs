use super::*;

#[test]
fn secrets_match_only_when_equal() {
    assert!(secrets_match("abc", "abc"));
    assert!(!secrets_match("abc", "abd"));
    assert!(!secrets_match("abc", "abcd"));
    assert!(!secrets_match("", "abc"));
    assert!(secrets_match("", ""));
}

#[test]
fn the_token_never_appears_in_debug_output() {
    assert!(!format!("{:?}", BearerToken("secret-token".into())).contains("secret-token"));
}
