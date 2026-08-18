use crate::authentication::authentication_middleware::authentication;
use crate::http::business_profile_controller::{get_active, get_by_owner_id};
use crate::http::image_controller::business_profile_image_upload;
use crate::AppState;
use axum::routing::get;
use axum::{middleware, Router};

/// Build business profile routes
pub fn business_profile_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_by_owner_id).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )

        .route(
            "/active",
            get(get_active).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/upload/{image_type}",
            get(business_profile_image_upload).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )

}
