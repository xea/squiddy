use super::config::AgentConfig;
use super::source::Source;
use super::target::Target;
use super::source::SourceBuilder;
use super::target::TargetBuilder;
use super::lang::SquiddyProgram;
use super::lang::ast::Parser;
use super::lang::compiler::Compiler;
use super::lang::token::Tokenizer;
use std::io::{Error, ErrorKind};
use bytes::Bytes;

pub struct Pipeline {
    source: Box<Source>,
    target: Box<Target>,
    program: SquiddyProgram
}

impl Pipeline {

    pub fn new(config: AgentConfig) -> Result<Pipeline, Error> {
        let program = Pipeline::compile_program(&config).unwrap();

        if let Ok(source) = SourceBuilder::build(config.source_type) {
            if let Ok(target) = TargetBuilder::build(config.target_type) {
                Ok(Pipeline {
                    source: source,
                    target: target,
                    program: program
                })
            } else {
                Err(Error::new(ErrorKind::Other, "No target definition found"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "No source definition found"))
        }
    }

    fn compile_program(_: &AgentConfig) -> Result<SquiddyProgram, Error> {
        Parser::parse(&Tokenizer::tokenize(Bytes::from("")))
        // TODO better error conversion
        .map_err(|_| Error::new(ErrorKind::Other, ""))
        .and_then(|p|
            Compiler::compile(p)
            // TODO better error conversion
            .map_err(|_| Error::new(ErrorKind::Other, "")))
    }

    pub fn run(&mut self) -> () {

        while self.source.has_more() {
            let item = self.source.next();

            for condition in &mut self.program.conditions {
                if condition.matcher.accept(&item) {
                    for action in &mut condition.actions {
                        if let Some(events) = action.perform() {
                            for event in events {
                                self.program.state.accept(event);
                            }
                        }
                    }
                }
            }

            self.target.accept(&self.program.state);
        }
    }
}
