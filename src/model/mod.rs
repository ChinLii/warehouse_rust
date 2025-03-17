pub mod dto;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Store {
    pub id: Uuid,
    pub name: Option<String>,
    pub address: Option<String>,
    pub phone: Option<String>,
}
