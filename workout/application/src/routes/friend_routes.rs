use axum::{middleware, Router};
use axum::routing::{get, put};
use crate::{http, AppState};
use crate::authentication::authentication_middleware::authentication;
use crate::http::friend_controller::{get_friend, get_friends};

/// Build friend-related routes
pub fn friend_routes(state: AppState) -> Router<AppState> {
	Router::new()
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