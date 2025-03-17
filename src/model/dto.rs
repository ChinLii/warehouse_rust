use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct StorePayload {
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}

#[derive(Serialize)]
pub struct ErrorResponse {
    pub status: u16,
    pub message: String,
}

#[derive(Serialize)]
pub struct OkResponse {
    pub status: u16,
    pub message: String,
}
