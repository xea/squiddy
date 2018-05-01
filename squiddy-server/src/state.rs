use super::agent::Agent;
use std::sync::{ Arc, RwLock };

pub struct State {
    pub registered_agents: Vec<Agent>,
}

impl State {
    pub fn new() -> Self {
        Self {
            registered_agents: vec![]
        }
    }

    pub fn register_agent(&mut self, agent: Agent) -> Arc<RwLock<AgentState>> {
        self.registered_agents.push(agent);

        Arc::new(RwLock::new(AgentState {}))
    }
}

pub struct AgentState {

}

impl AgentState {
    pub fn disconnect(&self) {
        
    }
}