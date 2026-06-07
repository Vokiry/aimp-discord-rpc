use std::{io::{self, Write}, mem::size_of};
use byteorder::{WriteBytesExt, ReadBytesExt, LittleEndian};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use serde::Serialize;
use crate::{Error, Result};

#[repr(u32)]
#[derive(Debug, Copy, Clone, PartialEq, FromPrimitive)]
pub enum OpCode {
    Handshake,
    Frame,
    Close,
    Ping,
    Pong,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub opcode: OpCode,
    pub payload: String,
}

impl Message {
    pub fn new<T>(opcode: OpCode, payload: T) -> Self
        where T: Serialize
    {
        let payload = serde_json::to_string(&payload).unwrap();
        log::debug!("RPC send: {:?} {}", opcode, payload);
        Self { opcode, payload }
    }

    pub fn encode(&self) -> Result<Vec<u8>> {
        let payload_len = self.payload.len();
        let mut bytes: Vec<u8> = Vec::with_capacity(2 * size_of::<u32>() + payload_len);

        bytes.write_u32::<LittleEndian>(self.opcode as u32)?;
        bytes.write_u32::<LittleEndian>(payload_len as u32)?;
        bytes.write_all(self.payload.as_bytes())?;

        Ok(bytes)
    }

    pub fn decode(mut bytes: &[u8]) -> Result<Self> {
        let opcode = OpCode::from_u32(bytes.read_u32::<LittleEndian>()?).ok_or(Error::Conversion)?;
        let len = bytes.read_u32::<LittleEndian>()? as usize;
        let actual = bytes.len().min(len);
        let payload = String::from_utf8(bytes[..actual].to_vec())
            .map_err(|_| Error::IoError(io::Error::new(io::ErrorKind::InvalidData, "invalid UTF-8")))?;

        Ok(Self { opcode, payload })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Something {
        empty: bool
    }

    #[test]
    fn test_encoder() {
        let msg = Message::new(OpCode::Frame, Something { empty: true });
        let encoded = msg.encode().unwrap();
        let decoded = Message::decode(&encoded).unwrap();
        assert_eq!(msg, decoded);
    }

    #[test]
    fn test_opcode() {
        assert_eq!(OpCode::from_u32(0), Some(OpCode::Handshake));
        assert_eq!(OpCode::from_u32(4), Some(OpCode::Pong));
        assert_eq!(OpCode::from_u32(5), None);
    }
}
