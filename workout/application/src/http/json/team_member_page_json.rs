use crate::http::json::business_profile_json::BusinessProfileJson;
use crate::http::json::person_json::PersonJson;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

/// Both sides of the team membership in one page. Which half is filled depends on who is
/// asking: a request carrying an active business profile gets the person-side lists, a request
/// carrying only a person gets the business-profile-side ones.
#[derive(Serialize, Deserialize, Debug, Clone, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct TeamMemberPageJson {
    /// Persons who accepted this business profile's invitation.
    pub members: Vec<PersonJson>,
    /// Persons this business profile invited and who have not answered yet.
    pub sent_requests: Vec<PersonJson>,
    /// Business profiles whose invitation this person accepted.
    pub teams: Vec<BusinessProfileJson>,
    /// Business profiles that invited this person and are waiting on an answer.
    pub received_requests: Vec<BusinessProfileJson>,
}
