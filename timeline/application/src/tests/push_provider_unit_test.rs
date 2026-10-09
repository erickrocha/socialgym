use super::{
    FcmMessage, FcmNotification, FcmRequest, PushProvider, PushProviderMode, PushSendError,
};
use axum::Router;
use axum::extract::Path;
use axum::http::StatusCode;
use axum::routing::post;
use domain::in_app_notification::InAppNotification;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

#[test]
fn push_payload_contains_only_navigation_identifiers_and_generic_text() {
    let notification = InAppNotification::from_social_interaction(
        "notification-1".to_string(),
        "Comment".to_string(),
        "recipient-1".to_string(),
        "actor-1".to_string(),
        "Actor Name".to_string(),
        "post-1".to_string(),
        Some("comment-1".to_string()),
        "private comment text".to_string(),
    );
    let message = FcmMessage {
        token: "test-token",
        notification: FcmNotification {
            title: "Social Gym",
            body: "You have a new notification.",
        },
        data: [
            ("notificationUuid", notification.uuid),
            ("targetType", notification.target_type.unwrap()),
            ("targetUuid", notification.target_uuid.unwrap()),
        ]
        .into_iter()
        .collect(),
    };
    let json = serde_json::to_value(FcmRequest { message }).unwrap();
    let serialized = json.to_string();
    assert!(!serialized.contains("private comment text"));
    assert!(serialized.contains("notificationUuid"));
    assert!(serialized.contains("targetType"));
    assert!(serialized.contains("targetUuid"));
    assert!(serialized.contains("You have a new notification."));
}

#[test]
fn provider_statuses_distinguish_transient_invalid_token_and_configuration() {
    assert!(matches!(
        PushProvider::classify_provider_result(
            reqwest::StatusCode::TOO_MANY_REQUESTS,
            "rate limited"
        ),
        Err(PushSendError::Transient(_))
    ));
    assert!(matches!(
        PushProvider::classify_provider_result(
            reqwest::StatusCode::INTERNAL_SERVER_ERROR,
            "temporary failure"
        ),
        Err(PushSendError::Transient(_))
    ));
    assert_eq!(
        PushProvider::classify_provider_result(reqwest::StatusCode::NOT_FOUND, "UNREGISTERED"),
        Err(PushSendError::InvalidToken)
    );
    assert!(matches!(
        PushProvider::classify_provider_result(
            reqwest::StatusCode::UNAUTHORIZED,
            "invalid credentials"
        ),
        Err(PushSendError::Configuration(_))
    ));
    assert!(matches!(
        PushProvider::classify_provider_result(reqwest::StatusCode::BAD_REQUEST, "invalid payload"),
        Err(PushSendError::Configuration(_))
    ));
}

