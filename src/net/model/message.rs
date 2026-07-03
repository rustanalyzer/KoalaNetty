use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct MessageRequest<'a> {
    pub cmd: &'a str,
    pub userid: i32,
    pub msg: &'a str,
}