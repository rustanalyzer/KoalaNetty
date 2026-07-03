use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ErrorMessage {
    pub event: String,
    pub data: ServerError,
}

#[derive(Debug, Deserialize)]
pub struct ServerError {
    pub msg: String,

    #[serde(rename = "type")]
    pub error_type: Option<String>,
}