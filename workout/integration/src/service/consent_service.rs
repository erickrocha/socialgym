use crate::infrastructure::mapper::{ConsentMapper, Mapper};
use crate::infrastructure::utils::{locale_of, localized_business_status, require_actor};
use crate::proto::consent::consent_service_server::ConsentService;
use crate::proto::consent::{
    AcceptConsentRequest, Consent, ListConsentsRequest, ListConsentsResponse, ListPendingConsentsRequest,
    ListPendingConsentsResponse, PendingConsent, RevokeConsentRequest, RevokeConsentResponse,
};
use business::commons::i18n::ErrorKey;
use business::domain::business_error::BusinessErrorKind;
use business::use_cases::consent_use_case::ConsentUseCase;
use business::use_cases::sign_up_use_case::acceptance_ip;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

/// The gRPC twin of the REST consent routes; the rules live in `ConsentUseCase`.
pub struct GrpcConsentService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcConsentService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

fn header<T>(request: &Request<T>, name: &str) -> Option<String> {
    request.metadata().get(name).and_then(|value| value.to_str().ok()).map(str::to_string)
}

#[tonic::async_trait]
impl ConsentService for GrpcConsentService {
    async fn list_consents(&self, request: Request<ListConsentsRequest>) -> Result<Response<ListConsentsResponse>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        let rows = ConsentUseCase::list(&self.conn, user.person_id)
            .await
            .map_err(|e| localized_business_status(e, ErrorKey::ConsentOperationFailed, locale))?;
        Ok(Response::new(ListConsentsResponse { consents: ConsentMapper::response_vec(rows) }))
    }

    async fn list_pending_consents(&self, request: Request<ListPendingConsentsRequest>) -> Result<Response<ListPendingConsentsResponse>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        let rows = ConsentUseCase::pending(&self.conn, user.person_id)
            .await
            .map_err(|e| localized_business_status(e, ErrorKey::ConsentOperationFailed, locale))?;
        let pending = rows
            .into_iter()
            .map(|row| PendingConsent { document: row.document, current_version: row.current_version, accepted_version: row.accepted_version })
            .collect();
        Ok(Response::new(ListPendingConsentsResponse { pending }))
    }

    async fn accept_consent(&self, request: Request<AcceptConsentRequest>) -> Result<Response<Consent>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        let ip = acceptance_ip(header(&request, "x-real-ip").as_deref(), header(&request, "x-forwarded-for").as_deref());
        let payload = request.into_inner();
        ConsentUseCase::accept_confirmed(&self.conn, user.person_id, &payload.document, &payload.version, payload.accepted, &ip)
            .await
            .map(|row| Response::new(ConsentMapper::response(row)))
            .map_err(|e| {
                let key = if e.kind == BusinessErrorKind::Validation { ErrorKey::ConsentRequired } else { ErrorKey::ConsentOperationFailed };
                localized_business_status(e, key, locale)
            })
    }

    async fn revoke_consent(&self, request: Request<RevokeConsentRequest>) -> Result<Response<RevokeConsentResponse>, Status> {
        let (locale, user) = (locale_of(&request), require_actor(&request)?);
        ConsentUseCase::revoke_for_user(&self.conn, &user, &request.into_inner().document)
            .await
            .map_err(|e| localized_business_status(e, ErrorKey::ConsentOperationFailed, locale))?;
        Ok(Response::new(RevokeConsentResponse {}))
    }
}
