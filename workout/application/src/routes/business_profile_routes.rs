use crate::authentication::authentication_middleware::authentication;
use crate::http::business_profile_controller::{get_active, get_by_owner_id};
use crate::http::image_controller::business_profile_image_upload;
use crate::AppState;
use axum::routing::{get, post};
use axum::{middleware, Router};

/// Build business profile routes
pub fn business_profile_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/id/{id}",
            get(crate::http::business_profile_controller::get_profile_by_id).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}",
            get(crate::http::business_profile_controller::get_profile_by_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/owner/id/{id}",
            get(crate::http::business_profile_controller::get_profiles_by_owner_id).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/owner/uuid/{uuid}",
            get(crate::http::business_profile_controller::get_profiles_by_owner_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/",
            post(crate::http::business_profile_controller::add_profile)
                .put(crate::http::business_profile_controller::update_profile)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
        .route(
            "/addresses",
            post(crate::http::business_profile_controller::save_address)
                .put(crate::http::business_profile_controller::save_address)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
        .route(
            "/addresses/id/{id}",
            axum::routing::delete(crate::http::business_profile_controller::delete_address_by_id)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
        .route(
            "/addresses/uuid/{uuid}",
            axum::routing::delete(crate::http::business_profile_controller::delete_address_by_uuid)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                )),
        )
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
