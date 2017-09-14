use std::path::Path;
use super::source::SourceType;
use super::target::TargetType;

#[derive(Deserialize)]
pub struct AgentConfig {
    pub source_type: SourceType,
    pub target_type: TargetType
}

impl Default for AgentConfig {
    fn default() -> Self {
        AgentConfig {
            source_type: SourceType::Stdin,
            target_type: TargetType::Nil
        }
    }
}

impl<T> From<T> for AgentConfig where T: AsRef<Path> {

    fn from(_: T) -> AgentConfig {
        AgentConfig::default()
    }

}
