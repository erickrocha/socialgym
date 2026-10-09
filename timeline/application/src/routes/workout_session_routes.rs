use crate::authentication::authentication_middleware::authentication;
use crate::authentication::rate_limit::rate_limit;
use crate::{AppState, http};
use axum::routing::{get, post};
use axum::{Router, middleware};
use business::commons::rate_limit::content_limiter;

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
            post(http::workout_session_controller::create_workout_session)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                ))
                .route_layer(middleware::from_fn_with_state(
                    content_limiter(),
                    rate_limit,
                )),
        )
}
