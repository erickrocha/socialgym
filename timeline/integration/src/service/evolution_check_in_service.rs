//! EvolutionCheckInService: the check-ins of the caller (`/timeline/api/evolution-checkin`).
use crate::infrastructure::mapper::{EvolutionCheckInMapper, optional_text_to_date};
use crate::infrastructure::utils::{business_status, with_caller};
use crate::proto::timeline::evolution_check_in_service_server::EvolutionCheckInService;
use crate::proto::timeline::*;
use business::commons::data_tools::opt_naive_to_bson_datetime;
use business::gateway::consent_gateway::ConsentGateway;
use business::gateway::evolution_check_in_gateway::EvolutionCheckInGateway;
use business::use_cases::evolution_check_in_use_case::EvolutionCheckInUseCase;
use chrono::{Duration, Utc};
use mongodb::Database;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcEvolutionCheckInService {
    database: Arc<Database>,
}

impl GrpcEvolutionCheckInService {
    pub fn new(database: Arc<Database>) -> Self {
        Self { database }
    }
}

#[tonic::async_trait]
impl EvolutionCheckInService for GrpcEvolutionCheckInService {
    async fn add_evolution_check_in(&self, request: Request<AddEvolutionCheckInRequest>) -> Result<Response<EvolutionCheckIn>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let check_in = EvolutionCheckInMapper::domain(body, &user.person_uuid)?;
            let use_case = EvolutionCheckInUseCase::new(EvolutionCheckInGateway::new(&db), ConsentGateway);
            let saved = use_case
                .add(check_in, &user.person_uuid)
                .await
                .map_err(|e| business_status(&e))?;
            Ok(Response::new(EvolutionCheckInMapper::proto(saved)))
        })
        .await
    }

    async fn list_evolution_check_ins(&self, request: Request<ListEvolutionCheckInsRequest>) -> Result<Response<ListEvolutionCheckInsResponse>, Status> {
        let (db, body) = (self.database.clone(), request.get_ref().clone());
        with_caller(&request, |user| async move {
            let end = optional_text_to_date(body.end_date.as_deref())?.unwrap_or_else(|| Utc::now().naive_utc());
            let start = optional_text_to_date(body.start_date.as_deref())?.unwrap_or_else(|| end - Duration::days(7));
            let use_case = EvolutionCheckInUseCase::new(EvolutionCheckInGateway::new(&db), ConsentGateway);
            let found = use_case
                .find_all_by_owner(
                    user.person_uuid,
                    opt_naive_to_bson_datetime(start).unwrap(),
                    opt_naive_to_bson_datetime(end).unwrap(),
                )
                .await;
            Ok(Response::new(ListEvolutionCheckInsResponse {
                check_ins: found.into_iter().map(EvolutionCheckInMapper::proto).collect(),
            }))
        })
        .await
    }
}
