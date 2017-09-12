extern crate bytes;
extern crate native_tls;
extern crate squiddy_proto;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate tokio_io;

use config::AgentConfig;
use filter::action::*;
use filter::condition::*;
use filter::matcher::*;
use source::{Source, StdinSource};
use squiddy_proto::message::Message;
use target::{Target, TlsTarget};

mod config;
mod filter;
mod source;
mod target;

fn main() {
    let config = AgentConfig::default();

    let source = StdinSource;

    if let Ok(mut target) = TlsTarget::connect() {
        let message = Message::ClientHello(String::from(""));

        target.accept(message);
    }

//    let config = AgentConfig::from_file(AgentConfig::DEFAULT_PATH);
    /*
    let input_lines = vec![
        "Hello world",
        "Delimited | Hello world",
        "Hello world,in,CSV,format",
    ];

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
