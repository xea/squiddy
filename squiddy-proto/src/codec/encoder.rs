use super::super::message::*;
use super::{OpCode, MAX_CLIENT_NAME_LENGTH };
use bytes::{BufMut, BytesMut, BigEndian};
use tokio_io::codec::Encoder;

use std::io::{ Error, Result };

pub struct SquiddyEncoder;

impl Encoder for SquiddyEncoder {
    type Item = Message;
    type Error = Error;

    fn encode(&mut self, msg: Message, buf: &mut BytesMut) -> Result<()> {
        match msg {
            Message::ServerHello(protocol_version) => SquiddyEncoder::encode_server_hello(buf, protocol_version),
            Message::ClientHello(client_name) => SquiddyEncoder::encode_client_hello(buf, client_name)
        }

        Ok(())
    }

}

impl SquiddyEncoder {

    // TODO add error handling for invalid protocol version numbers
    fn encode_server_hello(buf: &mut BytesMut, version: ProtocolVersion) {
        buf.put_u16::<BigEndian>(OpCode::ServerHello as u16);
        buf.put_u8(version.0);
        buf.put_u8(version.1);
    }

    // TODO add error handling for invalid and/or too long client names
    fn encode_client_hello(buf: &mut BytesMut, client_name: ClientName) {
        buf.put_u16::<BigEndian>(OpCode::ClientHello as u16);

        if client_name.len() > MAX_CLIENT_NAME_LENGTH as usize {
            let mut new_name = client_name.clone();
            new_name.truncate(MAX_CLIENT_NAME_LENGTH as usize);

            buf.put_u8(new_name.len() as u8);
            buf.extend(new_name.as_bytes());
        } else {
            buf.put_u8(client_name.len() as u8);
            buf.extend(client_name.as_bytes());
        }
    }
}
