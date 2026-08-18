use uuid::Uuid;

pub fn uuid_to_string(uuid: Uuid) -> String {
    uuid.to_string()
}

pub fn string_to_uuid(uuid_str: &str) -> Uuid {
    Uuid::parse_str(uuid_str).unwrap_or_else(|_| Uuid::nil())
}
