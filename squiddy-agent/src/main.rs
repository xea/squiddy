extern crate bytes;
extern crate native_tls;
extern crate squiddy_proto;
extern crate toml;
#[macro_use]
extern crate serde_derive;
extern crate tokio_io;

use config::AgentConfig;
use pipeline::Pipeline;

mod config;
mod filter;
mod pipeline;
mod source;
mod target;

fn main() {
    let config = AgentConfig::default();

    match Pipeline::new(config) {
        Ok(mut pipeline) => pipeline.run(),
        Err(err) => println!("Error: {}", err)
    }
}
