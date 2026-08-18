use axum::{middleware, Router};
use axum::routing::{get, put};
use crate::{http, AppState};
use crate::authentication::authentication_middleware::authentication;

pub fn notification_routes(state: AppState) -> Router<AppState> {
	Router::new()
		.route(
			"/{owner_uuid}",
			get(http::notification_controller::list_notifications).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
		.route(
			"/{owner_uuid}/read/{idempotency_key}",
			put(http::notification_controller::mark_notification_read).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
}