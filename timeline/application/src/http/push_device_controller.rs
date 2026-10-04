use crate::AppState;
use crate::commons::exception_response::{ExceptionResponse, HttpResponse};
use crate::commons::i18n::{ErrorKey, Locale};
use crate::http::json::push_device_json::RegisterPushDeviceJson;
use axum::Json;
use axum::extract::{Extension, Path, State};
use axum::http::StatusCode;
use business::gateway::push_device_gateway::PushDeviceGateway;
use domain::user::User;
use uuid::Uuid;

#[utoipa::path(
    put,
    path = "/timeline/api/push-devices/{device_uuid}",
    params(("device_uuid" = String, Path, description = "Stable installation UUID")),
    request_body = RegisterPushDeviceJson,
    responses(
        (status = 204, description = "Push device registered; registration token is never returned"),
        (status = 400, description = "Invalid device UUID, platform, or token"),
        (status = 401, description = "Unauthorized"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_key" = []))
)]
pub async fn register_push_device(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(device_uuid): Path<String>,
    Json(payload): Json<RegisterPushDeviceJson>,
) -> HttpResponse<StatusCode> {
    if Uuid::parse_str(&device_uuid).is_err()
        || !matches!(payload.platform.as_str(), "android" | "ios")
        || payload.registration_token.trim().is_empty()
        || payload.registration_token.len() > 4096
    {
        return Err(ExceptionResponse::bad_request(
            Locale::En,
            ErrorKey::Unknown,
        ));
    }

    PushDeviceGateway::register(
        &state.database,
        &device_uuid,
        &current_user.person_uuid,
        &payload.platform,
        &payload.registration_token,
    )
    .await
    .map_err(|_| ExceptionResponse::internal_server_error(Locale::En, ErrorKey::Unknown))?;

    Ok(StatusCode::NO_CONTENT)
}

#[utoipa::path(
    delete,
    path = "/timeline/api/push-devices/{device_uuid}",
    params(("device_uuid" = String, Path, description = "Stable installation UUID")),
    responses(
        (status = 204, description = "Push device removed"),
        (status = 401, description = "Unauthorized"),
        (status = 404, description = "Device not registered for this owner"),
        (status = 500, description = "Internal server error"),
    ),
    security(("api_key" = []))
)]
pub async fn remove_push_device(
    State(state): State<AppState>,
    Extension(current_user): Extension<User>,
    Path(device_uuid): Path<String>,
) -> HttpResponse<StatusCode> {
    let removed =
        PushDeviceGateway::remove_owned(&state.database, &device_uuid, &current_user.person_uuid)
            .await
            .map_err(|_| ExceptionResponse::internal_server_error(Locale::En, ErrorKey::Unknown))?;

    if !removed {
        return Err(ExceptionResponse::NotFound(Locale::En, ErrorKey::Unknown));
    }
    Ok(StatusCode::NO_CONTENT)
}

#[cfg(test)]
mod tests {
    use crate::infrastructure::chat_hub::ChatHub;
    use crate::AppState;
    use axum::body::{Body, to_bytes};
    use axum::http::{Request, StatusCode};
    use axum::Router;
    use business::gateway::push_device_gateway::PushDeviceGateway;
    use domain::push_device::PushDevice;
    use crate::routes::push_device_routes::push_device_routes;
    use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
    use mongodb::{Client, bson::doc};
    use serde_json::json;
    use std::sync::Arc;
    use std::env;
    use tower::ServiceExt;

