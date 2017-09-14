use bytes::BytesMut;
use native_tls::{TlsConnector, TlsStream};
use std::error::Error as ErrorError;
use std::io::{Write, Error, ErrorKind};
use std::net::{SocketAddr, TcpStream};
use squiddy_proto::message::Message;
use squiddy_proto::codec::encoder::SquiddyEncoder;
use tokio_io::codec::Encoder;
use super::Target;
use super::super::state::State;

pub struct TlsTarget {
    encoder: SquiddyEncoder,
    stream: TlsStream<TcpStream>
}

impl TlsTarget {

    pub fn connect(server_address: SocketAddr, server_name: &str) -> Result<TlsTarget, Error> {
        TcpStream::connect(server_address).and_then(|tcp_stream|
            TlsConnector::builder()
            .and_then(|builder| builder.build())
            .map_err(|error| Error::new(ErrorKind::Other, error.description()))
            .and_then(|connector|
                connector.connect(server_name, tcp_stream)
                .map_err(|error| Error::new(ErrorKind::Other, error.description()))
            ).map(|tls_stream| TlsTarget { stream: tls_stream, encoder: SquiddyEncoder })
        )
    }

    fn translate(&mut self, state: &State) -> Option<Message> {
        None
    }
}

impl Target for TlsTarget {

    fn accept(&mut self, state: &State) -> bool {
        let mut write_buffer = BytesMut::with_capacity(128);

        self.translate(state).map(|message|
            self.encoder.encode(message, &mut write_buffer)
            .and_then(|_| self.stream.write_all(&write_buffer))
            .map(|_| true)
            .unwrap_or(false)
        ).unwrap_or(true)
    }
}
