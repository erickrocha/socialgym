use domain::in_app_notification::InAppNotification;
use jsonwebtoken::{Algorithm, EncodingKey, Header, encode};
use mongodb::Database;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
use tokio::sync::Mutex;

const FCM_SCOPE: &str = "https://www.googleapis.com/auth/firebase.messaging";

#[derive(Clone)]
pub struct PushProvider {
    client: reqwest::Client,
    mode: PushProviderMode,
    cached_access_token: Arc<Mutex<Option<CachedAccessToken>>>,
}

#[derive(Clone)]
enum PushProviderMode {
    Fake { endpoint: Option<String> },
    Firebase { account: FirebaseServiceAccount },
}

#[derive(Clone)]
struct CachedAccessToken {
    value: String,
    expires_at: Instant,
}

#[derive(Debug, Deserialize, Clone)]
struct FirebaseServiceAccount {
    project_id: String,
    client_email: String,
    private_key: String,
    token_uri: String,
}

#[derive(Serialize)]
struct ServiceAccountClaims<'a> {
    iss: &'a str,
    scope: &'static str,
    aud: &'a str,
    iat: u64,
    exp: u64,
}

#[derive(Deserialize)]
struct OAuthTokenResponse {
    access_token: String,
    expires_in: u64,
}

#[derive(Serialize)]
struct FcmRequest<'a> {
    message: FcmMessage<'a>,
}

#[derive(Serialize)]
struct FcmMessage<'a> {
    token: &'a str,
    notification: FcmNotification,
    data: std::collections::BTreeMap<&'static str, String>,
}

#[derive(Serialize)]
struct FcmNotification {
    title: &'static str,
    body: &'static str,
}

#[derive(Debug, PartialEq, Eq)]
pub enum PushSendError {
    InvalidToken,
    Transient(String),
    Configuration(String),
}

impl PushProvider {
    pub fn from_environment() -> Result<Self, PushSendError> {
        Self::from_settings(
            &std::env::var("PUSH_PROVIDER_MODE").unwrap_or_else(|_| "firebase".to_string()),
            std::env::var("FCM_TEST_PROVIDER_URL").ok(),
            std::env::var("FCM_SERVICE_ACCOUNT_JSON").ok(),
        )
    }

