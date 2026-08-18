use crate::http::json::person_info_json::PersonInfoJson;
use crate::http::json::user_json::UserJson;
use chrono::{NaiveDate, NaiveDateTime};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use crate::http::json::business_profile_json::BusinessProfileJson;
use crate::http::json::person_address_json::PersonAddressJson;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PersonJson {
    pub id: Option<i32>,
    pub uuid: Option<String>,
    pub firstname: Option<String>,
    pub surname: Option<String>,
    pub date_of_birth: Option<NaiveDate>,
    pub gender: Option<String>,
    pub object_key: Option<String>,
    pub avatar: Option<String>,
    pub cover: Option<String>,
    pub user: Option<UserJson>,
    pub person_info: Option<PersonInfoJson>,
    #[serde(default)]
    pub addresses: Vec<PersonAddressJson>,
    pub created_at: Option<NaiveDateTime>,
    pub updated_at: Option<NaiveDateTime>,
    #[serde(default)]
    pub business_profiles: Vec<BusinessProfileJson>,
}
