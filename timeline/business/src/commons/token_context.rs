/// Request-local JWT forwarding context.
///
/// Uses a task-local variable so the raw token string from the HTTP layer can
/// be read by gRPC interceptors lower in the call stack without passing it
/// through every function signature.
use std::future::Future;

tokio::task_local! {
    /// Holds the raw Bearer token string for the current async task.
    static FORWARDED_TOKEN: Option<String>;
}

/// Run `fut` inside a scope where [`current_forwarded_token`] returns `token`.
///
/// Call this once per authenticated request, wrapping the `next` handler
/// execution so every gRPC call spawned from that handler can read the token.
pub fn with_forwarded_token<F: Future>(
    token: Option<String>,
    fut: F,
) -> impl Future<Output = F::Output> {
    FORWARDED_TOKEN.scope(token, fut)
}

/// Return the forwarded token for the currently executing async task, if any.
///
/// Returns `None` when called outside a [`with_forwarded_token`] scope or when
/// the scope was created with `None`.
pub fn current_forwarded_token() -> Option<String> {
    FORWARDED_TOKEN.try_with(|t: &Option<String>| t.clone()).unwrap_or(None)
}

#[cfg(test)]
mod tests {
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
        // outside the scope the value is gone
        assert_eq!(current_forwarded_token(), None);
    }
}