// Throwaway key generated for these tests only; it signs assertions for a local stub.
const TEST_KEY: &str = "-----BEGIN PRIVATE KEY-----\nMIIEvQIBADANBgkqhkiG9w0BAQEFAASCBKcwggSjAgEAAoIBAQDQgx7YxF13zkyV\npqgNfh4z1hzSSAA7qS5DDaE0F0GF9V0shs95fBJd68p3WcBmBBjWpxGVXPZ/k4lC\nK00s4Anl1eSBDHo/gOK90yBzul34Pcz4rFnsjoHPMKpZDfIODCpuSUKJtdgru7tQ\nRuUTDsmUAFjFjbqnfZuFy7LtdN9hJIWxi/C+Rolrr4w6hGEd0qjucEskzOa64rc3\nmHJV+fL5Pxs5rVJKbujFs65gY0ptpGMg/lJXQ1pW/1oAbb0gbqwTcCEGzs5vb8FT\n1onG8mNG3I6ijX6PZBapKfg4f2/iNQGxkwOCj/wpZIXClZ5mEMDoZUH8hjo+EEI+\nWahO3f/JAgMBAAECggEAFYtOvx/+9leLAzVujMJYtYtse++zknaHEKeSXfL/Mquo\n/mZJfqhqr2ijCVTtM709ocQuZWvvDjx8xKj7tuTJMQW6L/lNkk0rYGi1pC9/8RvV\nl2Ybfn00/WbuWvg9Z+Uq5y1LojX3SYGCt6czmRYAjvnAMl1UCWVJkff7m1xljkBL\ngC5JXQOTswciPhG0m7OHc63o9GFLJqtq+tlHU5VWMtXkjiuneXYCi5b83A5aYJ/W\nz1vqVq1sFrM6f27hXqZxG6EvvWLGVk3KzuBsRjg8m1pRGNmGXKoo9CKJU4ZllbWJ\nTdsqMQ/9Mc5yAJmACNwKHQKj6ijv9gZGhR7nBkXtOQKBgQDy6mhhNc8hgebkpyNk\nmn4GUGACd0rNZs6IlNEk8Yje/35S/VwIHrgrA8WJzFfaOo/2JniPQzCCNZPjuj8U\npT0JBjPf00Mv/npU+wrEb/a2Pt1IxeqohCDTEVrnTstdVgUAifGD/CZ2QoMyJkOH\ngz+3gCNeAbKxuacqmQtO7Hbf7QKBgQDbvlHB4O7+3dzLcglPBu0n37ysBQmXu4fv\n3l5/sqLgSfkPTr1jnAjaxP3CKz1hwjjfaIwC6NVJbdTk4w+SD7hPLW89iA8d7/Ye\nT1X7akhLmmIKq8lom9ldx/Rc18tH5/Sv6jK1EWW2wRtw3tIJfT/O02N7y5zhfnJ8\nQAcsv5OLzQKBgQDqaL11IUHSKdKvz4Hd8R+v5BviBU/Pymd/cWwpZFMaLh6u4wLO\nayZ5i9gx01jrpBuMAy/Pv5yt6hneZbm9qH6vmKayVOG/DKjsHJ0VVp4S27MaKLL7\nDxXF2DeKgoTIetu022iOXuKWuDR2TmpX+JWh9Css6iKQUoE4IBvDe3hzOQKBgAuM\nMNezblvOZO/8Cikw//18cHnJuftTS417bhPf21dWC9SWGrXFWWHFwGAXzZ35iuuS\nnuj4O10kfG+azxKp6NGgXZwqLkEwfwqQuMABkdMHReexp1/r8LxQ6bKHVJNuyRkF\ngayqQWIdO8furbL59gR0b2HuDUx8TJ/i7X2Yg7RxAoGAA/zt4Y3iB4TpgkvmPMBQ\nMzPVoZnFPlCBFknR+l2jj3pI4t1I1o/XHVlN67gWzBidU38gjWUAFDUXtZKfMXvh\nhaAkS6Cgh3VIxhvBwcClDk9o0hfgQ5bb1HCM1XCHJBepg97WWz/EMlQyC9IieQFK\nO2OMNhDVTyGzCRxeukNWB7w=\n-----END PRIVATE KEY-----"; // gitleaks:allow

/// Local stand-in for the OAuth token endpoint and the FCM send endpoint.
async fn stub_server() -> (String, Arc<AtomicUsize>) {
    let token_calls = Arc::new(AtomicUsize::new(0));
    let counter = token_calls.clone();
    let app = Router::new()
        .route(
            "/token/{code}",
            post(move |Path(code): Path<u16>| {
                counter.fetch_add(1, Ordering::SeqCst);
                async move {
                    match code {
                        200 => (
                            StatusCode::OK,
                            r#"{"access_token":"stub-token","expires_in":3600}"#.to_string(),
                        ),
                        201 => (StatusCode::OK, "not json".to_string()),
                        other => (StatusCode::from_u16(other).unwrap(), String::new()),
                    }
                }
            }),
        )
        .route(
            "/fcm/{code}",
            post(|Path(code): Path<u16>| async move {
                let body = if code == 404 { "UNREGISTERED" } else { "" };
                (StatusCode::from_u16(code).unwrap(), body)
            }),
        );
    let listener = tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
    let base = format!("http://{}", listener.local_addr().unwrap());
    tokio::spawn(async move { axum::serve(listener, app).await.unwrap() });
    (base, token_calls)
}

fn firebase(token_uri: &str, key: &str) -> PushProvider {
    let account = serde_json::json!({
        "project_id": "p", "client_email": "svc@p.iam", "private_key": key, "token_uri": token_uri
    })
    .to_string();
    PushProvider::from_settings("firebase", None, Some(account)).unwrap()
}

fn notification() -> InAppNotification {
    InAppNotification::from_social_interaction(
        "n1".into(),
        "Comment".into(),
        "r".into(),
        "a".into(),
        "Actor".into(),
        "post-1".into(),
        None,
        "text".into(),
    )
}

async fn unreachable_database() -> mongodb::Database {
    mongodb::Client::with_uri_str(
        "mongodb://127.0.0.1:1/?serverSelectionTimeoutMS=150&connectTimeoutMS=150",
    )
    .await
    .unwrap()
    .database("unreachable")
}

