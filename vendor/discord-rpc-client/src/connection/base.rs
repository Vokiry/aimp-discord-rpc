use std::{
    io::{self, Write, Read, ErrorKind},
    marker::Sized,
    path::PathBuf,
    thread,
    time,
};
use byteorder::{ReadBytesExt, LittleEndian};
use serde_json::json;
use log::{debug, error};
use crate::{
    utils,
    models::message::{Message, OpCode},
    error::{Error, Result},
};


/// Wait for a non-blocking connection until it's complete.
macro_rules! try_until_done {
    [ $e:expr ] => {
        loop {
            match $e {
                Ok(v) => break v,
                Err(Error::IoError(ref err)) if err.kind() == ErrorKind::WouldBlock => (),
                Err(why) => return Err(why),
            }

            thread::sleep(time::Duration::from_millis(500));
        }
    }
}


pub trait Connection: Sized {
    type Socket: Write + Read;

    /// The internally stored socket connection.
    fn socket(&mut self) -> &mut Self::Socket;

    /// The base path were the socket is located.
    fn ipc_path() -> PathBuf;

    /// Establish a new connection to the server.
    fn connect() -> Result<Self>;

    /// The full socket path.
    fn socket_path(n: u8) -> PathBuf {
        Self::ipc_path().join(format!("discord-ipc-{}", n))
    }

    /// Perform a handshake on this socket connection.
    /// Will block until complete.
    fn handshake(&mut self, client_id: u64) -> Result<Message> {
        let hs = json![{
            "client_id": client_id.to_string(),
            "v": 1,
            "nonce": utils::nonce()
        }];

        let msg = Message::new(OpCode::Handshake, hs);
        try_until_done!(self.send(&msg));
        let msg = try_until_done!(self.recv());

        Ok(msg)
    }

    /// Ping the server and get a pong response.
    /// Will block until complete.
    fn ping(&mut self) -> Result<OpCode> {
        let message = Message::new(OpCode::Ping, json![{}]);
        try_until_done!(self.send(&message));
        let response = try_until_done!(self.recv());
        Ok(response.opcode)
    }

    /// Send a message to the server.
    fn send(&mut self, message: &Message) -> Result<()> {
        match message.encode() {
            Err(why) => error!("{:?}", why),
            Ok(bytes) => {
                self.socket().write_all(&bytes)?;
            }
        };
        debug!("-> {:?}", message);
        Ok(())
    }

    /// Receive a message from the server.
    fn recv(&mut self) -> Result<Message> {
        fn read_n<R: Read>(reader: &mut R, buf: &mut [u8]) -> io::Result<()> {
            let mut offset = 0;
            while offset < buf.len() {
                match reader.read(&mut buf[offset..]) {
                    Ok(0) => return Err(io::Error::new(io::ErrorKind::UnexpectedEof, "unexpected EOF")),
                    Ok(n) => offset += n,
                    Err(ref e) if e.kind() == ErrorKind::WouldBlock => {
                        thread::sleep(time::Duration::from_millis(10));
                    }
                    Err(e) => return Err(e),
                }
            }
            Ok(())
        }

        // Read 8-byte header: opcode (u32 LE) + length (u32 LE)
        let mut header = [0u8; 8];
        read_n(self.socket(), &mut header)?;

        let len = (&header[4..8]).read_u32::<LittleEndian>().unwrap() as usize;

        // Read exactly len bytes of payload
        let mut payload = vec![0u8; len];
        read_n(self.socket(), &mut payload)?;

        // Reconstruct full buffer for Message::decode
        let mut full = Vec::with_capacity(8 + len);
        full.extend_from_slice(&header);
        full.extend_from_slice(&payload);

        let message = Message::decode(&full)?;
        debug!("<- {:?}", message);

        Ok(message)
    }
}
