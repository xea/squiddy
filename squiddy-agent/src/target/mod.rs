// TODO this module may be in need for a better name to avoid conflicts with cargo's targets
use std::io::{Error, ErrorKind};
use std::net::SocketAddr;
use self::dummy::DummyTarget;
use self::nil::NilTarget;
use self::tls::TlsTarget;
use super::state::State;

pub mod dummy;
pub mod nil;
pub mod tls;

#[derive(Deserialize)]
pub enum TargetType {
    Nil,
    DummyTarget,
    File,
    BinaryFile,
    Tcp(SocketAddr),
    Tls(SocketAddr, String),
    Stdout,
    BinaryStdout
}

pub trait Target {
    fn accept(&mut self, state: &State) -> bool;
}

pub struct TargetBuilder;

impl TargetBuilder {

    pub fn build(target_type: TargetType) -> Result<Box<Target>, Error> {
        match target_type {
            TargetType::Nil => Ok(Box::new(NilTarget)),
            TargetType::DummyTarget => Ok(Box::new(DummyTarget::new())),
            TargetType::Tls(server_address, server_name) => match TlsTarget::connect(server_address, &server_name) {
                Ok(target) => Ok(Box::new(target)),
                Err(err) => Err(err)
            },
            _ => Err(Error::new(ErrorKind::Other, "Not implemented"))
        }
    }
}
