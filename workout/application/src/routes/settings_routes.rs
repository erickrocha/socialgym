use crate::authentication::authentication_middleware::authentication;
use crate::http::settings_controller::{
    create_settings, get_my_settings, get_settings_by_id, get_settings_by_uuid, update_my_settings,
};
use crate::AppState;
use axum::routing::{get, post, put};
use axum::{middleware, Router};

/// Build settings-related routes
pub fn settings_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/owner/id/{id}",
            get(crate::http::settings_controller::get_settings_by_owner_id).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/owner/uuid/{uuid}",
            get(crate::http::settings_controller::get_settings_by_owner_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/me",
            get(get_my_settings).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me",
            put(update_my_settings).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/{id}",
            get(get_settings_by_id).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/uuid/{uuid}",
            get(get_settings_by_uuid).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/",
            post(create_settings).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
}
