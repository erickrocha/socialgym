use crate::infrastructure::utils::{locale_of, localized_business_status, require_actor};
use crate::proto::address::address_search_service_server::AddressSearchService;
use crate::proto::address::{AddressCandidate, SearchAddressRequest, SearchAddressResponse};
use business::commons::i18n::ErrorKey;
use business::domain::business_error::BusinessErrorKind;
use business::use_cases::address_search_use_case::AddressSearchUseCase;
use tonic::{Request, Response, Status};

/// The gRPC twin of the REST `/address/search` route; the rules live in `AddressSearchUseCase`.
pub struct GrpcAddressSearchService;

#[tonic::async_trait]
impl AddressSearchService for GrpcAddressSearchService {
    async fn search_address(
        &self,
        request: Request<SearchAddressRequest>,
    ) -> Result<Response<SearchAddressResponse>, Status> {
        let locale = locale_of(&request);
        require_actor(&request)?;
        let payload = request.into_inner();
        match AddressSearchUseCase::search(&payload.text, payload.latitude, payload.longitude).await
        {
            Ok(candidates) => Ok(Response::new(SearchAddressResponse {
                candidates: candidates
                    .into_iter()
                    .map(|c| AddressCandidate {
                        place_id: c.place_id,
                        formatted_address: c.formatted_address,
                        address_line_1: c.address_line1,
                        address_line_2: c.address_line2,
                        locality: c.locality,
                        administrative_area: c.administrative_area,
                        administrative_area_code: c.administrative_area_code,
                        postal_code: c.postal_code,
                        country_code: c.country_code,
                        latitude: c.latitude,
                        longitude: c.longitude,
                    })
                    .collect(),
            })),
            Err(error) => {
                let key = if error.kind == BusinessErrorKind::Forbidden {
                    ErrorKey::AddressSearchDisabled
                } else {
                    ErrorKey::AddressSearchFailed
                };
                Err(localized_business_status(error, key, locale))
            }
        }
    }
}
