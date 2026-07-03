use crate::{net::{client::Client, model::{AuthMessage, AuthRequest, ErrorMessage, EventMessage, ResponseMessage}}, packet::packet::Packet};
use serde_json::Value;

impl Client {
    pub async fn handle_friend(&mut self, packet: Packet) -> std::io::Result<()> {
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

    pub async fn handle_msg(&mut self, packet: Packet) -> std::io::Result<()> {
        println!("{}", String::from_utf8_lossy(&packet.payload));

        let value: Value = serde_json::from_slice(&packet.payload)?;

        if value.get("cmd").and_then(Value::as_str) == Some("auth") {
            let auth: AuthMessage = serde_json::from_value(value)?;
            self.handle_auth(auth).await?;
        } else if value.get("event").and_then(Value::as_str) == Some("error") {
            let error: ErrorMessage = serde_json::from_value(value)?;
            self.handle_error(error).await?;
        } else if value.get("event").is_some() {
            let event: EventMessage = serde_json::from_value(value)?;
            self.handle_event(event).await?;
        } else if value.get("ok").is_some() {
            let response: ResponseMessage = serde_json::from_value(value)?;
            self.handle_response(response).await?;
        } else {
            println!("Неизвестное сообщение: {}", value);
        }

        Ok(())
    }

    pub async fn handle_msg_binary(&mut self, packet: Packet) -> std::io::Result<()> {
        todo!("");
        Ok(())
    }

    async fn handle_response(
        &mut self,
        response: ResponseMessage,
    ) -> std::io::Result<()> {
        println!("{response:#?}");

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
}