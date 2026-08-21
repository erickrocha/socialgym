use crate::infrastructure::mapper::{ExerciseMapper, Mapper, WorkoutMapper};
use crate::proto::workout::workout_list_request::Identifier as OwnerIdentifier;
use crate::proto::workout::workout_request::Identifier;
use crate::proto::workout::workout_service_server::WorkoutService;
use crate::proto::workout::{
    Workout, WorkoutExercisesRequest, WorkoutListRequest, WorkoutRequest, WorkoutResponse,
};
use business::commons::authorization::ensure_owns;
use business::domain::exercise::Exercise;
use business::use_cases::exercise_use_case::ExerciseUseCase;
use business::use_cases::workout_use_case::WorkoutUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use crate::infrastructure::utils::{business_status, require_actor, validate_uuid};

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
    async fn get_workout(&self,request: Request<WorkoutRequest>) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let req = request.into_inner();
        let workout = match req.identifier {
            Some(Identifier::Id(id)) => WorkoutUseCase::get(&self.conn, id)
                .await
                .map_err(business_status)?,
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                WorkoutUseCase::get_by_uuid(&self.conn, uuid)
                    .await
                    .map_err(business_status)?
            }
            None => return Err(Status::invalid_argument("Identifier is required")),
        };
        WorkoutUseCase::ensure_readable(&workout, actor.person_id).map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(workout)))
    }

    async fn get_workouts_by_owner(&self,request: Request<WorkoutListRequest>) -> Result<Response<WorkoutResponse>, Status> {
        let req = request.into_inner();
        match req.identifier {
            Some(OwnerIdentifier::OwnerId(owner_id)) => {
                let workouts = WorkoutUseCase::find_all_by_owner_id(&self.conn, owner_id)
                    .await
                    .map_err(business_status)?;

                let grpc_workouts = WorkoutMapper::response_vec(workouts);
                Ok(Response::new(WorkoutResponse {
                    workouts: grpc_workouts,
                }))
            }
            Some(OwnerIdentifier::OwnerUuid(owner_uuid)) => {
                validate_uuid(&owner_uuid, "owner_uuid")?;
                let workouts = WorkoutUseCase::find_all_by_owner_uuid(&self.conn, owner_uuid)
                    .await
                    .map_err(business_status)?;
                let grpc_workouts = WorkoutMapper::response_vec(workouts);
                Ok(Response::new(WorkoutResponse {
                    workouts: grpc_workouts,
                }))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn add_workout(&self, request: Request<Workout>) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let payload = request.into_inner();
        let workout = WorkoutMapper::domain(payload);
        let created_workout = WorkoutUseCase::persist(&self.conn, workout, &actor)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(created_workout)))
    }

    async fn update_workout(&self, request: Request<Workout>) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let payload = request.into_inner();
        let workout = WorkoutMapper::domain(payload);
        let updated_workout = WorkoutUseCase::persist(&self.conn, workout, &actor)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(updated_workout)))
    }

    async fn delete_workout(&self,request: Request<WorkoutRequest>) -> Result<Response<()>, Status> {
        let actor = require_actor(&request)?;
        let payload = request.into_inner();
        match payload.identifier {
            Some(Identifier::Id(id)) => {
                WorkoutUseCase::delete_by_id(&self.conn, id, actor.person_id)
                    .await
                    .map_err(business_status)?;
                Ok(Response::new(()))
            }
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                WorkoutUseCase::delete_by_uuid(&self.conn, uuid, actor.person_id)
                    .await
                    .map_err(business_status)?;
                Ok(Response::new(()))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn add_exercises_to_workout(&self,request: Request<WorkoutExercisesRequest>) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let payload = request.into_inner();
        validate_uuid(&payload.workout_uuid, "workout_uuid")?;
        let workout_result =WorkoutUseCase::get_by_uuid(&self.conn, payload.workout_uuid.clone()).await;

        let workout = workout_result.map_err(business_status)?;
        ensure_owns(workout.owner_id, actor.person_id).map_err(business_status)?;

        let domain_exercises: Vec<Exercise> = ExerciseMapper::domain_vec(payload.exercises);
        let result = ExerciseUseCase::add_all_to_workout(
            &self.conn,
            workout.id.unwrap(),
            domain_exercises,
            &actor,
        )
        .await;
        let added_exercises = result.map_err(business_status)?;
        let mut workout_response = WorkoutMapper::response(workout.clone());
        workout_response.exercises = ExerciseMapper::response_vec(added_exercises);
        Ok(Response::new(workout_response))
    }
}
