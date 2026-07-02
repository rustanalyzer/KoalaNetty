use bytes::Bytes;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream,
};

use crate::{net::model::models::ServerMsg, packet::{netcmd::NetCmd, packet::Packet, reader::PacketReader}};

pub struct Client {
    stream: TcpStream,
    reader: PacketReader,

    token: String,
    is_connected: bool,

    sign_buffer: Option<Bytes>
}

impl Client {
    pub async fn connect(addr: &str, token: String) -> std::io::Result<Self> {
        let stream = TcpStream::connect(addr).await?;

        Ok(Self {
            stream,
            reader: PacketReader::new(),
            token,
            is_connected: true,
            sign_buffer: None,
        })
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        let mut buf = [0u8; 4096];

        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                break;
            }

            self.reader.push(&buf[..n]);

            while let Some(packet) = self.reader.next_packet() {
                self.handle_packet(packet).await?;
            }
        }

        Ok(())
    }

    pub async fn send_packet(&mut self, addr: &str, packet: Packet) -> std::io::Result<()> {
        let bytes = packet.encode();
        self.stream.write_all(&bytes).await?;

        Ok(())
    }

    async fn handle_packet(&mut self, packet: Packet) -> std::io::Result<()> {
        match packet.cmd {
            NetCmd::Friend => self.handle_friend(packet).await?,
            NetCmd::Msg => self.handle_msg(packet).await?,
            NetCmd::MsgBinary => self.handle_msg_binary(packet).await?,
        }

        Ok(())
    }

    async fn handle_friend(&mut self, packet: Packet) -> std::io::Result<()> {
        let msg: ServerMsg = serde_json::from_slice(&packet.payload)?;

        match msg {
            ServerMsg::Auth(auth) => {
                todo!()
            }

            ServerMsg::Event(event) => {
                todo!()
            }

            ServerMsg::Error(error) => {
                todo!()
            }
        }

        Ok(())
    }

    async fn handle_msg(&mut self, packet: Packet) -> std::io::Result<()> {
        todo!("");
        Ok(())
    }

    async fn handle_msg_binary(&mut self, packet: Packet) -> std::io::Result<()> {
        todo!("");
        Ok(())
    }
}