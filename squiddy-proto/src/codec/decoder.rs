use super::super::message::*;
use super::*;
use bytes::{Buf, BytesMut, BigEndian, IntoBuf};
use tokio_io::codec::Decoder;

use std::io::{ Error, ErrorKind, Result };

pub struct SquiddyDecoder;

impl Decoder for SquiddyDecoder {
    type Item = Message;
    type Error = Error;

    fn decode(&mut self, buf: &mut BytesMut) -> Result<Option<Message>> {
        if buf.is_empty() {
            return Err(Error::new(ErrorKind::InvalidData, "Received empty message"))
        }

        let mut local_buf = buf.split_to(OPCODE_LENGTH).into_buf();
        //
        if buf.len() > 1 {
            let opcode = local_buf.get_u16::<BigEndian>();

            match OpCode::from(opcode) {
                OpCode::ServerHello => match SquiddyDecoder::decode_protocol_version(buf) {
                    // Everything is alright, let's return the protocol version
                    Some(version) => Ok(Some(Message::ServerHello(version))),
                    // Protocol version is not right, whole message is rejected
                    None => Err(Error::from(ErrorKind::InvalidData))
                },
                OpCode::ClientHello => match SquiddyDecoder::decode_client_name(buf) {
                    Ok(client_name) => Ok(Some(Message::ClientHello(client_name))),
                    Err(error) => Err(error)
                },
                // We don't even know what message this was
                OpCode::Invalid => Err(Error::new(ErrorKind::InvalidData, format!("Unknown opcode: {}", opcode)))
            }
        } else {
            // We've requested reading from the buffer but there are no more bytes left :(
            Err(Error::from(ErrorKind::UnexpectedEof))
        }
    }
}

impl SquiddyDecoder {

    // TODO refactor return type to generate meaningful error messages instead of None
    fn decode_protocol_version(buf: &mut BytesMut) -> Option<ProtocolVersion> {
        if !buf.is_empty() {
            let mut version_buf = buf.split_to(2).into_buf();

            Some((version_buf.get_u8(), version_buf.get_u8()))
        } else {
            None
        }
     }

     fn decode_client_name(buf: &mut BytesMut) -> Result<ClientName> {
         if !buf.is_empty() {
            let name_len = buf.split_to(1).into_buf().get_u8() as usize;

            if name_len <= MAX_CLIENT_NAME_LENGTH as usize && buf.len() >= name_len {
                let name_bytes = buf.split_to(name_len);

                String::from_utf8(name_bytes.as_ref().to_vec()).map_err(|e| Error::new(ErrorKind::Other, e))
            } else {
                Err(Error::new(ErrorKind::InvalidData, format!("Client name length exceeds maximum value: {} MAX: {}", name_len, MAX_CLIENT_NAME_LENGTH)))
            }
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Empty buffer"))
        }
     }
}
