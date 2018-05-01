#[cfg(unix)]
extern crate termion;
extern crate bytes;
#[macro_use]
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

    let mut server = Server::new(config.clone(), arc.clone());
    let handle = server.start_detached();

    start_terminal(arc.clone());

    println!("\nPress Ctrl-c to exit program");

    server.stop(handle);
}

fn start_terminal(state: Arc<RwLock<State>>) {
    let mut stdout = Terminal::stdout();
    let mut terminal = Terminal::new(&mut stdout, state);

    terminal.start();

}
