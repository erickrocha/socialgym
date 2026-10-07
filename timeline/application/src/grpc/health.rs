use business::proto::proto::timeline::health_service_server::HealthService;
use business::proto::proto::timeline::{HealthRequest, HealthResponse};
use tonic::{Request, Response, Status};

use super::auth::caller;

pub struct Health;

#[tonic::async_trait]
impl HealthService for Health {
    async fn check(&self, request: Request<HealthRequest>) -> Result<Response<HealthResponse>, Status> {
        let (user, _) = caller(&request)?;
        Ok(Response::new(HealthResponse {
            status: "ok".into(),
            person_uuid: user.person_uuid,
        }))
    }
}
