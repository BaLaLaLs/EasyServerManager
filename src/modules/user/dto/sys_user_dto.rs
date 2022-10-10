
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct AuthPayload {
    pub username: String,
    pub password: String
}