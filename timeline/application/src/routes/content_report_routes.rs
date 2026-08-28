use crate::authentication::authentication_middleware::authentication;
use crate::{AppState, http};
use axum::routing::{get, post};
use axum::{Router, middleware};

pub fn report_routes(state: AppState) -> Router<AppState> {
    Router::new().route(
        "/",
        post(http::content_report_controller::create).route_layer(middleware::from_fn_with_state(
            state.clone(),
            authentication,
        )),
    )
}
pub fn moderation_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/reports",
            get(http::content_report_controller::list).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/reports/{id}/decision",
            post(http::content_report_controller::decide).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
}
