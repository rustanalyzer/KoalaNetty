use serde::{Deserialize, Serialize};

use super::bot::BotInfo;

#[derive(Debug, Deserialize, Clone)]
pub struct AuthMessage {
    pub cmd: String,
    pub data: AuthData,
}

#[derive(Debug, Deserialize, Clone)]
pub struct AuthData {
    pub appid: i32,
    pub bot: BotInfo,
}

#[derive(Debug, Serialize)]
pub struct AuthRequest<'a> {
    pub cmd: &'a str,
    pub token: &'a str,
}
