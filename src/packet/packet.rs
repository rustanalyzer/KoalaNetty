use std::fmt::{Display, Formatter};

use bytes::{BufMut, Bytes, BytesMut};

use crate::packet::netcmd::NetCmd;

pub const HEADER_SIZE: usize = 14;

#[derive(Debug)]
pub struct Packet {
    pub id: i32,
    pub cmd: NetCmd,
    pub payload: Bytes
}

impl Packet {
    pub fn encode(&self) -> BytesMut {
        let mut buf = BytesMut::with_capacity(
            HEADER_SIZE + self.payload.len()
        );

        buf.put_u32_le(self.payload.len() as u32);
        buf.put_i32_le(self.id);
        buf.put_u8(self.cmd as u8);
        buf.put_u8(0);
        buf.put_i32_le(0);
        buf.extend_from_slice(&self.payload);

        buf
    }
}

impl Display for Packet {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Packet {{ id: {}, cmd: {:?}, payload: {} bytes }}",
            self.id,
            self.cmd,
            self.payload.len()
        )
    }
}