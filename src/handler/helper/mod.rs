pub fn validate_uuid(uuid_str: &str) -> bool {
    return uuid::Uuid::parse_str(uuid_str).is_ok()
}