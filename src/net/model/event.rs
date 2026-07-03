use serde::Deserialize;
use serde_json::Value;

#[derive(Debug, Deserialize)]
pub struct EventMessage {
    pub event: String,
    pub data: Value,
}