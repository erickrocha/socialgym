use crate::infrastructure::utils::{locale_of, localized_status};
use crate::proto::legal::legal_document_service_server::LegalDocumentService;
use crate::proto::legal::{
    GetLegalDocumentRequest, LegalDocument, ListLegalDocumentsRequest, ListLegalDocumentsResponse,
};
use business::commons::i18n::ErrorKey;
use business::use_cases::legal_document_use_case::{LegalDocument as Domain, LegalDocumentUseCase};
use tonic::{Code, Request, Response, Status};

/// The gRPC twin of the REST `/legal/documents` routes. Public: the authentication layer lets these two
/// methods through without a token, with a per-address limit.
pub struct GrpcLegalDocumentService;

fn response(document: Domain) -> LegalDocument {
    LegalDocument {
        document: document.document.to_string(),
        version: document.version,
        title: document.title.to_string(),
        content: document.content.to_string(),
    }
}

#[tonic::async_trait]
impl LegalDocumentService for GrpcLegalDocumentService {
    async fn list_legal_documents(&self, _: Request<ListLegalDocumentsRequest>) -> Result<Response<ListLegalDocumentsResponse>, Status> {
        Ok(Response::new(ListLegalDocumentsResponse {
            documents: LegalDocumentUseCase::list().into_iter().map(response).collect(),
        }))
    }

    async fn get_legal_document(&self, request: Request<GetLegalDocumentRequest>) -> Result<Response<LegalDocument>, Status> {
        let locale = locale_of(&request);
        LegalDocumentUseCase::get(&request.into_inner().document)
            .map(|document| Response::new(response(document)))
            .ok_or_else(|| localized_status(Code::NotFound, ErrorKey::InvalidParameterValue, locale))
    }
}
