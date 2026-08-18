use async_trait::async_trait;
use crate::repositories::repository::Repository;
use domain::business_error::BusinessError;
use domain::workout_session::WorkoutSession;
use mongodb::bson::{doc, DateTime};
use mongodb::{Collection, Database};
use futures::stream::TryStreamExt;

const COLLECTION_NAME: &str = "workouts";

pub struct WorkoutSessionGateway {
    collection: Collection<WorkoutSession>,
}

impl WorkoutSessionGateway {
    pub fn new(db: &Database) -> WorkoutSessionGateway {
        Self {
            collection: db.collection::<WorkoutSession>(COLLECTION_NAME),
        }
    }
}
#[async_trait]
impl Repository<WorkoutSession, String> for WorkoutSessionGateway {
    async fn persist(&self, entity: WorkoutSession) -> Result<WorkoutSession, BusinessError> {
        let result = self.collection.insert_one(entity).await;
        if let Err(e) = result {
            log::error!("Error inserting workout: {:?}", e);
            return Err(BusinessError::new("Failed to insert workout".to_string()));
        }

        let inserted_id = result.unwrap().inserted_id;
        let inserted_workout = self.collection.find_one(doc! { "_id": inserted_id }).await;

        match inserted_workout {
            Ok(Some(w)) => Ok(w),
            _ => Err(BusinessError::new(
                "Failed to retrieve inserted workout".to_string(),
            )),
        }
    }

    async fn find_by_id(&self, id: String) -> Option<WorkoutSession> {
        let filter = doc! { "_id": id };
        let result = self.collection.find_one(filter).await;
        result.unwrap_or_else(|e| {
            log::error!("Error retrieving workout by id: {:?}", e);
            None
        })
    }

    async fn update(&self, id: String, entity: WorkoutSession) -> Result<WorkoutSession, BusinessError> {
        let filter = doc! { "_id": id.clone() };

        let result = self.collection.replace_one(filter, entity).await;

        match result {
            Ok(update_result) if update_result.matched_count > 0 => {
                let updated_workout = self.collection.find_one(doc! { "_id": id }).await;
                match updated_workout {
                    Ok(Some(w)) => Ok(w),
                    _ => Err(BusinessError::new(
                        "Failed to retrieve updated workout".to_string(),
                    )),
                }
            }
            Ok(_) => Err(BusinessError::new("No workout found to update".to_string())),
            Err(e) => {
                log::error!("Error updating workout: {:?}", e);
                Err(BusinessError::new("Failed to update workout".to_string()))
            }
        }
    }

    async fn delete(&self, id: String) -> Result<bool, BusinessError> {
        let filter = doc! { "_id": id };

        let result = self.collection.delete_one(filter).await;

        match result {
            Ok(delete_result) => Ok(delete_result.deleted_count > 0),
            Err(e) => {
                log::error!("Error deleting workout: {:?}", e);
                Err(BusinessError::new("Failed to delete workout".to_string()))
            }
        }
    }
}

impl WorkoutSessionGateway {
    pub async fn find_all_by_person(&self,person_uuid: &str,start: DateTime,end: DateTime) -> Vec<WorkoutSession> {
        let filter = doc! {
            "personUuid": person_uuid,
            "startedAt": { "$gte": start, "$lte": end }
        };
        let cursor = self.collection.find(filter).await;

        if cursor.is_err() {
            log::error!("Error finding workouts by person: {:?}", cursor.err().unwrap());
            return Vec::new();
        }

        let mut cursor = cursor.unwrap();
        let mut workouts = Vec::new();
        while let Some(workout) = cursor.try_next().await.unwrap_or(None) {
            workouts.push(workout);
        }
        workouts
    }
}
