use bytes::{ Buf, BigEndian, BytesMut, IntoBuf };
use tokio::net::TcpStream;
use tokio::prelude::{ Async, AsyncRead, Poll, Stream };
use tokio::io;
use super::ClientMessage;

pub struct ClientDecoder;

impl ClientDecoder {
    pub fn decode_packet(&mut self, mut buffer: BytesMut) -> Option<(usize, ClientMessage)> {
        const MNEMONIC_SIZE: usize = 2;

        let mut read_bytes: usize = 0;

        if buffer.len() >= MNEMONIC_SIZE {
            let mnemonic = buffer.split_to(MNEMONIC_SIZE);
            read_bytes += MNEMONIC_SIZE;

            match &mnemonic[..] {
                b"he" => (),
                _ => ()
            }
        }  

        None
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

            println!("Got value from try_ready: {}", n);

            if n == 0 {
                return Ok(Async::Ready(()))
            }
        }
    }

    fn read_label(buffer: &mut BytesMut) -> Async<Option<String>> {
        const LABEL_LEN_SIZE: usize = 1;

        if buffer.len() >= LABEL_LEN_SIZE {
            let label_len = buffer.split_to(LABEL_LEN_SIZE)[0] as usize;

            if buffer.len() >= label_len {
                let raw_label = buffer.split_to(label_len);

                Async::Ready(Some(String::from_utf8(raw_label.as_ref().to_vec()).unwrap_or(String::from("[Label error]"))))
            } else {
                Async::NotReady
            }
        } else {
            Async::NotReady
        }
    }
}

impl Stream for ClientCodec {

    type Item = ClientMessage;
    type Error = io::Error;

    /// For each invocation of `poll` we try to read and retrieve a `ClientMessage`. 
    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let sock_closed = self.read_next_packet()?.is_ready();
        let mut read_bytes: usize = 0;

        println!("Read buffer: {:?}", self.read_buffer);

        const MSG_MNEMONIC_SIZE: usize = 2;
        const STR_LENGTH_SIZE: usize = 1;

        let mut buffer = self.read_buffer.clone();

        if buffer.len() >= MSG_MNEMONIC_SIZE {
            let msg_code = buffer.split_to(MSG_MNEMONIC_SIZE);

            read_bytes += MSG_MNEMONIC_SIZE;

            println!("Message code: #{:?}", msg_code);

            let result = match &msg_code[..] {
                /*
                b"la" => ClientCodec::read_label(&mut buffer).map(|label| label.map(|name| ClientMessage::ClientHello { name: name })),
                b"he" => {
                    if buffer.len() >= STR_LENGTH_SIZE {
                        let msg_size = buffer.split_to(STR_LENGTH_SIZE);
                        read_bytes += STR_LENGTH_SIZE;

                        println!("Message size: {}", msg_size[0]);
                        let offset = msg_size[0] as usize - '0' as usize;

                        if buffer.len() >= offset {
                            let raw_name = buffer.split_to(offset);
                            read_bytes += offset;

                            println!("Found name: {:?}", raw_name);

                            Async::Ready(Some(ClientMessage::ClientHello { name: String::from_utf8(raw_name.as_ref().to_vec()).unwrap_or(String::from("Name error"))  }))
                        } else {
                            Async::NotReady
                        }
                    } else {
                        Async::NotReady
                    }
                },
                */
                _ => Async::Ready(None)
            };

            match result {
                Async::Ready(_) => { self.read_buffer.split_to(read_bytes); },
                _ => ()
            }

            println!("Inner result: {:?}", result);

            return Ok(result)
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}
