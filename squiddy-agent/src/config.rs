use std::path::Path;

#[derive(Deserialize)]
pub struct AgentConfig {

}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
        }
    }
}

impl<T> From<T> for AgentConfig where T: AsRef<Path> {

    fn from(source: T) -> AgentConfig {
        AgentConfig::default()
    }

}
