use crate::authentication::authentication_middleware::authentication;
use crate::{http, AppState};
use axum::routing::{get, post};
use axum::{middleware, Router};

pub fn evolution_checkin_routes(state: AppState) -> Router<AppState> {
	Router::new()
		.route(
			"/",
			post(http::evolution_controller::add).route_layer(middleware::from_fn_with_state(
				state.clone(),
				authentication,
			)),
		).route(
		"/",
		get(http::evolution_controller::get_by_owner).route_layer(middleware::from_fn_with_state(
			state.clone(),
			authentication,
		)),
	)
}