#[test]
fn settings_reject_unknown_mode_and_incomplete_firebase_accounts() {
    let config = |r: Result<PushProvider, PushSendError>| match r {
        Err(PushSendError::Configuration(m)) => m,
        _ => panic!("expected a configuration error"),
    };
    assert!(
        config(PushProvider::from_settings("smoke-signal", None, None))
            .contains("firebase or fake")
    );
    assert!(config(PushProvider::from_settings("firebase", None, None)).contains("not configured"));
    assert!(
        config(PushProvider::from_settings(
            "firebase",
            None,
            Some("{".into())
        ))
        .contains("invalid")
    );
    let incomplete = r#"{"project_id":"p","client_email":"","private_key":"k","token_uri":"u"}"#;
    assert!(
        config(PushProvider::from_settings(
            "firebase",
            None,
            Some(incomplete.into())
        ))
        .contains("incomplete")
    );
    assert!(PushProvider::from_settings("fake", Some(String::new()), None).is_ok());
    assert!(PushProvider::from_settings("fake", Some("http://x".into()), None).is_ok());
}

#[tokio::test]
async fn fake_endpoint_maps_provider_responses_to_send_outcomes() {
    let (base, _) = stub_server().await;
    let db = unreachable_database().await;
    let send = |code: u16| {
        let provider =
            PushProvider::from_settings("fake", Some(format!("{base}/fcm/{code}")), None).unwrap();
        let db = db.clone();
        async move { provider.send(&db, "device-token", &notification()).await }
    };

    assert_eq!(send(200).await, Ok(()));
    assert_eq!(send(404).await, Err(PushSendError::InvalidToken));
    assert!(matches!(send(429).await, Err(PushSendError::Transient(_))));
    assert!(matches!(send(503).await, Err(PushSendError::Transient(_))));
    assert!(
        matches!(send(401).await, Err(PushSendError::Configuration(m)) if m.contains("authorization"))
    );
    assert!(matches!(
        send(400).await,
        Err(PushSendError::Configuration(_))
    ));

    // A fake provider without an endpoint records to the database; an outage is transient.
    let recording = PushProvider::from_settings("fake", None, None).unwrap();
    assert!(matches!(
        recording.send(&db, "device-token", &notification()).await,
        Err(PushSendError::Transient(m)) if m.contains("record failed")
    ));
    // Nothing listens on the port: the transport error is transient.
    let dead =
        PushProvider::from_settings("fake", Some("http://127.0.0.1:1/".into()), None).unwrap();
    assert!(matches!(
        dead.send(&db, "t", &notification()).await,
        Err(PushSendError::Transient(_))
    ));
}

#[tokio::test]
async fn firebase_access_token_is_fetched_cached_and_failures_are_classified() {
    let (base, token_calls) = stub_server().await;
    let ok = firebase(&format!("{base}/token/200"), TEST_KEY);
    let PushProviderMode::Firebase { account } = &ok.mode else {
        unreachable!()
    };
    assert_eq!(ok.access_token(account).await.unwrap(), "stub-token");
    assert_eq!(ok.access_token(account).await.unwrap(), "stub-token");
    assert_eq!(
        token_calls.load(Ordering::SeqCst),
        1,
        "second call is served from the cache"
    );

    let expect = |uri: String, key: &'static str| {
        let provider = firebase(&uri, key);
        async move {
            let PushProviderMode::Firebase { account } = &provider.mode else {
                unreachable!()
            };
            provider.access_token(account).await.unwrap_err()
        }
    };
    assert!(matches!(
        expect(format!("{base}/token/401"), TEST_KEY).await,
        PushSendError::Configuration(_)
    ));
    assert!(matches!(
        expect(format!("{base}/token/500"), TEST_KEY).await,
        PushSendError::Transient(_)
    ));
    assert!(
        matches!(expect(format!("{base}/token/201"), TEST_KEY).await, PushSendError::Configuration(m) if m.contains("invalid"))
    );
    assert!(matches!(
        expect("http://127.0.0.1:1/".into(), TEST_KEY).await,
        PushSendError::Transient(_)
    ));
    assert!(
        matches!(expect(format!("{base}/token/200"), "not a pem").await, PushSendError::Configuration(m) if m.contains("private key"))
    );

    // send() stops at the failed token exchange and never reaches FCM.
    let denied = firebase(&format!("{base}/token/401"), TEST_KEY);
    let db = unreachable_database().await;
    assert!(matches!(
        denied.send(&db, "t", &notification()).await,
        Err(PushSendError::Configuration(_))
    ));
}
