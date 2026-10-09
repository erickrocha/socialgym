use crate::authentication::authentication_middleware::authentication;
use crate::authentication::rate_limit::{auth_limiter, rate_limit, refresh_limiter};
use crate::http::auth_controller::{activate, deactivate, logout, refresh_token, sign_in, sign_up};
use crate::AppState;
use axum::routing::post;
use axum::{middleware, Router};

/// Build authentication routes; refresh uses a refresh token, other protected routes use access authentication.
pub fn auth_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/signup",
            post(sign_up)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                ))
                .route_layer(middleware::from_fn_with_state(auth_limiter(), rate_limit)),
        )
        .route(
            "/login",
            post(sign_in)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                ))
                .route_layer(middleware::from_fn_with_state(auth_limiter(), rate_limit)),
        )
        .route(
            "/refresh",
            post(refresh_token)
                .route_layer(middleware::from_fn_with_state(
                    state.clone(),
                    authentication,
                ))
                .route_layer(middleware::from_fn_with_state(
                    refresh_limiter(),
                    rate_limit,
                )),
        )
        .route(
            "/logout",
            post(logout).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/auth/profile/deactivate",
            post(deactivate).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/auth/profile/{business_profile_uuid}/activate",
            post(activate).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
}
