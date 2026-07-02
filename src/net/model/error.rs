use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorMessage {
    pub event: String,
    pub data: ServerError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerError {
    pub msg: String,
    #[serde(rename="type")]
    pub error_type: Option<String>
}