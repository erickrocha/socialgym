//! HealthService: tells an authenticated caller the service is up and who it is.
use crate::infrastructure::utils::caller;
use crate::proto::timeline::health_service_server::HealthService;
use crate::proto::timeline::{HealthRequest, HealthResponse};
use tonic::{Request, Response, Status};

pub struct GrpcHealthService;

#[tonic::async_trait]
impl HealthService for GrpcHealthService {
    async fn check(
        &self,
        request: Request<HealthRequest>,
    ) -> Result<Response<HealthResponse>, Status> {
        let (user, _) = caller(&request)?;
        Ok(Response::new(HealthResponse {
            status: "ok".into(),
            person_uuid: user.person_uuid,
        }))
    }
}
