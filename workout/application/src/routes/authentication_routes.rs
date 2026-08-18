use axum::{middleware, Router};
use axum::routing::post;
use crate::AppState;
use crate::authentication::authentication_middleware::authentication;
use crate::http::auth_controller::{activate, deactivate, logout, refresh_token, sign_in, sign_up};

/// Build authentication routes (signup and login, no auth required, others auth is required)
pub fn auth_routes(state: AppState) -> Router<AppState> {
	Router::new()
		.route("/signup", post(sign_up).route_layer(middleware::from_fn_with_state(
			state.clone(),
			authentication,
		)))
		.route("/login", post(sign_in).route_layer(middleware::from_fn_with_state(
			state.clone(),
			authentication,
		)))
		.route("/refresh", post(refresh_token).route_layer(middleware::from_fn_with_state(
			state.clone(),
			authentication,
		)))
		.route("/logout", post(logout).route_layer(middleware::from_fn_with_state(
			state.clone(),
			authentication,
		)))
		.route(
			"/auth/profile/deactivate",
			post(deactivate).route_layer(middleware::from_fn_with_state(
				state.clone(),
				authentication,
			)),
		).route(
		"/auth/profile/{business_profile_uuid}/activate",
		post(activate).route_layer(middleware::from_fn_with_state(
			state.clone(),
			authentication,
		)),
	)
}