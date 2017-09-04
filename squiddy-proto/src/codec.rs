use super::message::*;
use bytes::{Buf, BufMut, BytesMut, BigEndian, IntoBuf};
use tokio_io::codec::{Encoder, Decoder};

use std::io::{ Error, ErrorKind, Result };

pub struct SquiddyEncoder;
pub struct SquiddyDecoder;

/// Provides a list of possible opcodes describing individual protocol messages.
enum OpCode {
    ServerHello = 0x01,
    ClientHello = 0x02,
    Invalid
}

impl From<u16> for OpCode {
    fn from(opcode: u16) -> OpCode {
        match opcode {
            0x01 => OpCode::ServerHello,
            0x02 => OpCode::ClientHello,
            _ => OpCode::Invalid
        }
    }
}

impl Decoder for SquiddyDecoder {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Message>> {
        let mut local_buf = buf.split_to(2).into_buf();
        //
        if buf.remaining_mut() > 1 {
            match OpCode::from(local_buf.get_u16::<BigEndian>()) {
                OpCode::ServerHello => match SquiddyDecoder::decode_protocol_version(buf) {
                    // Everything is alright, let's return the protocol version
                    Some(version) => Ok(Some(Message::ServerHello(version))),
                    // Protocol version is not right, whole message is rejected
                    None => Err(Error::from(ErrorKind::InvalidData))
                },
                // We don't even know what message this was
                _ => Err(Error::from(ErrorKind::InvalidData))
            }
        } else {
            // We've requested reading from the buffer but there are no more bytes left :(
            Err(Error::from(ErrorKind::UnexpectedEof))
        }

    }
}

impl Encoder for SquiddyEncoder {
    type Item = Message;
    type Error = Error;

    fn encode(&mut self, msg: Message, buf: &mut BytesMut) -> Result<()> {
        match msg {
            Message::ServerHello(protocol_version) => SquiddyEncoder::encode_server_hello(buf, protocol_version),
            Message::ClientHello => SquiddyEncoder::encode_client_hello(buf)
        }

        // TODO implement error handling
        Ok(())
    }

}


impl SquiddyEncoder {

    fn encode_server_hello(buf: &mut BytesMut, version: ProtocolVersion) {
        buf.put_u16::<BigEndian>(OpCode::ServerHello as u16);
        buf.put_u8(version.0);
        buf.put_u8(version.1);
    }

    fn encode_client_hello(buf: &mut BytesMut) {
        buf.put_u16::<BigEndian>(OpCode::ClientHello as u16);
    }
}

impl SquiddyDecoder {

    fn decode_protocol_version(buf: &BytesMut) -> Option<ProtocolVersion> {
        None
    }
}
