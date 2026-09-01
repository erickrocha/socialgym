use crate::authentication::authentication_middleware::authentication;
use crate::{http, AppState};
use axum::routing::{get, post, put};
use axum::{middleware, Router};

/// Build workout-related routes
pub fn workout_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/id/{id}",
            get(http::workout_controller::get_workout_by_id)
                .delete(http::workout_controller::delete_workout_by_id)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
        .route(
            "/uuid/{uuid}",
            get(http::workout_controller::get_workout_by_uuid)
                .delete(http::workout_controller::delete_workout_by_uuid)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
        .route(
            "/owner/uuid/{uuid}",
            get(http::workout_controller::get_workouts_by_owner_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/assigned-by-profile",
            get(http::workout_controller::get_workouts_assigned_by_profile).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}/exercises",
            post(http::workout_controller::add_exercises_by_workout_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}/accept",
            put(http::workout_controller::accept_workout_assignment).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}/reject",
            put(http::workout_controller::reject_workout_assignment).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}/cancel",
            put(http::workout_controller::cancel_workout_assignment).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/",
            put(http::workout_controller::update_workout).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
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
