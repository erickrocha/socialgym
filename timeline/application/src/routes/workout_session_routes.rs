use crate::authentication::authentication_middleware::authentication;
use crate::{http, AppState};
use axum::routing::{get, post};
use axum::{middleware, Router};

pub fn workout_session_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/{workout_session_id}",
            get(http::workout_session_controller::get_workout_session).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/",
            get(http::workout_session_controller::get_workout_sessions).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/",
            post(http::workout_session_controller::create_workout_session).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
}
