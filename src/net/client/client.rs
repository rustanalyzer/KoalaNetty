use bytes::Bytes;
use tokio::{
    net::TcpStream,
};

use crate::{net::model::*, packet::reader::PacketReader};

pub struct Client {
    pub(crate) stream: TcpStream,
    pub(crate) reader: PacketReader,

    pub(crate) token: String,
    pub(crate) is_connected: bool,

    pub(crate) sign_buffer: Option<Bytes>,
    pub(crate) bot_info: Option<BotInfo>,
}

impl Client {
    pub async fn connect(addr: &str, token: String) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;

        Ok(Self {
            stream,
            reader: PacketReader::new(),
            token,
            is_connected: false,
            sign_buffer: None,
            bot_info: None,
        })
    }
}