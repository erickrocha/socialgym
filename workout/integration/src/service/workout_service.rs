use std::sync::Arc;
use hyper::StatusCode;
use business::use_cases::workout_use_case::WorkoutUseCase;
use crate::service::validate_uuid;
use sea_orm::DatabaseConnection;
use tonic::{Request, Response, Status};
use business::domain::exercise::Exercise;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use crate::infrastructure::mapper::{ExerciseMapper, Mapper, WorkoutMapper};
use crate::proto::workout::workout_service_server::WorkoutService;
use crate::proto::workout::{Workout, WorkoutExercisesRequest, WorkoutListRequest, WorkoutRequest, WorkoutResponse};
use crate::proto::workout::workout_request::Identifier;
use crate::proto::workout::workout_list_request::Identifier as OwnerIdentifier;

pub struct GrpcWorkoutService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcWorkoutService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }
}

#[tonic::async_trait]
impl WorkoutService for GrpcWorkoutService {
    async fn get_workout(&self, request: Request<WorkoutRequest>) -> Result<Response<Workout>, Status> {
        let req = request.into_inner();
        match req.identifier {
            Some(Identifier::Id(id)) => {
                let workout = WorkoutUseCase::get(&self.conn, id)
                    .await
                    .ok_or_else(|| Status::not_found("workout not found"))?;

                let grpc_workout = WorkoutMapper::response(workout);
                Ok(Response::new(grpc_workout))
            }
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                let workout = WorkoutUseCase::get_by_uuid(&self.conn, uuid)
                    .await
                    .ok_or_else(|| Status::not_found("workout not found"))?;

                let grpc_workout = WorkoutMapper::response(workout);
                Ok(Response::new(grpc_workout))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn get_workouts_by_owner(&self, request: Request<WorkoutListRequest>) -> Result<Response<WorkoutResponse>, Status> {
        let req = request.into_inner();
        match req.identifier {
            Some(OwnerIdentifier::OwnerId(owner_id)) => {
                let workouts = WorkoutUseCase::find_all_by_owner_id(&self.conn, owner_id).await;


                let grpc_workouts = WorkoutMapper::response_vec(workouts);
                Ok(Response::new(WorkoutResponse{workouts: grpc_workouts}))
            }
            Some(OwnerIdentifier::OwnerUuid(owner_uuid)) => {
                validate_uuid(&owner_uuid, "owner_uuid")?;
                let workouts = WorkoutUseCase::find_all_by_owner_uuid(&self.conn, owner_uuid)
                    .await;
                let grpc_workouts = WorkoutMapper::response_vec(workouts);
                Ok(Response::new(WorkoutResponse{workouts: grpc_workouts}))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn add_workout(&self, request: Request<Workout>) -> Result<Response<Workout>, Status> {
        let payload = request.into_inner();
        let workout = WorkoutMapper::domain(payload);
        let created_workout = WorkoutUseCase::persist(&self.conn, workout).await;
        if created_workout.is_err() {
            return Err(Status::internal("Failed to create workout"));
        }
        let grpc_workout = WorkoutMapper::response(created_workout.unwrap());
        Ok(Response::new(grpc_workout))
    }

    async fn update_workout(&self, request: Request<Workout>) -> Result<Response<Workout>, Status> {
        let payload = request.into_inner();
        let workout = WorkoutMapper::domain(payload);
        let updated_workout = WorkoutUseCase::persist(&self.conn, workout).await;
        if updated_workout.is_err() {
            return Err(Status::internal("Failed to update workout"));
        }
        let grpc_workout = WorkoutMapper::response(updated_workout.unwrap());
        Ok(Response::new(grpc_workout))
    }

    async fn delete_workout(&self, request: Request<WorkoutRequest>) -> Result<Response<()>, Status> {
        let payload = request.into_inner();
        match payload.identifier {
            Some(Identifier::Id(id)) => {
                let result = WorkoutUseCase::delete_by_id(&self.conn, id).await;
                if result.is_err() {
                    return Err(Status::internal("Failed to delete workout"));
                }
                Ok(Response::new(()))
            }
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                let result = WorkoutUseCase::delete_by_uuid(&self.conn, uuid).await;
                if result.is_err() {
                    return Err(Status::internal("Failed to delete workout"));
                }
                Ok(Response::new(()))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn add_exercises_to_workout(&self, request: Request<WorkoutExercisesRequest>) -> Result<Response<Workout>, Status> {
        let payload = request.into_inner();
        validate_uuid(&payload.workout_uuid, "workout_uuid")?;
        let workout_result = WorkoutUseCase::get_by_uuid(&self.conn, payload.workout_uuid.clone()).await;

        if workout_result.is_none() {
            log::error!("Workout not found for workout_id={}", payload.workout_uuid);
            return Err(Status::not_found("Workout not found"));
        }

        let workout = workout_result.unwrap();

        let domain_exercises: Vec<Exercise> = ExerciseMapper::domain_vec(payload.exercises);
        let result = ExerciseUseCase::add_all_to_workout(&self.conn, workout.id.unwrap(), domain_exercises).await;
        if result.is_err() {
            log::error!("Error adding exercises: {:?}", result.err().unwrap());
            return Err(Status::internal("Failed to add exercises"));
        }
        let added_exercises = result.unwrap();
        let mut workout_response = WorkoutMapper::response(workout.clone());
        workout_response.exercises = ExerciseMapper::response_vec(added_exercises);
        Ok(Response::new(workout_response))
    }
}
