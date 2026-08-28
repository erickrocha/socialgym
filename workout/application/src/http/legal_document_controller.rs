use axum::extract::Path;
use axum::http::StatusCode;
use axum::Json;
use business::commons::legal_documents;
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

fn document(name: &str) -> Option<LegalDocumentJson> {
    let (document, title, content) = match name {
        legal_documents::TERMS => (
            legal_documents::TERMS,
            "Termos de Uso",
            include_str!("../../resources/legal/pt-BR/terms.md"),
        ),
        legal_documents::PRIVACY => (
            legal_documents::PRIVACY,
            "Política de Privacidade",
            include_str!("../../resources/legal/pt-BR/privacy.md"),
        ),
        legal_documents::HEALTH_DATA => (
            legal_documents::HEALTH_DATA,
            "Consentimento para dados de saúde",
            include_str!("../../resources/legal/pt-BR/health-data.md"),
        ),
        _ => return None,
    };
    Some(LegalDocumentJson {
        document,
        version: legal_documents::current_version(document).unwrap(),
        title,
        content,
    })
}

pub async fn list() -> Json<Vec<LegalDocumentJson>> {
    Json(
        [
            legal_documents::TERMS,
            legal_documents::PRIVACY,
            legal_documents::HEALTH_DATA,
        ]
        .into_iter()
        .filter_map(document)
        .collect(),
    )
}

pub async fn get(Path(name): Path<String>) -> Result<Json<LegalDocumentJson>, StatusCode> {
    document(&name).map(Json).ok_or(StatusCode::NOT_FOUND)
}
