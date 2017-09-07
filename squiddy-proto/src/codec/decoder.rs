use super::super::message::*;
use super::*;
use bytes::{Buf, BytesMut, BigEndian, IntoBuf};
use tokio_io::codec::Decoder;

use std::io::{ Error, ErrorKind, Result };

/// `SquiddyDecoder` takes raw bytes sent by a remote peer and attempts to interpret them as
/// meaningful protocol messages.
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
                    Ok(version) => Ok(Some(Message::ServerHello(version))),
                    // Protocol version is not right, whole message is rejected
                    Err(error) => Err(error)
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

    /// Attempts to decode the bytes in the buffer as a squiddy protocol version. The procotol
    /// version should consist of two bytes, the first one being the major protocol version number
    /// and the second one the minor protocol version number.
    fn decode_protocol_version(buf: &mut BytesMut) -> Result<ProtocolVersion> {
        if !buf.is_empty() {
            let mut version_buf = buf.split_to(2).into_buf();

            Ok((version_buf.get_u8(), version_buf.get_u8()))
        } else {
            Err(Error::new(ErrorKind::UnexpectedEof, "Empty buffer"))
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
