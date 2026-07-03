use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ResponseMessage {
    pub ok: bool,
    pub status: Option<Vec<i32>>,
}