extern crate termion;

mod agent;
mod config;
mod event;
mod server;
mod state;
mod terminal;

use config::ServerConfig;
use server::Server;
use state::State;
use std::sync::{ Arc, RwLock };
use std::io::stdout;
use terminal::Terminal;
use termion::raw::{ IntoRawMode };

fn main() {
    let state = State::new();
    let arc = Arc::new(RwLock::new(state));
    let config = ServerConfig::from_args();

    let mut server = Server::new(&config, arc.clone());
    let mut stdout = stdout().into_raw_mode().unwrap();
    let mut terminal = Terminal::new(&mut stdout, arc.clone());

    server.start();
    terminal.start();
}
