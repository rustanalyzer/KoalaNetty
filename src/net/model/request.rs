use serde::Serialize;

#[derive(Serialize)]
pub struct FriendsRequest<'a> {
    pub cmd: &'a str,
    pub method: &'a str,
}

#[derive(Serialize)]
pub struct GiftRequest<'a> {
    pub cmd: &'a str,
    pub method: &'a str,
    pub userid: i32,
    pub itemid: i32,
}

#[derive(Serialize)]
pub struct NickChangeRequest {
    pub cmd: &'static str,
    pub method: &'static str,
    pub v: String,
}

#[derive(Serialize)]
pub struct NickColorChangeRequest {
    pub cmd: &'static str,
    pub method: &'static str,
    pub v: String,
}

#[derive(Serialize)]
pub struct PolChangeRequest {
    pub cmd: &'static str,
    pub method: &'static str,
    pub v: String,
}

#[derive(Serialize)]
pub struct TextInfoChangeRequest {
    pub cmd: &'static str,
    pub method: &'static str,
    pub v: String,
}

#[derive(Debug, Serialize)]
pub struct MessageRequest<'a> {
    pub cmd: &'a str,
    pub userid: i32,
    pub msg: &'a str,
}