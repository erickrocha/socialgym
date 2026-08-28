use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use serde::de::{self, Deserializer};
fn deserialize_naive_date<'de, D>(des: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let s = String::deserialize(des)?;
    NaiveDate::parse_from_str(&s, "%Y-%m-%d").map_err(de::Error::custom)
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct SignUpJson {
    pub firstname: String,
    pub surname: String,
    #[serde(deserialize_with = "deserialize_naive_date")]
    pub date_of_birth: NaiveDate,
    pub gender: String,
    pub email: String,
    pub password: String,
    pub enabled: Option<bool>,
    pub first_login: Option<bool>,
    pub terms_version: String,
    pub privacy_version: String,
    pub terms_accepted: bool,
    pub privacy_accepted: bool,
}
