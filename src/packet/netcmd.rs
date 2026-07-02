#[repr(u8)]
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum NetCmd {
    Friend = 1,
    Msg = 2,
    MsgBinary = 3,
}

impl TryFrom<u8> for NetCmd {
    type Error = ();
    
    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(NetCmd::Friend),
            2 => Ok(NetCmd::Msg),
            3 => Ok(NetCmd::MsgBinary),

            _ => Err(())
        }
    }
}