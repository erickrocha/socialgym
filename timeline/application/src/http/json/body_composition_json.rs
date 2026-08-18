use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct BodyCompositionJson {


    #[serde(skip_serializing_if = "Option::is_none")]
    pub uuid: Option<String>,

    #[serde(default)]
    pub weight: f64,
    #[serde(default)]
    pub body_fat_pct: f64,
    #[serde(default)]
    pub muscle_mass_pct: f64,
    #[serde(default)]
    pub visceral_fat: f64,
}

impl BodyCompositionJson {
    pub fn new(
        uuid: Option<String>,
        weight: f64,
        body_fat_pct: f64,
        muscle_mass_pct: f64,
        visceral_fat: f64,
    ) -> BodyCompositionJson {
        Self {
            uuid,
            weight,
            body_fat_pct,
            muscle_mass_pct,
            visceral_fat,
        }
    }
}