    #[tokio::test]
    #[ignore = "requires a dedicated TEST_MONGO_URL MongoDB database"]
    async fn c006_push_device_http_contract_returns_no_token_and_uses_authenticated_owner() {
        let mongo_url = std::env::var("TEST_MONGO_URL")
            .expect("TEST_MONGO_URL must point to the disposable timeline_test database");
        let mongo_client = Client::with_uri_str(mongo_url).await.unwrap();
        let database = mongo_client.database("timeline_test");
        let device_uuid = "d2a7c810-2a10-4cab-8a9e-3df935d20c01";
        let owner_uuid = "00000000-0000-0000-0000-000000000061";
        database
            .collection::<PushDevice>("push_devices")
            .delete_many(doc! { "deviceUuid": device_uuid })
            .await
            .unwrap();

        let access_token_secret = "c005-test-secret";
        for (name, value) in [
            ("ACCESS_TOKEN_SECRET", access_token_secret),
            ("GRPC_PROTOCOL", "https"),
            ("GRPC_HOST", "127.0.0.1"),
            ("GRPC_PORT", "50051"),
            ("GRPC_USE_TLS", "true"),
            ("GRPC_CERT_PATH", "/tmp/socialgym-c006-ca.crt"),
            ("GRPC_DOMAIN_NAME", "localhost"),
        ] {
            unsafe { env::set_var(name, value) };
        }
        let now = chrono::Utc::now().timestamp();
        let claims = json!({
            "sub": "c006-sender@example.test",
            "exp": now + 3600,
            "iat": now,
            "jti": "c006-tc009-http-jwt",
            "uuid": "10000000-0000-0000-0000-000000000061",
            "name": "Sender Person",
            "person_id": 1,
            "person_uuid": owner_uuid,
            "person_object_key": "default",
            "active_business_profile_id": null,
            "active_business_profile_uuid": null
        });
        let token = encode(
            &Header::new(Algorithm::HS512),
            &claims,
            &EncodingKey::from_secret(access_token_secret.as_bytes()),
        )
        .unwrap();
        let authorization = format!("Bearer {token}");
        let state = AppState {
            database: Arc::new(database.clone()),
            chat_hub: ChatHub::new(),
        };
        let app = Router::new()
            .nest(
                "/timeline/api/push-devices",
                push_device_routes(state.clone()),
            )
            .with_state(state);

        let valid_response = app
            .clone()
            .oneshot(
                Request::put(format!("/timeline/api/push-devices/{device_uuid}"))
                    .header("content-type", "application/json")
                    .header("authorization", &authorization)
                    .body(Body::from(
                        serde_json::json!({
                            "platform": "android",
                            "registrationToken": "c006-tc009-provider-token"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(valid_response.status(), StatusCode::NO_CONTENT);
        assert!(to_bytes(valid_response.into_body(), 1024).await.unwrap().is_empty());

        let registered = PushDeviceGateway::find_all_for_person(&database, owner_uuid)
            .await
            .unwrap();
        assert_eq!(registered.len(), 1);
        assert_eq!(registered[0].device_uuid, device_uuid);
        assert_eq!(registered[0].registration_token, "c006-tc009-provider-token");

        let invalid_response = app
            .oneshot(
                Request::put(format!("/timeline/api/push-devices/{device_uuid}"))
                    .header("content-type", "application/json")
                    .header("authorization", &authorization)
                    .body(Body::from(
                        serde_json::json!({
                            "platform": "web",
                            "registrationToken": "must-not-be-stored"
                        })
                        .to_string(),
                    ))
                    .unwrap(),
            )
            .await
            .unwrap();
        assert_eq!(invalid_response.status(), StatusCode::BAD_REQUEST);
        assert!(
            PushDeviceGateway::find_all_for_person(&database, owner_uuid)
                .await
                .unwrap()
                .iter()
                .all(|device| device.registration_token != "must-not-be-stored")
        );

        database
            .collection::<PushDevice>("push_devices")
            .delete_many(doc! { "deviceUuid": device_uuid })
            .await
            .unwrap();
        for name in [
            "ACCESS_TOKEN_SECRET",
            "GRPC_PROTOCOL",
            "GRPC_HOST",
            "GRPC_PORT",
            "GRPC_USE_TLS",
            "GRPC_CERT_PATH",
            "GRPC_DOMAIN_NAME",
        ] {
            unsafe { env::remove_var(name) };
        }
    }
}
