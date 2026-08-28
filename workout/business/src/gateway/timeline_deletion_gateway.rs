use crate::domain::business_error::BusinessError;
use std::env;
use std::sync::OnceLock;

const TIMELINE_BASE_URL: &str = "TIMELINE_BASE_URL";
const INTERNAL_SERVICE_SECRET: &str = "INTERNAL_SERVICE_SECRET";

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(reqwest::Client::new)
}

pub struct TimelineDeletionGateway {}

impl TimelineDeletionGateway {
    pub async fn export_person_data(person_uuid: &str) -> Result<serde_json::Value, BusinessError> {
        let base_url = std::env::var(TIMELINE_BASE_URL)
            .unwrap_or_else(|_| "http://127.0.0.1:8091".to_string());
        let secret = std::env::var(INTERNAL_SERVICE_SECRET).map_err(|_| {
            BusinessError::infrastructure("INTERNAL_SERVICE_SECRET is not configured")
        })?;
        let url = format!(
            "{}/timeline/api/internal/persons/{}/export",
            base_url.trim_end_matches('/'),
            person_uuid
        );
        let response = reqwest::Client::new()
            .get(url)
            .header("x-internal-secret", secret)
            .send()
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Timeline export failed: {e}")))?;
        if !response.status().is_success() {
            return Err(BusinessError::infrastructure(format!(
                "Timeline export returned {}",
                response.status()
            )));
        }
        response
            .json()
            .await
            .map_err(|e| BusinessError::infrastructure(format!("Invalid timeline export: {e}")))
    }
    /// Calls `timeline`'s internal endpoint to cascade-delete every document
    /// belonging to `person_uuid` (posts, comments, reactions, evolution
    /// check-ins, workout-session mirrors, notifications). Idempotent — safe
    /// to retry on failure.
    pub async fn delete_person_data(person_uuid: &str) -> Result<(), BusinessError> {
        let base_url = env::var(TIMELINE_BASE_URL).expect("TIMELINE_BASE_URL must be set");
        let secret =
            env::var(INTERNAL_SERVICE_SECRET).expect("INTERNAL_SERVICE_SECRET must be set");
        let url = format!("{}/timeline/api/internal/persons/{}", base_url, person_uuid);

        let response = http_client()
            .delete(&url)
            .header("X-Internal-Secret", secret)
            .send()
            .await;

        let response = match response {
            Ok(response) => response,
            Err(error) => {
                log::error!(
                    "Error calling timeline account-deletion endpoint: {}",
                    error
                );
                return Err(BusinessError::infrastructure(
                    "Failed to delete timeline data for account".to_string(),
                ));
            }
        };

        if !response.status().is_success() {
            let status = response.status();
            let body = response.text().await.unwrap_or_default();
            log::error!(
                "Timeline account-deletion endpoint returned {}: {}",
                status,
                body
            );
            return Err(BusinessError::infrastructure(
                "Failed to delete timeline data for account".to_string(),
            ));
        }

        Ok(())
    }
}
