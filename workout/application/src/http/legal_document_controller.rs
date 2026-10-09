use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use business::use_cases::legal_document_use_case::{LegalDocument, LegalDocumentUseCase};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct LegalDocumentJson {
    pub document: &'static str,
    pub version: String,
    pub title: &'static str,
    pub content: &'static str,
}

impl From<LegalDocument> for LegalDocumentJson {
    fn from(value: LegalDocument) -> Self {
        Self {
            document: value.document,
            version: value.version,
            title: value.title,
            content: value.content,
        }
    }
}

pub async fn list() -> Json<Vec<LegalDocumentJson>> {
    Json(
        LegalDocumentUseCase::list()
            .into_iter()
            .map(LegalDocumentJson::from)
            .collect(),
    )
}

pub async fn get(Path(name): Path<String>) -> Result<Json<LegalDocumentJson>, StatusCode> {
    LegalDocumentUseCase::get(&name)
        .map(|document| Json(LegalDocumentJson::from(document)))
        .ok_or(StatusCode::NOT_FOUND)
}
