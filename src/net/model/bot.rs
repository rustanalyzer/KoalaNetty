use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BotInfo {
    pub id: i32,
    pub app: i32,
    pub pol: String,
    pub owner: i32,
    pub nick: String,
    pub nick_color: Vec<String>,
}