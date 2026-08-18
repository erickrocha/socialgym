use axum::{middleware, Router};
use axum::routing::get;
use crate::AppState;
use crate::authentication::authentication_middleware::authentication;
use crate::http::feed_controller::{get_feed, get_feed_by_uuid};

pub fn feed_route(state :AppState) -> Router<AppState> {
	Router::new()
		.route(
			"/",
			get(get_feed).route_layer(middleware::from_fn_with_state(
				state.clone(),
				authentication,
			)),
		).route(
			"/{business_profile_uuid}",
			get(get_feed_by_uuid).route_layer(middleware::from_fn_with_state(
				state.clone(),
				authentication,
			)),
		)
}