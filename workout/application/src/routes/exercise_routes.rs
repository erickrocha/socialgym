use axum::{middleware, Router};
use axum::routing::{delete, post};
use crate::{http, AppState};
use crate::authentication::authentication_middleware::authentication;

/// Build exercise-related routes
pub fn exercise_routes(state: AppState) -> Router<AppState> {
	Router::new()
		.route(
			"/",
			post(http::exercise_controller::add_exercise).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
		.route(
			"/query",
			post(http::exercise_controller::query_exercises).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
		.route(
			"/{exercise_id}",
			delete(http::exercise_controller::delete_exercise).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
}