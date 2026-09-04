use crate::authentication::authentication_middleware::authentication;
use crate::authentication::rate_limit::{chat_limiter, rate_limit};
use crate::{http, AppState};
use axum::routing::{get, post, put};
use axum::{middleware, Router};

pub fn chat_routes(state: AppState) -> Router<AppState> {
    let auth = || middleware::from_fn_with_state(state.clone(), authentication);
    let throttle = || middleware::from_fn_with_state(chat_limiter(), rate_limit);

    Router::new()
        .route(
            "/conversations",
            get(http::chat_controller::list_conversations).route_layer(auth()),
        )
        .route(
            "/presence",
            get(http::chat_controller::presence).route_layer(auth()),
        )
        .route(
            "/conversations/direct",
            post(http::chat_controller::create_direct)
                .route_layer(auth())
                .route_layer(throttle()),
        )
        .route(
            "/conversations/business-team",
            post(http::chat_controller::create_business_team_group)
                .route_layer(auth())
                .route_layer(throttle()),
        )
        .route(
            "/conversations/business-direct",
            post(http::chat_controller::create_business_direct)
                .route_layer(auth())
                .route_layer(throttle()),
        )
        .route(
            "/conversations/{conversation_uuid}/messages",
            get(http::chat_controller::list_messages)
                .post(http::chat_controller::send_message)
                .route_layer(auth())
                .route_layer(throttle()),
        )
        .route(
            "/conversations/{conversation_uuid}/read",
            put(http::chat_controller::mark_read).route_layer(auth()),
        )
}
