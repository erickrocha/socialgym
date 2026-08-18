use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, Clone)]
#[serde(default, rename_all = "camelCase")]
pub struct User {
    pub id: Option<i32>,
    pub email: String,
    pub uuid: String,
    pub person_id: i32,
    pub name: String,
    pub person_uuid: String,
    pub person_object_key: String,
}

impl User {
    pub fn new(
        name: String,
        email: String,
        uuid: String,
        person_id: i32,
        person_uuid: String,
        person_object_key: String,
    ) -> Self {
        Self::update(None, name, email, uuid, person_id, person_uuid,person_object_key)
    }

    pub fn update(
        id: Option<i32>,
        name: String,
        email: String,
        uuid: String,
        person_id: i32,
        person_uuid: String,
        person_object_key: String,
    ) -> Self {
        Self {
            id,
            email,
            uuid,
            person_id,
            name,
            person_uuid,
            person_object_key
        }
    }
}
