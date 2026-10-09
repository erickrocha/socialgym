//! PushDeviceService: the push devices of the caller (`/timeline/api/push-devices`).
use crate::infrastructure::utils::{business_status, with_caller};
use crate::proto::timeline::push_device_service_server::PushDeviceService;
use crate::proto::timeline::*;
use business::use_cases::push_device_use_case::PushDeviceUseCase;
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcPushDeviceService {
    database: Arc<Database>,
}

impl GrpcPushDeviceService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl PushDeviceService for GrpcPushDeviceService {
    async fn register_push_device(
        &self,
        request: Request<RegisterPushDeviceRequest>,
    ) -> Result<Response<RegisterPushDeviceResponse>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            PushDeviceUseCase::register(
                &db,
                &user.person_uuid,
                &body.device_uuid,
                &body.platform,
                &body.registration_token,
            )
            .await
            .map_err(|e| business_status(&e))?;
            Ok(Response::new(RegisterPushDeviceResponse {}))
        })
        .await
    }

    async fn remove_push_device(
        &self,
        request: Request<RemovePushDeviceRequest>,
    ) -> Result<Response<RemovePushDeviceResponse>, Status> {
        let (db, device) = (self.database.clone(), request.get_ref().device_uuid.clone());
        with_caller(&request, |user| async move {
            PushDeviceUseCase::remove(&db, &user.person_uuid, &device)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(RemovePushDeviceResponse {}))
        })
        .await
    }
}
