use crate::authentication::authentication_middleware::authentication;
use crate::{http, AppState};
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};

/// Build exercise-related routes
pub fn exercise_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/id/{id}",
            get(http::exercise_controller::get_exercise_by_id).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}",
            get(http::exercise_controller::get_exercise_by_uuid)
                .delete(http::exercise_controller::delete_exercise_by_uuid)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
        .route(
            "/",
            put(http::exercise_controller::update_exercise).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
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
