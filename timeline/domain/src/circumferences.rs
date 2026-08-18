use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Circumferences {

    #[serde(rename = "_id")]
    pub uuid: String,
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
    pub biceps: Biceps,
    #[serde(default)]
    pub thigh: Thighs,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Biceps {
    pub right: f64,
    pub left: f64,
}

impl Default for Biceps {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }
}

impl Biceps {
    pub fn new(right: f64, left: f64) -> Self {
        Self { right, left }
    }
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Thighs {
    pub right: f64,
    pub left: f64,
}

impl Default for Thighs {
    fn default() -> Self {
        Self {
            left: 0.0,
            right: 0.0,
        }
    }
}

impl Thighs {
    pub fn new(right: f64, left: f64) -> Self {
        Self { right, left }
    }
}

impl Circumferences {

    #[allow(clippy::too_many_arguments)]
    pub fn new(
        uuid:String,
        neck: f64,
        chest: f64,
        waist: f64,
        abdomen: f64,
        hip: f64,
        biceps: Biceps,
        thigh: Thighs,
    ) -> Circumferences {
        Self {
            uuid,
            neck,
            chest,
            waist,
            abdomen,
            hip,
            biceps,
            thigh,
        }
    }
}
