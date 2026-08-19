use crate::authentication::authentication_middleware::authentication;
use crate::http::friend_controller::{get_friend, get_friends};
use crate::{http, AppState};
use axum::routing::{get, put};
use axum::{middleware, Router};

/// Build friend-related routes
pub fn friend_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/relationships/id/{id}",
            get(crate::http::friend_controller::get_friend_relationships_by_id).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/relationships/uuid/{uuid}",
            get(crate::http::friend_controller::get_friend_relationships_by_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/",
            get(get_friends).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/{friend_id}",
            get(get_friend).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/request/{receiver_id}",
            put(http::friend_controller::send_friend_request).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/accept/{id}",
            put(http::friend_controller::accept_friend_request).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/deny/{id}",
            put(http::friend_controller::deny_friend_request).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/cancel/{sender_id}",
            put(http::friend_controller::cancel_friend_request).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
}
