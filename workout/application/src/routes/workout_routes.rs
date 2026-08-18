use axum::{middleware, Router};
use axum::routing::{get, post};
use crate::{http, AppState};
use crate::authentication::authentication_middleware::authentication;

/// Build workout-related routes
pub fn workout_routes(state: AppState) -> Router<AppState> {
	Router::new()
		.route(
			"/",
			post(http::workout_controller::add_workout).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
		.route(
			"/{person_id}",
			get(http::workout_controller::get_workouts).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
		.route(
			"/{workout_id}/exercises",
			get(http::workout_controller::get_exercises).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
		.route(
			"/{workout_id}/exercises",
			post(http::workout_controller::add_exercises).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
}