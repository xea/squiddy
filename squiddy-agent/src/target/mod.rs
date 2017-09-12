// TODO this module may be in need for a better name to avoid conflicts with cargo's targets
use squiddy_proto::message::Message;
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use self::memory::MemoryTarget;
use self::tls::TlsTarget;

pub mod memory;
pub mod tls;

#[derive(Deserialize)]
pub enum TargetType {
    Nil,
    File,
    BinaryFile,
    Tcp(SocketAddr),
    Tls(SocketAddr, String),
    Memory,
    Stdout,
    BinaryStdout
}

pub trait Target {
    fn accept(&mut self, message: Message) -> bool;
}

pub struct TargetBuilder;

impl TargetBuilder {

    pub fn build(target_type: TargetType) -> Result<Box<Target>, Error> {
        match target_type {
            TargetType::Memory => Ok(Box::new(MemoryTarget::default())),
            TargetType::Tls(server_address, server_name) => match TlsTarget::connect(server_address, &server_name) {
                Ok(target) => Ok(Box::new(target)),
                Err(err) => Err(err)
            },
            _ => Err(Error::new(ErrorKind::Other, "Not implemented"))
        }
    }
}
