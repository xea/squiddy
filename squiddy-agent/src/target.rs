use bytes::BytesMut;
use native_tls::{TlsConnector, TlsStream};
use std::error::Error as ErrorError;
use std::io::{Write, Error, ErrorKind};
use std::net::TcpStream;
use squiddy_proto::message::Message;
use squiddy_proto::codec::encoder::SquiddyEncoder;
use tokio_io::codec::Encoder;

pub trait Target {

    fn accept(&mut self, message: Message) -> bool;
}

pub struct TlsTarget {
    encoder: SquiddyEncoder,
    stream: TlsStream<TcpStream>
}

impl TlsTarget {

    pub fn connect() -> Result<TlsTarget, Error> {
        TcpStream::connect("localhost:9041").and_then(|tcp_stream|
            TlsConnector::builder()
            .and_then(|builder| builder.build())
            .map_err(|error| Error::new(ErrorKind::Other, error.description()))
            .and_then(|connector|
                connector.connect("localhost", tcp_stream)
                .map_err(|error| Error::new(ErrorKind::Other, error.description()))
            ).map(|tls_stream| TlsTarget { stream: tls_stream, encoder: SquiddyEncoder })
        )
    }
}

impl Target for TlsTarget {

    fn accept(&mut self, message: Message) -> bool {
        let mut write_buffer = BytesMut::with_capacity(128);

        self.encoder.encode(message, &mut write_buffer)
        .and_then(|_| self.stream.write_all(&mut write_buffer))
        .map(|_| true)
        .unwrap_or(false)
    }
}
