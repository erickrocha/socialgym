use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct CircumferencesJson {

    pub uuid: Option<String>,
    #[serde(default)]
    pub neck: f64,
    #[serde(default)]
    pub chest: f64,
    #[serde(default)]
    pub waist: f64,
    #[serde(default)]
    pub abdomen: f64,
    #[serde(default)]
    pub hip: f64,
    #[serde(default)]
    pub biceps_right: f64,
    #[serde(default)]
    pub biceps_left: f64,
    #[serde(default)]
    pub thigh_right: f64,
    #[serde(default)]
    pub thigh_left: f64,
}


impl CircumferencesJson {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uuid: Option<String>,
        neck: f64,
        chest: f64,
        waist: f64,
        abdomen: f64,
        hip: f64,
        biceps_right: f64,
        biceps_left: f64,
        thigh_right: f64,
        thigh_left: f64,
    ) -> CircumferencesJson {
        Self {
            uuid,
            neck,
            chest,
            waist,
            abdomen,
            hip,
            biceps_right,
            biceps_left,
            thigh_right,
            thigh_left,
        }
    }
}
