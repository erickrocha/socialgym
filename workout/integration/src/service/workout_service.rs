use crate::infrastructure::mapper::{ExerciseMapper, Mapper, WorkoutMapper};
use crate::infrastructure::utils::{
    business_status, require_acting_owner, require_active_profile, require_actor, validate_uuid,
};
use crate::proto::workout::assigned_workout_list_request::Identifier as AssignedIdentifier;
use crate::proto::workout::workout_list_request::Identifier as OwnerIdentifier;
use crate::proto::workout::workout_request::Identifier;
use crate::proto::workout::workout_service_server::WorkoutService;
use crate::proto::workout::{
    AssignedWorkoutListRequest, Workout, WorkoutExercisesRequest, WorkoutExercisesResponse,
    WorkoutListRequest, WorkoutRequest, WorkoutResponse,
};
use business::domain::exercise::Exercise;
use business::gateway::business_profile_gateway::BusinessProfileGateway;
use business::use_cases::workout_use_case::WorkoutUseCase;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tonic::{Request, Response, Status};

pub struct GrpcWorkoutService {
    conn: Arc<DatabaseConnection>,
}

impl GrpcWorkoutService {
    pub fn new(conn: Arc<DatabaseConnection>) -> Self {
        Self { conn }
    }

    /// Resolves a `WorkoutRequest` identifier to the workout's uuid (the
    /// assignment-transition use cases are uuid-addressed).
    async fn resolve_uuid(&self, req: WorkoutRequest) -> Result<String, Status> {
        match req.identifier {
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                Ok(uuid)
            }
            Some(Identifier::Id(id)) => {
                let workout = WorkoutUseCase::get(&self.conn, id)
                    .await
                    .map_err(business_status)?;
                workout
                    .uuid
                    .ok_or_else(|| Status::internal("Workout missing uuid"))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }
}

#[tonic::async_trait]
impl WorkoutService for GrpcWorkoutService {
    async fn get_workout(
        &self,
        request: Request<WorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let acting = require_acting_owner(&request)?;
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
        WorkoutUseCase::ensure_readable(&self.conn, &workout, &acting)
            .await
            .map_err(business_status)?;
        let workout = WorkoutUseCase::redact_unreadable_exercises(&self.conn, workout, &acting)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(workout)))
    }

    async fn get_workouts_by_owner(
        &self,
        request: Request<WorkoutListRequest>,
    ) -> Result<Response<WorkoutResponse>, Status> {
        let acting = require_acting_owner(&request)?;
        let req = request.into_inner();
        match req.identifier {
            Some(OwnerIdentifier::OwnerId(owner_id)) => {
                let workouts =
                    WorkoutUseCase::find_readable_by_person_id(&self.conn, owner_id, &acting)
                        .await
                        .map_err(business_status)?;

                let grpc_workouts = WorkoutMapper::response_vec(workouts);
                Ok(Response::new(WorkoutResponse {
                    workouts: grpc_workouts,
                }))
            }
            Some(OwnerIdentifier::OwnerUuid(owner_uuid)) => {
                validate_uuid(&owner_uuid, "owner_uuid")?;
                let workouts =
                    WorkoutUseCase::find_readable_by_owner_uuid(&self.conn, owner_uuid, &acting)
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

    async fn get_workouts_assigned_by_profile(
        &self,
        request: Request<AssignedWorkoutListRequest>,
    ) -> Result<Response<WorkoutResponse>, Status> {
        require_actor(&request)?;
        let acting_profile = require_active_profile(&request)
            .ok_or_else(|| Status::failed_precondition("An active business profile is required"))?;
        let req = request.into_inner();

        let profile_id = match req.identifier {
            Some(AssignedIdentifier::BusinessProfileId(id)) => id,
            Some(AssignedIdentifier::BusinessProfileUuid(uuid)) => {
                validate_uuid(&uuid, "business_profile_uuid")?;
                BusinessProfileGateway::find_by_uuid(&self.conn, &uuid)
                    .await
                    .map_err(|e| Status::internal(format!("Error finding business profile: {e}")))?
                    .ok_or_else(|| Status::not_found("Business profile not found"))?
                    .id
            }
            None => return Err(Status::invalid_argument("Identifier is required")),
        };

        if acting_profile.id != Some(profile_id) {
            return Err(Status::permission_denied(
                "Cannot list assignments for another business profile",
            ));
        }

        let workouts = WorkoutUseCase::find_all_assigned_by_profile(&self.conn, profile_id)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutResponse {
            workouts: WorkoutMapper::response_vec(workouts),
        }))
    }

    async fn add_workout(&self, request: Request<Workout>) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let active_profile = require_active_profile(&request);
        let payload = request.into_inner();
        let assign_to_person_uuid = if payload.target_person_uuid.is_empty() {
            None
        } else {
            Some(payload.target_person_uuid.clone())
        };
        let workout = WorkoutMapper::domain(payload);
        let created_workout = WorkoutUseCase::persist(
            &self.conn,
            workout,
            &actor,
            active_profile.as_ref(),
            assign_to_person_uuid.as_deref(),
        )
        .await
        .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(created_workout)))
    }

    async fn update_workout(&self, request: Request<Workout>) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let active_profile = require_active_profile(&request);
        let payload = request.into_inner();
        let workout = WorkoutMapper::domain(payload);
        let updated_workout =
            WorkoutUseCase::persist(&self.conn, workout, &actor, active_profile.as_ref(), None)
                .await
                .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(updated_workout)))
    }

    async fn delete_workout(
        &self,
        request: Request<WorkoutRequest>,
    ) -> Result<Response<()>, Status> {
        let acting = require_acting_owner(&request)?;
        let payload = request.into_inner();
        match payload.identifier {
            Some(Identifier::Id(id)) => {
                WorkoutUseCase::delete_by_id(&self.conn, id, &acting)
                    .await
                    .map_err(business_status)?;
                Ok(Response::new(()))
            }
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                WorkoutUseCase::delete_by_uuid(&self.conn, uuid, &acting)
                    .await
                    .map_err(business_status)?;
                Ok(Response::new(()))
            }
            None => Err(Status::invalid_argument("Identifier is required")),
        }
    }

    async fn accept_workout(
        &self,
        request: Request<WorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let uuid = self.resolve_uuid(request.into_inner()).await?;
        let workout = WorkoutUseCase::accept_assignment(&self.conn, uuid, actor.person_id)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(workout)))
    }

    async fn reject_workout(
        &self,
        request: Request<WorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let uuid = self.resolve_uuid(request.into_inner()).await?;
        let workout = WorkoutUseCase::reject_assignment(&self.conn, uuid, actor.person_id)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(workout)))
    }

    async fn cancel_workout(
        &self,
        request: Request<WorkoutRequest>,
    ) -> Result<Response<Workout>, Status> {
        require_actor(&request)?;
        let profile_id = require_active_profile(&request)
            .and_then(|p| p.id)
            .ok_or_else(|| {
                Status::failed_precondition("An active business profile is required to cancel")
            })?;
        let uuid = self.resolve_uuid(request.into_inner()).await?;
        let workout = WorkoutUseCase::cancel_assignment(&self.conn, uuid, profile_id)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(workout)))
    }

    async fn add_exercises_to_workout(
        &self,
        request: Request<WorkoutExercisesRequest>,
    ) -> Result<Response<Workout>, Status> {
        let actor = require_actor(&request)?;
        let active_profile = require_active_profile(&request);
        let payload = request.into_inner();
        // By uuid, or by the numeric id when no uuid is given, as the two REST routes do.
        let workout = if !payload.workout_uuid.is_empty() {
            validate_uuid(&payload.workout_uuid, "workout_uuid")?;
            WorkoutUseCase::get_by_uuid(&self.conn, payload.workout_uuid).await
        } else if payload.workout_id > 0 {
            WorkoutUseCase::get(&self.conn, payload.workout_id).await
        } else {
            return Err(Status::invalid_argument(
                "either workout_uuid or workout_id must be informed",
            ));
        }
        .map_err(business_status)?;

        let domain_exercises: Vec<Exercise> = ExerciseMapper::domain_vec(payload.exercises);
        WorkoutUseCase::add_exercises_as(
            &self.conn,
            &workout,
            domain_exercises,
            &actor,
            active_profile.as_ref(),
        )
        .await
        .map_err(|error| business_status(error.into_business()))?;
        // The full composition, not just the exercises added by this call.
        let workout = WorkoutUseCase::get(&self.conn, workout.id.unwrap_or_default())
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutMapper::response(workout)))
    }

    async fn get_workout_exercises(
        &self,
        request: Request<WorkoutRequest>,
    ) -> Result<Response<WorkoutExercisesResponse>, Status> {
        let acting = require_acting_owner(&request)?;
        let workout_id = match request.into_inner().identifier {
            Some(Identifier::Id(id)) => id,
            Some(Identifier::Uuid(uuid)) => {
                validate_uuid(&uuid, "uuid")?;
                WorkoutUseCase::get_by_uuid(&self.conn, uuid)
                    .await
                    .map_err(business_status)?
                    .id
                    .unwrap_or_default()
            }
            None => return Err(Status::invalid_argument("Identifier is required")),
        };
        let exercises = WorkoutUseCase::readable_exercises(&self.conn, workout_id, &acting)
            .await
            .map_err(business_status)?;
        Ok(Response::new(WorkoutExercisesResponse {
            exercises: ExerciseMapper::response_vec(exercises),
        }))
    }
}
