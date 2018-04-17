use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use std::time::Instant;
use super::agent::Agent;
use super::state::*;
use std::sync::{Arc, RwLock};

pub struct Server {
    state: Arc<RwLock<State>>
}

impl Server {
    pub fn new(state: Arc<RwLock<State>>) -> Self {
        Server {
            state: state
        }
    }

    pub fn start(&mut self) {
        if let Ok(mut state) = self.state.write() {
            let agents: &mut Vec<Agent> = &mut (*state).registered_agents;

            let a = Ipv4Addr::new(127, 0, 0, 1);
            let b = IpAddr::V4(a);
            let c = SocketAddr::new(b, 8080);
            agents.push(Agent::new(c));
        }
    }
}
