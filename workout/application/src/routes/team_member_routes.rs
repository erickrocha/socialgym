use crate::authentication::authentication_middleware::authentication;
use crate::http::team_member_controller::{
    accept_team_member_request, cancel_team_member_request, deny_team_member_request,
    get_team_member, get_team_members, send_team_member_request,
};
use crate::AppState;
use axum::routing::{get, put};
use axum::{middleware, Router};

/// Build team member related routes
pub fn team_member_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/",
            get(get_team_members).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/{person_id}",
            get(get_team_member).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/request/{person_id}",
            put(send_team_member_request).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/accept/{business_profile_id}",
            put(accept_team_member_request).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/deny/{business_profile_id}",
            put(deny_team_member_request).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/cancel/{person_id}",
            put(cancel_team_member_request).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
}
