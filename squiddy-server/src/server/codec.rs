use bytes::{ Buf, BigEndian, BytesMut, IntoBuf };
use tokio::net::TcpStream;
use tokio::prelude::{ Async, AsyncRead, Poll, Stream };
use tokio::io;
use super::ClientMessage;

pub struct ClientDecoder;

impl ClientDecoder {
    pub fn decode_packet(&mut self, mut buffer: BytesMut) -> Option<(usize, ClientMessage)> {
        const MNEMONIC_SIZE: usize = 2;

        if buffer.len() >= MNEMONIC_SIZE {
            let mnemonic = buffer.split_to(MNEMONIC_SIZE);

            match &mnemonic[..] {
                b"he" => self.decode_client_hello(&mut buffer).map(|(len, msg)| (len + MNEMONIC_SIZE, msg)),
                _ => None
            }
        } else {
            None
        }
    }

    fn decode_client_hello(&mut self, buffer: &mut BytesMut) -> Option<(usize, ClientMessage)> {
        self.decode_label(buffer).map(|(len, label)| (len, ClientMessage::ClientHello { name: label }))
    }

    /// Attempts to decode the read buffer as a label which is essentially just a size prefixed UTF-8 string.
    /// The maximum lenght of a label is 255 bytes (which may be shorter due to multi-byte encoding). 
    fn decode_label(&mut self, buffer: &mut BytesMut) -> Option<(usize, String)> {
        // Read the expected length of the label
        self.decode_u8(buffer)
        // Convert the length from u8 to usize because that's what every length function expects
        .map(|(read_bytes, label_len)| (read_bytes, label_len as usize))
        // TODO remove this line once testing is done, this only makes manual testing easier
        .map(|(read_bytes, label_len)| (read_bytes, label_len - '0' as usize))
        .and_then(|(read_bytes, label_len)| {
            if buffer.len() > label_len {
                String::from_utf8(buffer.split_to(label_len).as_ref().to_vec())
                    .ok()
                    .map(|label| (read_bytes + label_len, label))
            } else {
                None
            }
        })
    }

    fn decode_u8(&mut self, buffer: &mut BytesMut) -> Option<(usize, u8)> {
        const CHUNK_SIZE: usize = ::std::mem::size_of::<u8>();

        if buffer.len() >= CHUNK_SIZE {
            let raw_data = buffer.split_to(CHUNK_SIZE).into_buf().get_u8();

            Some((CHUNK_SIZE, raw_data))
        } else {
            None
        }
    }

    fn decode_u32(&mut self, mut buffer: BytesMut) -> Option<u32> {
        const CHUNK_SIZE: usize = ::std::mem::size_of::<u32>();

        if buffer.len() >= CHUNK_SIZE {
            let raw_data = buffer.split_to(CHUNK_SIZE).into_buf().get_u32::<BigEndian>();

            Some(raw_data)
        } else {
            None
        }
    }

}

#[cfg(tests)]
mod test {
    use super::ClientDecoder;
}

pub struct ClientCodec {
    stream: TcpStream,
    read_buffer: BytesMut,
//    write_buffer: BytesMut
}

impl ClientCodec {
    pub fn new(stream: TcpStream) -> Self {
        Self { 
            stream,
            read_buffer: BytesMut::new(),
 //           write_buffer: BytesMut::new()
        }
    }

    fn read_next_packet(&mut self) -> Poll<(), io::Error> {
        loop {
            // This initial value of 1024 is totally just an educated guess. It should be reviewed at some later point.
            self.read_buffer.reserve(1024);

            let n = try_ready!(self.stream.read_buf(&mut self.read_buffer));

            if n == 0 {
                return Ok(Async::Ready(()))
            }
        }
    }
}

impl Stream for ClientCodec {

    type Item = ClientMessage;
    type Error = io::Error;

    /// For each invocation of `poll` we try to read and retrieve a `ClientMessage`. 
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let sock_closed = self.read_next_packet()?.is_ready();

        let mut decoder = ClientDecoder;

        if let Some((len, packet)) = decoder.decode_packet(self.read_buffer.clone()) {
            // Decoding itself was done on a clone of the read buffer so we need to update the current
            // instance too, indicating that the reading a whole message has completed.
            let _ = self.read_buffer.split_to(len);
            return Ok(Async::Ready(Some(packet)))
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}