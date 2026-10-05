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
        let mode = std::env::var("PUSH_PROVIDER_MODE").unwrap_or_else(|_| "firebase".to_string());
        let mode = match mode.as_str() {
            "fake" => {
                let endpoint = std::env::var("FCM_TEST_PROVIDER_URL")
                    .ok()
                    .filter(|value| !value.is_empty());
                PushProviderMode::Fake { endpoint }
            }
            "firebase" => {
                let raw = std::env::var("FCM_SERVICE_ACCOUNT_JSON").map_err(|_| {
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
    use super::{FcmMessage, FcmNotification, FcmRequest, PushProvider, PushSendError};
    use domain::in_app_notification::InAppNotification;

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
}
