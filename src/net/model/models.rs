use serde::Deserialize;

use crate::net::model::error::ErrorMessage;

#[derive(Debug, Deserialize)]
#[serde(untagged)]
pub enum ServerMsg {
    Auth(AuthMessage),
    Event(EventMessage),
    Error(ErrorMessage)
}

#[derive(Debug, Deserialize)]
pub struct AuthMessage {
    pub cmd: String,
    pub data: AuthData
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub bot: BotInfo
}

#[derive(Debug, Deserialize)]
pub struct BotInfo {
    pub id: i32,
    pub owner: i32
}

#[derive(Debug, Deserialize)]
pub struct EventMessage {
    pub event: String,
    pub data: serde_json::Value,
}

