use crate::infrastructure::mapper::{ExerciseMapper, Mapper};
use crate::proto::exercise::exercise_request::Identifier;
use crate::proto::exercise::exercise_service_server::ExerciseService;
use crate::proto::exercise::{Exercise, ExerciseParams, ExerciseRequest, PaginatedExercise};
use business::commons::functions::string_to_uuid;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

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
        let req = request.into_inner();
        match req.identifier {
            Some(Identifier::Id(id)) => {
                let exercise = ExerciseUseCase::get(&self.conn, id)
                    .await
                    .ok_or_else(|| Status::not_found("exercise not found"))?;
                let grpc_exercise = ExerciseMapper::response(exercise);
                Ok(Response::new(grpc_exercise))
            }
            Some(Identifier::Uuid(uuid)) => {
                let exercise = ExerciseUseCase::get_by_uuid(&self.conn, uuid)
                    .await
                    .ok_or_else(|| Status::not_found("exercise not found"))?;
                let grpc_exercise = ExerciseMapper::response(exercise);
                Ok(Response::new(grpc_exercise))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn get_exercises(
        &self,
        request: Request<ExerciseParams>,
    ) -> Result<Response<PaginatedExercise>, Status> {
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

        // Get public_owner_ids, default to empty if not provided
        let public_owner_uuids = req
            .owners
            .iter()
            .map(|owner| string_to_uuid(owner))
            .collect::<Vec<Uuid>>();
        let result = ExerciseUseCase::find_by_complex_filters_paginated_uuid(
            &self.conn,
            string_to_uuid(&req.owner_uuid),
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
        let payload = request.into_inner();
        let exercise = ExerciseMapper::domain(payload);
        let persisted = ExerciseUseCase::persist(&self.conn, exercise).await;
        if persisted.is_none() {
            return Err(Status::internal("Failed to persist exercise"));
        }
        let grpc_exercise = ExerciseMapper::response(persisted.unwrap());
        Ok(Response::new(grpc_exercise))
    }

    async fn update_exercise(
        &self,
        request: Request<Exercise>,
    ) -> Result<Response<Exercise>, Status> {
        let payload = request.into_inner();
        let exercise = ExerciseMapper::domain(payload);
        let persisted = ExerciseUseCase::persist(&self.conn, exercise).await;
        if persisted.is_none() {
            return Err(Status::internal("Failed to persist exercise"));
        }
        let grpc_exercise = ExerciseMapper::response(persisted.unwrap());
        Ok(Response::new(grpc_exercise))
    }

    async fn delete_exercise(
        &self,
        request: Request<ExerciseRequest>,
    ) -> Result<Response<()>, Status> {
        let req = request.into_inner();
        match req.identifier {
            Some(Identifier::Id(id)) => {
                let result = ExerciseUseCase::delete_by_id(&self.conn, id).await;
                if result.is_err() {
                    return Err(Status::internal("Failed to delete exercise"));
                }
                Ok(Response::new(()))
            }
            Some(Identifier::Uuid(uuid)) => {
                let result = ExerciseUseCase::delete_by_uuid(&self.conn, uuid).await;
                if result.is_err() {
                    return Err(Status::internal("Failed to delete exercise"));
                }
                Ok(Response::new(()))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }
}
