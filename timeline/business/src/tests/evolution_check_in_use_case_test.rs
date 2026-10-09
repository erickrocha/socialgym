use super::EvolutionCheckInUseCase;
use crate::gateway::consent_gateway::HealthConsentGatewayPort;
use crate::gateway::evolution_check_in_gateway::EvolutionCheckInGatewayPort;
use domain::business_error::BusinessError;
use domain::enums::Visibility;
use domain::evolution_check_in::EvolutionCheckIn;
use mongodb::bson::{DateTime, Document, from_document, to_document};
use std::sync::Mutex;

struct FakeConsentGateway {
    allowed: bool,
}

#[async_trait::async_trait]
impl HealthConsentGatewayPort for FakeConsentGateway {
    async fn require_health_consent(&self) -> Result<(), BusinessError> {
        if self.allowed {
            Ok(())
        } else {
            Err(BusinessError::forbidden("health consent required"))
        }
    }
}

#[derive(Default)]
struct FakeCheckInGateway {
    documents: Mutex<Vec<Document>>,
    last_query: Mutex<Option<(String, DateTime, DateTime)>>,
}

#[async_trait::async_trait]
impl EvolutionCheckInGatewayPort for FakeCheckInGateway {
    async fn persist_check_in(
        &self,
        check_in: EvolutionCheckIn,
    ) -> Result<EvolutionCheckIn, BusinessError> {
        let document = to_document(&check_in)
            .map_err(|error| BusinessError::infrastructure(error.to_string()))?;
        self.documents.lock().unwrap().push(document);
        Ok(check_in)
    }

    async fn find_check_in(&self, id: String) -> Option<EvolutionCheckIn> {
        self.documents
            .lock()
            .unwrap()
            .iter()
            .find(|document| document.get_str("_id").ok() == Some(id.as_str()))
            .cloned()
            .and_then(|document| from_document(document).ok())
    }

    async fn find_all_by_person_uuid(
        &self,
        person_uuid: String,
        start: DateTime,
        end: DateTime,
    ) -> Vec<EvolutionCheckIn> {
        *self.last_query.lock().unwrap() = Some((person_uuid.clone(), start, end));
        self.documents
            .lock()
            .unwrap()
            .iter()
            .filter(|document| {
                document.get_str("personUuid").ok() == Some(person_uuid.as_str())
                    && document
                        .get_datetime("createdAt")
                        .is_ok_and(|created_at| *created_at >= start && *created_at <= end)
            })
            .cloned()
            .filter_map(|document| from_document(document).ok())
            .collect()
    }
}

fn check_in(uuid: &str, person_uuid: &str, created_at_ms: i64) -> EvolutionCheckIn {
    EvolutionCheckIn::new(
        uuid.to_string(),
        person_uuid.to_string(),
        DateTime::from_millis(created_at_ms),
        None,
        Visibility::Private,
        None,
        None,
    )
}

#[tokio::test]
async fn add_requires_health_consent_before_persisting() {
    let gateway = FakeCheckInGateway::default();
    let use_case = EvolutionCheckInUseCase::new(gateway, FakeConsentGateway { allowed: false });

    let error = use_case
        .add(check_in("c1", "spoofed-owner", 10), "caller")
        .await
        .unwrap_err();

    assert_eq!(
        error.kind,
        domain::business_error::BusinessErrorKind::Forbidden
    );
    assert!(use_case.gateway.documents.lock().unwrap().is_empty());
}

#[tokio::test]
async fn add_overwrites_client_owner_with_authenticated_caller() {
    let gateway = FakeCheckInGateway::default();
    let use_case = EvolutionCheckInUseCase::new(gateway, FakeConsentGateway { allowed: true });

    let saved = use_case
        .add(check_in("c1", "spoofed-owner", 10), "caller")
        .await
        .unwrap();

    assert_eq!(saved.person_uuid, "caller");
    let stored: EvolutionCheckIn =
        from_document(use_case.gateway.documents.lock().unwrap()[0].clone()).unwrap();
    assert_eq!(stored.person_uuid, "caller");
}

#[tokio::test]
async fn find_distinguishes_missing_check_in_from_another_owners_check_in() {
    let gateway = FakeCheckInGateway::default();
    let use_case = EvolutionCheckInUseCase::new(gateway, FakeConsentGateway { allowed: true });

    assert_eq!(
        use_case
            .find("missing".to_string(), "owner")
            .await
            .unwrap_err()
            .kind,
        domain::business_error::BusinessErrorKind::NotFound
    );
    use_case
        .gateway
        .persist_check_in(check_in("private", "owner-a", 10))
        .await
        .unwrap();
    assert_eq!(
        use_case
            .find("private".to_string(), "owner-b")
            .await
            .unwrap_err()
            .kind,
        domain::business_error::BusinessErrorKind::Forbidden
    );
    assert_eq!(
        use_case
            .find("private".to_string(), "owner-a")
            .await
            .unwrap()
            .uuid,
        "private"
    );
}

#[tokio::test]
async fn find_all_forwards_owner_and_inclusive_date_range() {
    let gateway = FakeCheckInGateway::default();
    let use_case = EvolutionCheckInUseCase::new(gateway, FakeConsentGateway { allowed: true });
    use_case
        .gateway
        .persist_check_in(check_in("in-range", "owner-a", 10))
        .await
        .unwrap();
    use_case
        .gateway
        .persist_check_in(check_in("before", "owner-a", 9))
        .await
        .unwrap();
    use_case
        .gateway
        .persist_check_in(check_in("other-owner", "owner-b", 10))
        .await
        .unwrap();

    let results = use_case
        .find_all_by_owner(
            "owner-a".to_string(),
            DateTime::from_millis(10),
            DateTime::from_millis(20),
        )
        .await;

    assert_eq!(results.len(), 1);
    assert_eq!(results[0].uuid, "in-range");
    let query = use_case.gateway.last_query.lock().unwrap().clone().unwrap();
    assert_eq!(query.0, "owner-a");
    assert_eq!(query.1, DateTime::from_millis(10));
    assert_eq!(query.2, DateTime::from_millis(20));
}
