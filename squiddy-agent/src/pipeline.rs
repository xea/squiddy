use super::config::AgentConfig;
use super::source::Source;
use super::target::Target;
use super::filter::condition::Condition;
use super::source::SourceBuilder;
use super::target::TargetBuilder;
use squiddy_proto::message::Message;
use std::io::{Error, ErrorKind};

pub struct Pipeline {
    source: Box<Source>,
    target: Box<Target>,
    conditions: Vec<Condition>
}

impl Pipeline {

    pub fn new(config: AgentConfig) -> Result<Pipeline, Error> {
        if let Ok(source) = SourceBuilder::build(config.source_type) {
            if let Ok(target) = TargetBuilder::build(config.target_type) {
                Ok(Pipeline {
                    source: source,
                    target: target,
                    conditions: vec![]
                })
            } else {
                Err(Error::new(ErrorKind::Other, "No target definition found"))
            }
        } else {
            Err(Error::new(ErrorKind::Other, "No source definition found"))
        }
    }

    pub fn run(&mut self) -> () {
        while self.source.has_more() {
            let item = self.source.next();

            let message = Message::ClientHello(String::from(""));

            self.target.accept(message);
        }

    /*
    let mut conditions = vec![
        Condition { matcher: Box::new(AnyMatcher { children:
            vec![
                Box::new(LengthMatcher { threshold: 13 })
            ] }), actions: vec![ Box::new(IncrementCounter { name: String::from("asdf") }) ] }
    ];

    let mut bytes = BytesMut::with_capacity(1024);

    for line in input_lines {
        bytes.put_slice(line.as_bytes());

        println!("bytes len: {}", bytes.len());

        for condition in &mut conditions {
            println!("Yay condition");
            if condition.matcher.accept(&mut bytes) {
                for action in &mut condition.actions {
                    action.perform();
                }
            } else {
                println!("Nope ..|..");
            }
        }

        bytes.clear();
    }
    */

    }
}
