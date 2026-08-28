use chrono::NaiveDateTime;
use crate::http::json::body_composition_json::BodyCompositionJson;
use crate::http::json::circumferences_json::CircumferencesJson;
use serde::{Deserialize, Serialize};
#[allow(unused_imports)]
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(example =  json!({
    "id": "evo-uuid-1",
    "personUuid": "person-uuid-1",
    "createdAt": "2026-05-23T16:47:28",
    "note": "Weekly update",
    "visibility": "Private",
    "composition": {
        "weight": 82.5,
        "bodyFatPct": 14.2,
        "muscleMassPct": 45.0,
        "visceralFat": 7
    },
    "circumferences": {
        "neck": 39.0,
        "chest": 104.0,
        "waist": 84.0,
        "abdomen": 86.0,
        "hip": 97.0,
        "bicepsRight": 37.0,
        "bicepsLeft": 36.5,
        "thighRight": 58.0,
        "thighLeft": 57.5
    }
}))]
pub struct EvolutionCheckInJson {

    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,
    pub person_uuid: String,
    pub created_at: NaiveDateTime,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub note: Option<String>,
    pub visibility: String,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub composition: Option<BodyCompositionJson>,
    #[serde(default)]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub circumferences: Option<CircumferencesJson>,
}

impl EvolutionCheckInJson {

    pub fn new(
        uuid: Option<String>,
        person_uuid: String,
        created_at: NaiveDateTime,
        note: Option<String>,
        visibility: String,
        composition: Option<BodyCompositionJson>,
        circumferences: Option<CircumferencesJson>,
    ) -> EvolutionCheckInJson {
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
