use crate::authentication::authentication_middleware::authentication;
use crate::{AppState, http};
use axum::Router;
use axum::middleware;
use axum::routing::{delete, put};

pub fn push_device_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/{device_uuid}",
            put(http::push_device_controller::register_push_device).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/{device_uuid}",
            delete(http::push_device_controller::remove_push_device)
                .route_layer(middleware::from_fn_with_state(state, authentication)),
        )
}
