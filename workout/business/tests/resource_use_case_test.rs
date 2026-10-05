use business::domain::business_error::BusinessError;
use business::domain::business_error::BusinessErrorKind;
use business::domain::country::Country;
use business::gateway::country_gateway::{CountryGateway, CountryGatewayPort};
use business::use_cases::resource_use_case::ResourceUseCase;
use entity::country_entity::CountryEntity;
use sea_orm::{DatabaseBackend, MockDatabase};

fn country(id: i32, name: &str) -> CountryEntity {
    CountryEntity {
        id,
        ddi: format!("+{id}"),
        name: name.to_string(),
        acronym: format!("C{id}"),
        currency: "TST".to_string(),
    }
}

struct FakeCountryGateway {
    countries: Vec<Country>,
    failure: Option<BusinessError>,
}

impl CountryGatewayPort for FakeCountryGateway {
    async fn find_all(&self) -> Result<Vec<Country>, BusinessError> {
        if let Some(error) = &self.failure {
            Err(BusinessError::new(error.message.clone()))
        } else {
            Ok(self.countries.clone())
        }
    }
}

#[tokio::test]
async fn returns_empty_list_when_no_countries_are_configured() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![Vec::<CountryEntity>::new()])
        .into_connection();

    let countries = ResourceUseCase::new(CountryGateway::new(db))
        .get_countries()
        .await
        .unwrap();

    assert!(countries.is_empty());
}

#[tokio::test]
async fn maps_country_fields_from_the_entity_layer() {
    let db = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![country(55, "Testland")]])
        .into_connection();

    let countries = ResourceUseCase::new(CountryGateway::new(db))
        .get_countries()
        .await
        .unwrap();

    assert_eq!(countries.len(), 1);
    assert_eq!(countries[0].id, Some(55));
    assert_eq!(countries[0].ddi, "+55");
    assert_eq!(countries[0].name, "Testland");
    assert_eq!(countries[0].acronym, "C55");
    assert_eq!(countries[0].currency, "TST");
}

#[tokio::test]
async fn maps_gateway_database_failure_to_infrastructure_error() {
    let error = ResourceUseCase::new(FakeCountryGateway {
        countries: Vec::new(),
        failure: Some(BusinessError::infrastructure("database unavailable")),
    })
    .get_countries()
    .await
    .unwrap_err();

    assert_eq!(error.kind, BusinessErrorKind::Infrastructure);
    assert_eq!(error.message, "database unavailable");
}
