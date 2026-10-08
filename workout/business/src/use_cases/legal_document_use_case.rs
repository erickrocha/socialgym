use crate::commons::legal_documents;

/// A legal document the platform tracks consent for, as the clients read it.
#[derive(Debug, Clone)]
pub struct LegalDocument {
    pub document: &'static str,
    /// The version in force (`TERMS_VERSION`, `PRIVACY_VERSION`, `HEALTH_DATA_CONSENT_VERSION`).
    pub version: String,
    pub title: &'static str,
    pub content: &'static str,
}

/// The text of the legal documents: public, static, and the same for REST and gRPC.
pub struct LegalDocumentUseCase;

impl LegalDocumentUseCase {
    pub fn get(name: &str) -> Option<LegalDocument> {
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
        Some(LegalDocument { document, version: legal_documents::current_version(document)?, title, content })
    }

    pub fn list() -> Vec<LegalDocument> {
        legal_documents::ALL.into_iter().filter_map(Self::get).collect()
    }
}
