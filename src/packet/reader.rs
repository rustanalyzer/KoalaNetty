use bytes::BytesMut;
use bytes::Buf;

use crate::packet::{netcmd::NetCmd, packet::{HEADER_SIZE, Packet}};

pub struct PacketReader {
    buffer: BytesMut
}

impl PacketReader {
    pub fn new() -> Self {
        Self { buffer: BytesMut::new() }
    }

    pub fn push(&mut self, data: &[u8]) {
        self.buffer.extend_from_slice(data);
    }

    pub fn next_packet(&mut self) -> Option<Packet> {
        if self.buffer.len() < HEADER_SIZE {
            return None;
        }

        let len = u32::from_le_bytes([
            self.buffer[0],
            self.buffer[1],
            self.buffer[2],
            self.buffer[3],
        ]) as usize;

        if self.buffer.len() < HEADER_SIZE + len {
            return None;
        }

        let mut header = self.buffer.split_to(HEADER_SIZE);

        let _len = header.get_u32_le();
        let id = header.get_i32_le();
        let cmd = NetCmd::try_from(header.get_u8()).ok()?;

        header.advance(5);

        let payload = self.buffer.split_to(len).freeze();

        Some(Packet {
            id,
            cmd,
            payload,
        })
    }
}