#[cfg(unix)]
extern crate termion;

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
    let mut server = Server::new(&config, arc.clone());
//        stdout().into_raw_mode().unwrap()
    let mut terminal = Terminal::new(&mut stdout, arc.clone());

    server.start();
    terminal.start();
}
