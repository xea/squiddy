#[cfg(unix)]
extern crate termion;
extern crate bytes;
extern crate futures;
extern crate tokio;

mod agent;
mod config;
mod server;
mod state;
mod terminal;

use config::ServerConfig;
use server::Server;
use state::State;
use std::sync::{ Arc, RwLock };
use terminal::terminal::Terminal;

fn main() {
    let state = State::new();
    let arc = Arc::new(RwLock::new(state));
    let config = ServerConfig::from_args();

    let mut stdout = Terminal::stdout();
    let mut terminal = Terminal::new(&mut stdout, arc.clone());
    let mut server = Server::new(&config, arc.clone());
    
    server.start();
    terminal.start();
    
    server.stop();
    
}
