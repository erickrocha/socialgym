use crate::authentication::authentication_middleware::authentication;
use crate::{http, AppState};
use axum::routing::post;
use axum::{middleware, Router};

pub fn post_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            post(http::post_controller::create_post).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/{post_id}/comments",
            post(http::post_controller::add_comment).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/{post_id}/reactions",
            post(http::post_controller::add_reaction)
                .delete(http::post_controller::remove_reaction)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
}
