use std::time::Instant;
use std::net::SocketAddr;

/// `Agent` objects represent registered clients connecting to this server instance. Each registered
/// `Agent` must have a unique `agent_id` (generated by the server) and an `app` type associated with it.
pub struct Agent {
    /// The unique id of this agent
    pub agent_id: String,
    /// The remote address of this agent
    pub address: SocketAddr,
    pub app: String,
    /// The time this agent was registered
    pub registration_time: Instant
}

impl Agent {
    pub fn new(address: SocketAddr) -> Self {
        Agent {
            address,
            agent_id: String::from("new_agent"),
            app: String::from("test_app"),
            registration_time: Instant::now()
        }
    }
}
