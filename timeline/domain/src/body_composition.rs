use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BodyComposition {
    #[serde(rename = "_id")]
    pub uuid: String,
    pub weight: f64,
    pub body_fat_pct: f64,
    pub muscle_mass_pct: f64,
    pub visceral_fat: f64,
}

impl BodyComposition {
    pub fn new(
        uuid: String,
        weight: f64,
        body_fat_pct: f64,
        muscle_mass_pct: f64,
        visceral_fat: f64,
    ) -> BodyComposition {
        Self {
            uuid,
            weight,
            body_fat_pct,
            muscle_mass_pct,
            visceral_fat,
        }
    }
}
