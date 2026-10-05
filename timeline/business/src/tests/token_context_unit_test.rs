use super::*;

#[tokio::test]
async fn returns_none_outside_scope() {
    assert_eq!(current_forwarded_token(), None);
}

#[tokio::test]
async fn returns_token_inside_scope() {
    let token = Some("test-token-123".to_string());
    let result = with_forwarded_token(token.clone(), async { current_forwarded_token() }).await;
    assert_eq!(result, token);
}

#[tokio::test]
async fn returns_none_when_scope_has_none() {
    let result = with_forwarded_token(None, async { current_forwarded_token() }).await;
    assert_eq!(result, None);
}

#[tokio::test]
async fn inner_scope_does_not_leak_to_caller() {
    with_forwarded_token(Some("inner".to_string()), async {
        assert_eq!(current_forwarded_token(), Some("inner".to_string()));
    })
    .await;
    assert_eq!(current_forwarded_token(), None);
}
