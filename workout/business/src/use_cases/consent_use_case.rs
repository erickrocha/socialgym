use crate::commons::legal_documents;
use crate::domain::business_error::BusinessError;
use crate::gateway::consent_gateway::ConsentGateway;
use chrono::Utc;
use entity::consent_entity;
use sea_orm::DbConn;

pub struct ConsentUseCase;

impl ConsentUseCase {
    pub async fn accept(
        db: &DbConn,
        person_id: i32,
        document: &str,
        version: &str,
        ip: &str,
    ) -> Result<consent_entity::Model, BusinessError> {
        if !legal_documents::is_current(document, version) {
            return Err(BusinessError::validation(
                "Unknown or outdated legal document version",
            ));
        }
        if ConsentGateway::has_active(db, person_id, document, version)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?
        {
            return Err(BusinessError::conflict("Consent is already active"));
        }
        ConsentGateway::accept(db, person_id, document, version, ip)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }

    pub async fn list(
        db: &DbConn,
        person_id: i32,
    ) -> Result<Vec<consent_entity::Model>, BusinessError> {
        ConsentGateway::list(db, person_id)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))
    }

    pub async fn revoke(db: &DbConn, person_id: i32, document: &str) -> Result<(), BusinessError> {
        if legal_documents::current_version(document).is_none() {
            return Err(BusinessError::validation("Unknown legal document"));
        }
        let affected = ConsentGateway::revoke_active(db, person_id, document, Utc::now())
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?;
        if affected == 0 {
            return Err(BusinessError::not_found("No active consent found"));
        }
        Ok(())
    }

    pub async fn require_current(
        db: &DbConn,
        person_id: i32,
        document: &str,
    ) -> Result<(), BusinessError> {
        let version = legal_documents::current_version(document)
            .ok_or_else(|| BusinessError::validation("Unknown legal document"))?;
        if ConsentGateway::has_active(db, person_id, document, &version)
            .await
            .map_err(|e| BusinessError::infrastructure(e.to_string()))?
        {
            Ok(())
        } else {
            Err(BusinessError::forbidden("Required consent is not active"))
        }
    }
}
