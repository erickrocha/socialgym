//! `GrpcAuthLayer` behaviors that need no database: the public-method allow-list with its per-IP limit,
//! the internal-secret methods, and the consent exemption list.
use super::{GrpcAuthLayer, is_consent_exempt};
use business::commons::rate_limit::RateLimiter;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use std::time::Duration;
use tonic::body::Body as TonicBody;
use tower::{Layer, ServiceExt};

const PUBLIC: &str = "/grpc.auth.AuthService/Login";
const INTERNAL: &str = "/grpc.settings.SettingsService/GetPushPreferenceByOwnerUuid";

/// The `grpc-status` the layer answers, or `None` when it let the call through to the service.
async fn status(layer: &GrpcAuthLayer, path: &str, headers: &[(&str, &str)]) -> Option<String> {
    let inner = tower::service_fn(|_: hyper::Request<TonicBody>| async {
        Ok::<_, std::convert::Infallible>(hyper::Response::new(TonicBody::empty()))
    });
    let mut builder = hyper::Request::builder().uri(path);
    for (name, value) in headers {
        builder = builder.header(*name, *value);
    }
    let response = layer
        .layer(inner)
        .oneshot(builder.body(TonicBody::empty()).unwrap())
        .await
        .unwrap();
    response
        .headers()
        .get("grpc-status")
        .and_then(|v| v.to_str().ok())
        .map(str::to_string)
}

fn layer() -> GrpcAuthLayer {
    GrpcAuthLayer::new(Arc::new(DatabaseConnection::default()))
}

#[tokio::test]
async fn a_public_method_needs_no_token_but_is_limited_per_address() {
    let layer = layer().public_method(PUBLIC, RateLimiter::new(2, Duration::from_secs(60)));
    let from = |ip: &'static str| [("x-real-ip", ip)];
    assert_eq!(
        status(&layer, PUBLIC, &from("192.0.2.1")).await,
        None,
        "first call passes without a token"
    );
    assert_eq!(status(&layer, PUBLIC, &from("192.0.2.1")).await, None);
    assert_eq!(
        status(&layer, PUBLIC, &from("192.0.2.1")).await.as_deref(),
        Some("8"),
        "RESOURCE_EXHAUSTED past the limit"
    );
    assert_eq!(
        status(&layer, PUBLIC, &from("192.0.2.2")).await,
        None,
        "another address is not affected"
    );
}

#[tokio::test]
async fn every_other_method_still_needs_a_token() {
    let layer = layer().public_method(PUBLIC, RateLimiter::new(100, Duration::from_secs(60)));
    assert_eq!(
        status(&layer, "/grpc.workout.WorkoutService/GetWorkout", &[])
            .await
            .as_deref(),
        Some("16")
    );
    assert_eq!(
        status(&layer, "/grpc.auth.AuthService/Signup", &[])
            .await
            .as_deref(),
        Some("16"),
        "only listed methods are public"
    );
}

#[tokio::test]
async fn an_internal_method_opens_only_to_the_secret() {
    unsafe { std::env::set_var("INTERNAL_SERVICE_SECRET", "layer-test-secret") };
    let layer = layer();
    assert_eq!(
        status(
            &layer,
            INTERNAL,
            &[("x-internal-secret", "layer-test-secret")]
        )
        .await,
        None
    );
    assert_eq!(
        status(
            &layer,
            INTERNAL,
            &[("x-internal-secret", "layer-test-secreT")]
        )
        .await
        .as_deref(),
        Some("16")
    );
    assert_eq!(
        status(&layer, INTERNAL, &[("authorization", "Bearer anything")])
            .await
            .as_deref(),
        Some("16"),
        "a user token never opens it"
    );
}

#[test]
fn the_consent_check_and_the_recovery_services_stay_reachable_without_consent() {
    assert!(is_consent_exempt(
        "/grpc.person.PersonService/HasActiveConsent"
    ));
    assert!(is_consent_exempt(
        "/grpc.consent.ConsentService/AcceptConsent"
    ));
    assert!(is_consent_exempt(
        "/grpc.account.AccountService/CancelAccountDeletion"
    ));
    assert!(!is_consent_exempt("/grpc.person.PersonService/GetMe"));
    assert!(
        !is_consent_exempt("/grpc.person.PersonService/HasActiveConsentX"),
        "an exact method is not a prefix"
    );
    assert!(!is_consent_exempt("/grpc.consentless.Other/Call"));
}
