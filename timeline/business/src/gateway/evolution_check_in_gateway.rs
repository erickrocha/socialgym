use crate::repositories::repository::Repository;
use async_trait::async_trait;
use domain::business_error::BusinessError;
use domain::evolution_check_in::EvolutionCheckIn;
use futures::TryStreamExt;
use mongodb::bson::{DateTime, doc};
use mongodb::{Collection, Cursor, Database};

const COLLECTION_NAME: &str = "evolutions";

pub struct EvolutionCheckInGateway {
    collection: Collection<EvolutionCheckIn>,
}

impl EvolutionCheckInGateway {
    pub fn new(db: &Database) -> EvolutionCheckInGateway {
        Self {
            collection: db.collection::<EvolutionCheckIn>(COLLECTION_NAME),
        }
    }
}

#[async_trait]
impl Repository<EvolutionCheckIn, String> for EvolutionCheckInGateway {
    async fn persist(&self, entity: EvolutionCheckIn) -> Result<EvolutionCheckIn, BusinessError> {
        let result = self.collection.insert_one(entity).await;
        if let Err(e) = result {
            log::error!("Error inserting evolution check-in: {:?}", e);
            return Err(BusinessError::new(
                "Failed to insert evolution check-in".to_string(),
            ));
        }

        let inserted_id = result.unwrap().inserted_id;
        let inserted_check_in = self
            .collection
            .find_one(mongodb::bson::doc! { "_id": inserted_id })
            .await;

        match inserted_check_in {
            Ok(Some(check_in)) => Ok(check_in),
            _ => Err(BusinessError::new(
                "Failed to retrieve inserted evolution check-in".to_string(),
            )),
        }
    }

    async fn find_by_id(&self, id: String) -> Option<EvolutionCheckIn> {
        let filter = doc! { "_id": id };
        let result = self.collection.find_one(filter).await;
        result.unwrap_or_else(|e| {
            log::error!("Error retrieving evolution check-in by id: {:?}", e);
            None
        })
    }

    async fn update(&self,id: String,entity: EvolutionCheckIn) -> Result<EvolutionCheckIn, BusinessError> {
        let filter = doc! { "_id": id.clone() };

        let result = self.collection.replace_one(filter, entity).await;

        match result {
            Ok(update_result) if update_result.matched_count > 0 => {
                // Retrieve the updated document
                let updated_check_in = self
                    .collection
                    .find_one(mongodb::bson::doc! { "_id": id })
                    .await;
                match updated_check_in {
                    Ok(Some(check_in)) => Ok(check_in),
                    _ => Err(BusinessError::new(
                        "Failed to retrieve updated evolution check-in".to_string(),
                    )),
                }
            }
            Ok(_) => Err(BusinessError::new(
                "No evolution check-in found to update".to_string(),
            )),
            Err(e) => {
                log::error!("Error updating evolution check-in: {:?}", e);
                Err(BusinessError::new(
                    "Failed to update evolution check-in".to_string(),
                ))
            }
        }
    }

    async fn delete(&self, id: String) -> Result<bool, BusinessError> {
        let filter = mongodb::bson::doc! { "_id": id };
        let result = self.collection.delete_one(filter).await;

        match result {
            Ok(delete_result) => Ok(delete_result.deleted_count > 0),
            Err(e) => {
                log::error!("Error deleting evolution check-in: {:?}", e);
                Err(BusinessError::new(
                    "Failed to delete evolution check-in".to_string(),
                ))
            }
        }
    }
}

impl EvolutionCheckInGateway {
    pub async fn find_all_by_person_uuid(&self,person_uuid: String,start: DateTime, end: DateTime) -> Vec<EvolutionCheckIn> {
        let filter = doc! {
            "personUuid": person_uuid,
            "createdAt": { "$gte": start, "$lte": end }
        };
        let cursor = self.collection.find(filter).await;

        if cursor.is_err() {
            log::error!("Error finding evolution check-ins by person uuid: {:?}", cursor.err().unwrap());
            return Vec::new();
        }

        let cursor = cursor.unwrap();


        Self::process_response(cursor).await
    }

    async fn process_response(mut cursor: Cursor<EvolutionCheckIn>) -> Vec<EvolutionCheckIn> {
        let mut evolution_checkin = Vec::new();
        while let Some(row) = cursor.try_next().await.unwrap_or(None) {
            evolution_checkin.push(row);
        }
        evolution_checkin
    }

    /// Account-deletion cascade: deletes every evolution check-in for this person.
    pub async fn delete_all_by_person(&self, person_uuid: &str) -> Result<(), BusinessError> {
        self.collection
            .delete_many(doc! { "personUuid": person_uuid })
            .await
            .map(|_| ())
            .map_err(|e| {
                log::error!("Error deleting evolution check-ins by person: {:?}", e);
                BusinessError::new("Failed to delete evolution check-ins".to_string())
            })
    }
}