    fn from_settings(
        mode: &str,
        fake_endpoint: Option<String>,
        service_account_json: Option<String>,
    ) -> Result<Self, PushSendError> {
        let mode = match mode {
            "fake" => PushProviderMode::Fake {
                endpoint: fake_endpoint.filter(|value| !value.is_empty()),
            },
            "firebase" => {
                let raw = service_account_json.ok_or_else(|| {
                    PushSendError::Configuration(
                        "FCM_SERVICE_ACCOUNT_JSON is not configured".to_string(),
                    )
                })?;
                let account: FirebaseServiceAccount = serde_json::from_str(&raw).map_err(|_| {
                    PushSendError::Configuration("FCM service account JSON is invalid".to_string())
                })?;
                if account.project_id.is_empty()
                    || account.client_email.is_empty()
                    || account.private_key.is_empty()
                    || account.token_uri.is_empty()
                {
                    return Err(PushSendError::Configuration(
                        "FCM service account is incomplete".to_string(),
                    ));
                }
                PushProviderMode::Firebase { account }
            }
            _ => {
                return Err(PushSendError::Configuration(
                    "PUSH_PROVIDER_MODE must be firebase or fake".to_string(),
                ));
            }
        };

        let client = reqwest::Client::builder()
            .timeout(Duration::from_secs(10))
            .build()
            .map_err(|_| {
                PushSendError::Configuration("FCM HTTP client configuration failed".to_string())
            })?;
        Ok(Self {
            client,
            mode,
            cached_access_token: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn send(
        &self,
        db: &Database,
        registration_token: &str,
        notification: &InAppNotification,
    ) -> Result<(), PushSendError> {
        let request = FcmRequest {
            message: FcmMessage {
                token: registration_token,
                notification: FcmNotification {
                    title: "Social Gym",
                    body: "You have a new notification.",
                },
                data: [
                    ("notificationUuid", notification.uuid.clone()),
                    (
                        "targetType",
                        notification
                            .target_type
                            .clone()
                            .unwrap_or_else(|| "notifications".to_string()),
                    ),
                    (
                        "targetUuid",
                        notification.target_uuid.clone().unwrap_or_default(),
                    ),
                ]
                .into_iter()
                .collect(),
            },
        };

        if let PushProviderMode::Fake { endpoint: None } = &self.mode {
            db.collection::<mongodb::bson::Document>("push_provider_test_deliveries")
                .insert_one(doc! {
                    "notificationUuid": &notification.uuid,
                    "targetType": notification.target_type.as_deref().unwrap_or("notifications"),
                    "targetUuid": notification.target_uuid.as_deref().unwrap_or_default(),
                    "title": "Social Gym",
                    "body": "You have a new notification.",
                })
                .await
                .map_err(|error| {
                    PushSendError::Transient(format!("fake provider record failed: {error}"))
                })?;
            return match std::env::var("FAKE_PUSH_PROVIDER_STATUS").as_deref() {
                Ok("429") | Ok("500") | Ok("timeout") => Err(PushSendError::Transient(
                    "fake provider simulated a transient failure".to_string(),
                )),
                Ok("invalid-token") => Err(PushSendError::InvalidToken),
                Ok("401") | Ok("403") | Ok("configuration") => Err(PushSendError::Configuration(
                    "fake provider simulated a configuration failure".to_string(),
                )),
                _ => Ok(()),
            };
        }

        let response = match &self.mode {
            PushProviderMode::Fake { endpoint: None } => {
                return Err(PushSendError::Configuration(
                    "fake provider endpoint is unavailable".to_string(),
                ));
            }
            PushProviderMode::Fake {
                endpoint: Some(endpoint),
            } => self.client.post(endpoint),
            PushProviderMode::Firebase { account } => {
                let token = self.access_token(account).await?;
                self.client
                    .post(format!(
                        "https://fcm.googleapis.com/v1/projects/{}/messages:send",
                        account.project_id
                    ))
                    .bearer_auth(token)
            }
        }
        .json(&request)
        .send()
        .await
        .map_err(|error| PushSendError::Transient(format!("FCM request failed: {error}")))?;

        Self::classify_provider_response(response).await
    }

    async fn access_token(
        &self,
        account: &FirebaseServiceAccount,
    ) -> Result<String, PushSendError> {
        let mut cache = self.cached_access_token.lock().await;
        if let Some(token) = cache.as_ref()
            && token.expires_at > Instant::now() + Duration::from_secs(60)
        {
            return Ok(token.value.clone());
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_err(|_| PushSendError::Configuration("system clock is invalid".to_string()))?
            .as_secs();
        let claims = ServiceAccountClaims {
            iss: &account.client_email,
            scope: FCM_SCOPE,
            aud: &account.token_uri,
            iat: now,
            exp: now + 3600,
        };
        let key = EncodingKey::from_rsa_pem(account.private_key.as_bytes())
            .map_err(|_| PushSendError::Configuration("FCM private key is invalid".to_string()))?;
        let assertion = encode(&Header::new(Algorithm::RS256), &claims, &key).map_err(|_| {
            PushSendError::Configuration("FCM service assertion could not be signed".to_string())
        })?;
        let response = self
            .client
            .post(&account.token_uri)
            .form(&[
                ("grant_type", "urn:ietf:params:oauth:grant-type:jwt-bearer"),
                ("assertion", assertion.as_str()),
            ])
            .send()
            .await
            .map_err(|error| {
                PushSendError::Transient(format!("FCM OAuth request failed: {error}"))
            })?;

        if !response.status().is_success() {
            return Err(Self::classify_status(
                response.status(),
                "FCM OAuth request failed",
            ));
        }
        let token: OAuthTokenResponse = response.json().await.map_err(|error| {
            PushSendError::Configuration(format!("FCM OAuth response is invalid: {error}"))
        })?;
        let expires_in = token.expires_in.max(60);
        let cached = CachedAccessToken {
            value: token.access_token.clone(),
            expires_at: Instant::now() + Duration::from_secs(expires_in),
        };
        *cache = Some(cached);
        Ok(token.access_token)
    }

    async fn classify_provider_response(response: reqwest::Response) -> Result<(), PushSendError> {
        let status = response.status();
        if status.is_success() {
            return Ok(());
        }
        let body = response.text().await.unwrap_or_default();
        Self::classify_provider_result(status, &body)
    }

    fn classify_provider_result(
        status: reqwest::StatusCode,
        body: &str,
    ) -> Result<(), PushSendError> {
        if body.contains("UNREGISTERED") || body.contains("registration-token-not-registered") {
            return Err(PushSendError::InvalidToken);
        }
        Err(Self::classify_status(status, &body))
    }

    fn classify_status(status: reqwest::StatusCode, body: &str) -> PushSendError {
        if status == reqwest::StatusCode::TOO_MANY_REQUESTS || status.is_server_error() {
            PushSendError::Transient(format!("provider returned {status}: {body}"))
        } else if status == reqwest::StatusCode::UNAUTHORIZED
            || status == reqwest::StatusCode::FORBIDDEN
        {
            PushSendError::Configuration(format!("provider authorization failed with {status}"))
        } else {
            PushSendError::Configuration(format!("provider returned {status}: {body}"))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{FcmMessage, FcmNotification, FcmRequest, PushProvider, PushProviderMode, PushSendError};
    use axum::extract::Path;
    use axum::http::StatusCode;
    use axum::routing::post;
    use axum::Router;
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
            PushProvider::classify_provider_result(
                reqwest::StatusCode::BAD_REQUEST,
                "invalid payload"
            ),
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
                            200 => (StatusCode::OK, r#"{"access_token":"stub-token","expires_in":3600}"#.to_string()),
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
            "n1".into(), "Comment".into(), "r".into(), "a".into(), "Actor".into(),
            "post-1".into(), None, "text".into(),
        )
    }

    async fn unreachable_database() -> mongodb::Database {
        mongodb::Client::with_uri_str("mongodb://127.0.0.1:1/?serverSelectionTimeoutMS=150&connectTimeoutMS=150")
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
        assert!(config(PushProvider::from_settings("smoke-signal", None, None)).contains("firebase or fake"));
        assert!(config(PushProvider::from_settings("firebase", None, None)).contains("not configured"));
        assert!(config(PushProvider::from_settings("firebase", None, Some("{".into()))).contains("invalid"));
        let incomplete = r#"{"project_id":"p","client_email":"","private_key":"k","token_uri":"u"}"#;
        assert!(config(PushProvider::from_settings("firebase", None, Some(incomplete.into()))).contains("incomplete"));
        assert!(PushProvider::from_settings("fake", Some(String::new()), None).is_ok());
        assert!(PushProvider::from_settings("fake", Some("http://x".into()), None).is_ok());
    }

    #[tokio::test]
    async fn fake_endpoint_maps_provider_responses_to_send_outcomes() {
        let (base, _) = stub_server().await;
        let db = unreachable_database().await;
        let send = |code: u16| {
            let provider = PushProvider::from_settings("fake", Some(format!("{base}/fcm/{code}")), None).unwrap();
            let db = db.clone();
            async move { provider.send(&db, "device-token", &notification()).await }
        };

        assert_eq!(send(200).await, Ok(()));
        assert_eq!(send(404).await, Err(PushSendError::InvalidToken));
        assert!(matches!(send(429).await, Err(PushSendError::Transient(_))));
        assert!(matches!(send(503).await, Err(PushSendError::Transient(_))));
        assert!(matches!(send(401).await, Err(PushSendError::Configuration(m)) if m.contains("authorization")));
        assert!(matches!(send(400).await, Err(PushSendError::Configuration(_))));

        // A fake provider without an endpoint records to the database; an outage is transient.
        let recording = PushProvider::from_settings("fake", None, None).unwrap();
        assert!(matches!(
            recording.send(&db, "device-token", &notification()).await,
            Err(PushSendError::Transient(m)) if m.contains("record failed")
        ));
        // Nothing listens on the port: the transport error is transient.
        let dead = PushProvider::from_settings("fake", Some("http://127.0.0.1:1/".into()), None).unwrap();
        assert!(matches!(dead.send(&db, "t", &notification()).await, Err(PushSendError::Transient(_))));
    }

    #[tokio::test]
    async fn firebase_access_token_is_fetched_cached_and_failures_are_classified() {
        let (base, token_calls) = stub_server().await;
        let ok = firebase(&format!("{base}/token/200"), TEST_KEY);
        let PushProviderMode::Firebase { account } = &ok.mode else { unreachable!() };
        assert_eq!(ok.access_token(account).await.unwrap(), "stub-token");
        assert_eq!(ok.access_token(account).await.unwrap(), "stub-token");
        assert_eq!(token_calls.load(Ordering::SeqCst), 1, "second call is served from the cache");

        let expect = |uri: String, key: &'static str| {
            let provider = firebase(&uri, key);
            async move {
                let PushProviderMode::Firebase { account } = &provider.mode else { unreachable!() };
                provider.access_token(account).await.unwrap_err()
            }
        };
        assert!(matches!(expect(format!("{base}/token/401"), TEST_KEY).await, PushSendError::Configuration(_)));
        assert!(matches!(expect(format!("{base}/token/500"), TEST_KEY).await, PushSendError::Transient(_)));
        assert!(matches!(expect(format!("{base}/token/201"), TEST_KEY).await, PushSendError::Configuration(m) if m.contains("invalid")));
        assert!(matches!(expect("http://127.0.0.1:1/".into(), TEST_KEY).await, PushSendError::Transient(_)));
        assert!(matches!(expect(format!("{base}/token/200"), "not a pem").await, PushSendError::Configuration(m) if m.contains("private key")));

        // send() stops at the failed token exchange and never reaches FCM.
        let denied = firebase(&format!("{base}/token/401"), TEST_KEY);
        let db = unreachable_database().await;
        assert!(matches!(denied.send(&db, "t", &notification()).await, Err(PushSendError::Configuration(_))));
    }
}
