extern crate futures;
extern crate termion;
extern crate tokio;
extern crate tokio_core;
extern crate tokio_tls;

mod agent;
mod config;
mod event;
mod server;
mod state;
mod terminal;

use agent::Agent;
use futures::Stream;
use server::Server;
use state::State;
use std::io::stdout;
use std::sync::{ Arc, RwLock };
use termion::raw::{ IntoRawMode };
use terminal::Terminal;

fn main() {
    if let Ok(mut out) = stdout().into_raw_mode() {
        let mut state = State::new();
        //let arc = Arc::new(RwLock::new(state));

        let (tx, rx) = ::futures::sync::mpsc::channel(65535);

        let mut terminal = Terminal::new(&mut out);

        let vt = std::thread::spawn(move || {
            rx.then(|res| {
                println!("asdfasdf");
                res
            });
        });

        let mut server = Server::new(tx);//arc.clone());
        server.start();

        vt.join();
    }
}
