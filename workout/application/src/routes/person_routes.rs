use crate::authentication::authentication_middleware::authentication;
use crate::http::account_deletion_controller::{cancel_account_deletion, request_account_deletion};
use crate::http::image_controller::{person_image_delete, person_image_upload};
use crate::http::person_address_controller::{
    add_person_address, delete_person_address, update_person_address,
};
use crate::http::person_controller::{
    get_me, get_me_by_uuid, get_my_friend, search_persons, update_person,
};
use crate::http::person_info_controller::update_person_info;
use crate::AppState;
use axum::routing::{delete, get, post, put};
use axum::{middleware, Router};

/// Build person-related routes (profile, search, addresses, info)
pub fn person_routes(state: AppState) -> Router<AppState> {
    Router::new()
        .route(
            "/id/{id}",
            get(crate::http::person_controller::get_person_by_id).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/uuid/{uuid}",
            get(crate::http::person_controller::get_person_by_uuid).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/id/{id}/mentionable-friends",
            get(crate::http::person_controller::search_mentionable_friends).route_layer(
                middleware::from_fn_with_state(state.clone(), authentication),
            ),
        )
        .route(
            "/me",
            get(get_me).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/{uuid}",
            get(get_me_by_uuid).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me",
            put(update_person).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/friend/{friend_id}",
            get(get_my_friend).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/search",
            get(search_persons).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/upload/{image_type}",
            get(person_image_upload).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/delete/{image_type}",
            delete(person_image_delete).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/address",
            post(add_person_address).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
		.route(
			"/me/address/{person_address_id}",
            put(update_person_address).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
		)
		.route(
			"/me/address/uuid/{uuid}",
			delete(crate::http::person_address_controller::delete_person_address_by_uuid).route_layer(
				middleware::from_fn_with_state(state.clone(), authentication),
			),
		)
        .route(
            "/me/address/{person_address_id}",
            delete(delete_person_address).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/info/{person_info_id}",
            put(update_person_info).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/account/delete",
            post(request_account_deletion).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
        .route(
            "/me/account/cancel-deletion",
            post(cancel_account_deletion).route_layer(middleware::from_fn_with_state(
                state.clone(),
                authentication,
            )),
        )
}
