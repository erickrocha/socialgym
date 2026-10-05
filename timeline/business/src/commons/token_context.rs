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
#[path = "../tests/token_context_unit_test.rs"]
mod tests;

