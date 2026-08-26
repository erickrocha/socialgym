use crate::authentication::internal_auth_middleware::internal_authentication;
use crate::{http, AppState};
use axum::routing::delete;
use axum::{middleware, Router};

/// Service-to-service routes, authenticated via a shared secret header
/// instead of the end-user JWT middleware. Never exposed to end users.
pub fn internal_routes(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/persons/{person_uuid}",
        delete(http::internal_controller::delete_person_data).route_layer(
            middleware::from_fn_with_state(state.clone(), internal_authentication),
        ),
    )
}
