use super::config::AgentConfig;
use super::source::Source;
use super::target::Target;
use super::filter::condition::*;
use super::source::SourceBuilder;
use super::target::TargetBuilder;
use super::state::State;
use std::io::{Error, ErrorKind};

pub struct Pipeline {
    source: Box<Source>,
    target: Box<Target>,
    conditions: Vec<Condition>
}

impl Pipeline {

    pub fn new(config: AgentConfig) -> Result<Pipeline, Error> {
        let conditions = Pipeline::prepare_conditions(&config).unwrap();

        if let Ok(source) = SourceBuilder::build(config.source_type) {
            if let Ok(target) = TargetBuilder::build(config.target_type) {
                Ok(Pipeline {
                    source: source,
                    target: target,
                    conditions: conditions
                })
            } else {
                Err(Error::new(ErrorKind::Other, "No target definition found"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "No source definition found"))
        }
    }

    fn prepare_conditions(_: &AgentConfig) -> Result<Vec<Condition>, Error> {
        Ok(vec![])
    }

    pub fn run(&mut self) -> () {
        let mut state = State::default();

        while self.source.has_more() {
            let item = self.source.next();

            for condition in &mut self.conditions {
                if condition.matcher.accept(&item) {
                    for action in &mut condition.actions {
                        if let Some(events) = action.perform() {
                            for event in events {
                                state.accept(event);
                            }
                        }
                    }
                }
            }

            self.target.accept(&state);
        }
    }
}
