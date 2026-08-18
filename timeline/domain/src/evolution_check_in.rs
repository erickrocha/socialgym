use crate::body_composition::BodyComposition;
use crate::circumferences::Circumferences;
use crate::enums::Visibility;
use mongodb::bson::DateTime;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EvolutionCheckIn {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub person_uuid: String,
    pub created_at: DateTime,
    pub note: Option<String>,
    pub visibility: Visibility,
    pub composition: Option<BodyComposition>,
    pub circumferences: Option<Circumferences>,
}

impl EvolutionCheckIn {
    pub fn new(
        uuid: String,
        person_uuid: String,
        created_at: DateTime,
        note: Option<String>,
        visibility: Visibility,
        composition: Option<BodyComposition>,
        circumferences: Option<Circumferences>,
    ) -> EvolutionCheckIn {
        Self {
            uuid,
            person_uuid,
            created_at,
            note,
            visibility,
            composition,
            circumferences,
        }
    }
}
