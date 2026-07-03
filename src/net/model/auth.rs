use serde::{Deserialize, Serialize};

use super::bot::BotInfo;

#[derive(Debug, Deserialize)]
pub struct AuthMessage {
    pub cmd: String,
    pub data: AuthData,
}

#[derive(Debug, Deserialize)]
pub struct AuthData {
    pub appid: i32,
    pub bot: BotInfo,
}

#[derive(Debug, Serialize)]
pub struct AuthRequest<'a> {
    pub cmd: &'a str,
    pub token: &'a str,
}