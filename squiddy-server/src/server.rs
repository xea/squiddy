use std::net::SocketAddr;
use super::agent::Agent;

pub struct Server {

}

impl Server {
    pub fn new() -> Self {
        Server {}
    }

    pub fn register_agent(remote_address: SocketAddr) -> Option<Agent> {
        None
    }
}
