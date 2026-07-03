use bytes::Bytes;
use serde::Serialize;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt}, net::TcpStream,
};

use crate::{net::model::{error::ErrorMessage, models::{AuthMessage, BotInfo, EventMessage, ServerMsg}}, packet::{netcmd::NetCmd, packet::Packet, reader::PacketReader}};

pub struct Client {
    stream: TcpStream,
    reader: PacketReader,

    token: String,
    is_connected: bool,

    sign_buffer: Option<Bytes>,
    bot_info: Option<BotInfo>,
}

#[derive(Serialize)]
struct AuthRequest<'a> {
    cmd: &'a str,
    token: &'a str,
}

#[derive(Serialize)]
struct MessageRequest<'a> {
    cmd: &'a str,
    userid: i32,
    msg: &'a str,
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

    pub async fn run(&mut self) -> std::io::Result<()> {
        self.send_packet(Packet {
            id: 0,
            cmd: NetCmd::Friend,
            payload: Bytes::new(),
        }).await?;

        println!("FRIEND пакет отправлен");
        
        let mut buf = [0u8; 4096];

        loop {
            let n = self.stream.read(&mut buf).await?;

            if n == 0 {
                break;
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
        }

        Ok(())
    }

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

    async fn handle_packet(&mut self, packet: Packet) -> std::io::Result<()> {
        println!("<<< {}", packet);
        match packet.cmd {
            NetCmd::Friend => self.handle_friend(packet).await?,
            NetCmd::Msg => self.handle_msg(packet).await?,
            NetCmd::MsgBinary => self.handle_msg_binary(packet).await?,
        }

        Ok(())
    }

    async fn handle_friend(&mut self, packet: Packet) -> std::io::Result<()> {
        self.sign_buffer = Some(packet.payload);

        print!("Получен sign_buffer");

        let token = self.token.clone();
        let auth = AuthRequest {
            cmd: "auth",
            token: &token,
        };

        self.send_json(0, &auth).await?;

        Ok(())
    }

    async fn handle_msg(&mut self, packet: Packet) -> std::io::Result<()> {
        println!("{}", String::from_utf8_lossy(&packet.payload));
        let msg: ServerMsg = serde_json::from_slice(&packet.payload)?;

        match msg {
            ServerMsg::Auth(auth) => {
                self.handle_auth(auth).await?;
            }

            ServerMsg::Event(event) => {
                self.handle_event(event).await?;
            }

            ServerMsg::Error(error) => {
                self.handle_error(error).await?;
            }
        }

        Ok(())
    }

    fn crypt(&self, data: &mut [u8]) {
        let Some(sign) = &self.sign_buffer else {
            return;
        };

        let len = sign.len();

        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= sign[i % len];
        }
    }

    async fn handle_msg_binary(&mut self, packet: Packet) -> std::io::Result<()> {
        todo!("");
        Ok(())
    }

    async fn handle_auth(&mut self, auth: AuthMessage) -> std::io::Result<()> {
        self.bot_info = Some(auth.data.bot);

        self.is_connected = true;

        println!("Авторизация успешна");

        let owner = self.bot_info.as_ref().unwrap().owner;

        self.send_message(owner, "Тест 🦀").await?;

        Ok(())
    }

    async fn handle_event(&mut self, event: EventMessage) -> std::io::Result<()> {
        match event.event.as_str() {
            "message" => print!("Получено сообщение!"),
            "file" => print!("Получен файл!"),
            _ => print!("Неизвестное событие: {}", event.event),
        }

        Ok(())
    }

    async fn handle_error(&mut self, error: ErrorMessage) -> std::io::Result<()> {
        eprintln!(
            "[{}] {}",
            error.data.error_type.as_deref().unwrap_or("unknown"),
            error.data.msg
        );

        Ok(())
    }

    pub async fn send_json<T: Serialize>(
        &mut self, 
        id: i32, 
        value: &T,
    ) -> std::io::Result<()> {
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