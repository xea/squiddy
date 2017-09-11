use native_tls::{TlsConnector, TlsConnectorBuilder, TlsStream, HandshakeError, Error as TLSError};
use std::io::{Read, Write, Error, ErrorKind};
use std::net::TcpStream;

pub struct TlsTarget {
    stream: TlsStream<TcpStream>
}

impl TlsTarget {

    pub fn connect() -> Result<TlsTarget, Error> {
        TcpStream::connect("localhost:9041").and_then(|tcp_stream|
            TlsConnector::builder()
            .and_then(|builder| builder.build())
            .map_err(|error| Error::new(ErrorKind::InvalidData, ""))
            .and_then(|connector|
                connector.connect("localhost", tcp_stream)
                .map_err(|error| Error::new(ErrorKind::InvalidData, ""))
            ).map(|tls_stream|
                TlsTarget {
                    stream: tls_stream
                }
            )
        )
    }
}
