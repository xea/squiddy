extern crate termion;

mod agent;
mod server;
mod terminal;

use std::io::stdout;

use agent::Agent;
use server::Server;
use termion::raw::{ IntoRawMode };
use terminal::Terminal;

fn main() {
    if let Ok(mut terminal) = stdout().into_raw_mode() {
        let server = Server::new();

        let out = &mut terminal;
        let mut terminal = Terminal::new(out);

    }
}
