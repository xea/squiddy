use super::agent::Agent;

pub struct State {
    pub registered_agents: Vec<Agent>
}

impl State {
    pub fn new() -> Self {
        Self {
            registered_agents: vec![]
        }
    }
}
