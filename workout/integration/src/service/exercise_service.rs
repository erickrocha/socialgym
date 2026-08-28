use crate::infrastructure::mapper::{ExerciseMapper, Mapper};
use crate::proto::exercise::exercise_request::Identifier;
use crate::proto::exercise::exercise_service_server::ExerciseService;
use crate::proto::exercise::{Exercise, ExerciseParams, ExerciseRequest, PaginatedExercise};
use business::use_cases::exercise_use_case::ExerciseUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::infrastructure::utils::{
    business_status, require_active_profile, require_actor, validate_uuid, validate_uuids,
};

pub struct GrpcExerciseService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcExerciseService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[tonic::async_trait]
impl ExerciseService for GrpcExerciseService {
    async fn get_exercise(
        &self,
        request: Request<ExerciseRequest>,
    ) -> Result<Response<Exercise>, Status> {
        let actor = require_actor(&request)?;
        let req = request.into_inner();
        let exercise = match req.identifier {
            Some(Identifier::Id(id)) => ExerciseUseCase::get(&self.conn, id)
                .await
                .map_err(business_status)?,
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                ExerciseUseCase::get_by_uuid(&self.conn, uuid)
                    .await
                    .map_err(business_status)?
            }
            None => return Err(Status::invalid_argument("Identifier is required")),
        };
        ExerciseUseCase::ensure_readable(&exercise, actor.person_id).map_err(business_status)?;
        Ok(Response::new(ExerciseMapper::response(exercise)))
    }

    async fn get_exercises(
        &self,
        request: Request<ExerciseParams>,
    ) -> Result<Response<PaginatedExercise>, Status> {
        let actor = require_actor(&request)?;
        let active_profile = require_active_profile(&request);
        let req = request.into_inner();
        let page_number = req.page_number;
        let page_size = req.page_size;

        let category = Some(req.category);
        let visibility = Some(req.visibility);
        let sort_by = Some(req.sort_by);

        // Validate page_number
        if page_number < 1 {
            return Err(Status::invalid_argument(
                "page_number must be greater than 0",
            ));
        }

        // Validate and cap page_size
        if page_size < 1 {
            return Err(Status::invalid_argument("page_size must be greater than 0"));
        }

        let capped_page_size = if page_size > 100 { 100 } else { page_size };

        validate_uuids(&req.owners, "owners")?;

        // Get public_owner_ids, default to empty if not provided
        let public_owner_uuids = req.owners.iter().cloned().collect::<Vec<String>>();
        // The query always runs as the caller's acting identity (their active
        // business profile, if any, else themself); `owner_uuid` in the
        // request is ignored.
        let acting_owner_uuid = active_profile
            .and_then(|p| p.uuid)
            .unwrap_or_else(|| actor.person_uuid.clone());
        let result = ExerciseUseCase::find_by_complex_filters_paginated_uuid(
            &self.conn,
            acting_owner_uuid,
            public_owner_uuids,
            category,
            visibility,
            page_number as u64,
            capped_page_size as u64,
            sort_by,
        )
        .await;
        Ok(Response::new(PaginatedExercise {
            content: ExerciseMapper::response_vec(result.0),
            total_count: result.1,
            page_number,
            page_size: capped_page_size,
            has_next_page: result.2,
        }))
    }

    async fn add_exercise(&self, request: Request<Exercise>) -> Result<Response<Exercise>, Status> {
        let actor = require_actor(&request)?;
        let active_profile = require_active_profile(&request);
        let payload = request.into_inner();
        let exercise = ExerciseMapper::domain(payload);
        let persisted =
            ExerciseUseCase::persist(&self.conn, exercise, &actor, active_profile.as_ref()).await;
        let grpc_exercise = ExerciseMapper::response(persisted.map_err(business_status)?);
        Ok(Response::new(grpc_exercise))
    }

    async fn update_exercise(
        &self,
        request: Request<Exercise>,
    ) -> Result<Response<Exercise>, Status> {
        let actor = require_actor(&request)?;
        let active_profile = require_active_profile(&request);
        let payload = request.into_inner();
        let exercise = ExerciseMapper::domain(payload);
        let persisted =
            ExerciseUseCase::persist(&self.conn, exercise, &actor, active_profile.as_ref()).await;
        let grpc_exercise = ExerciseMapper::response(persisted.map_err(business_status)?);
        Ok(Response::new(grpc_exercise))
    }

    async fn delete_exercise(
        &self,
        request: Request<ExerciseRequest>,
    ) -> Result<Response<()>, Status> {
        let actor = require_actor(&request)?;
        let req = request.into_inner();
        match req.identifier {
            Some(Identifier::Id(id)) => {
                ExerciseUseCase::delete_by_id(&self.conn, id, actor.person_id)
                    .await
                    .map_err(business_status)?;
                Ok(Response::new(()))
            }
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                ExerciseUseCase::delete_by_uuid(&self.conn, uuid, actor.person_id)
                    .await
                    .map_err(business_status)?;
                Ok(Response::new(()))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }
}
