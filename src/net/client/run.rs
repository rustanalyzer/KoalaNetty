use bytes::Bytes;
use tokio::io::AsyncReadExt;

use crate::{net::client::client::Client, packet::{netcmd::NetCmd, packet::Packet}};

impl Client {
    pub async fn poll(&mut self) -> std::io::Result<()> {
        let mut buf = [0u8; 4096];

        let n = self.stream.read(&mut buf).await?;

        if n == 0 {
            return Ok(());
        }

        self.reader.push(&buf[..n]);

        while let Some(mut packet) = self.reader.next_packet() {
            if packet.cmd != NetCmd::Friend {
                let mut payload = packet.payload.to_vec();

                self.crypt(&mut payload);

                packet.payload = Bytes::from(payload);
            }

            self.handle_packet(packet).await?;
        }

        Ok(())
    }

    pub async fn start(&mut self) -> std::io::Result<()> {
        self.send_packet(Packet {
            id: 0,
            cmd: NetCmd::Friend,
            payload: Bytes::new(),
        })
        .await?;

        println!("FRIEND пакет отправлен");

        while self.bot_info.is_none() {
            self.poll().await?;
        }

        Ok(())
    }

    pub async fn run(&mut self) -> std::io::Result<()> {
        loop {
            self.poll().await?;
        }
    }

    async fn handle_packet(&mut self, packet: Packet) -> std::io::Result<()> {
        println!("<<< {}", packet);
        match packet.cmd {
            NetCmd::Friend => self.handle_friend(packet).await?,
            NetCmd::Msg => self.handle_msg(packet).await?,
            NetCmd::MsgBinary => self.handle_msg_binary(packet).await?,
        }

        Ok(())
    }
}