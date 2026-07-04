use bytes::Bytes;
use serde::Serialize;
use tokio::io::AsyncWriteExt;

use crate::{net::{client::client::Client, model::MessageRequest}, packet::{netcmd::NetCmd, packet::Packet}};

impl Client {
    pub async fn send_packet(&mut self, packet: Packet) -> std::io::Result<()> {
        println!(">>> {}", packet);

        let bytes = packet.encode();
        self.stream.write_all(&bytes).await?;

        Ok(())
    }

    pub async fn send_message(
        &mut self,
        user_id: i32,
        text: &str,
    ) -> std::io::Result<()> {
        let req = MessageRequest {
            cmd: "message",
            userid: user_id,
            msg: text,
        };

        self.send_json(0, &req).await
    }

    pub async fn send_json<T: Serialize>(
        &mut self, 
        id: i32, 
        value: &T,
    ) -> std::io::Result<()> {
        let json = serde_json::to_string(value)?;
        println!("SEND: {}", json);

        let mut payload = serde_json::to_vec(value)?;
        self.crypt(&mut payload);

        let packet = Packet {
            id,
            cmd: NetCmd::Msg,
            payload: Bytes::from(payload),
        };

        self.send_packet(packet).await?;

        Ok(())
    }
}