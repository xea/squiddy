use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use super::agent::Agent;
use super::config::ServerConfig;
use super::state::*;
use std::sync::{Arc, RwLock};

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::channel::mpsc::{ channel, Sender, Receiver };
use futures::future;
use futures::{ IntoFuture, SinkExt };
use std::thread;

pub struct Server<'c> {
    state: Arc<RwLock<State>>,
    config: &'c ServerConfig
}

#[derive(Debug)]
enum Event {
    NOP
}

pub struct SyncTask {
    tx: Sender<Event>
}

impl Future for SyncTask {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<()>, ()> {
        self.tx.try_send(Event::NOP);//.unwrap();
        Err(())
    }
}

impl<'c> Server<'c> {
    pub fn new(config: &'c ServerConfig, state: Arc<RwLock<State>>) -> Self {
        Server { state, config }
    }

    pub fn start(&mut self) {
        let addr = "0.0.0.0:7979".parse().unwrap();
        let listener = TcpListener::bind(&addr).unwrap();

        let (tx, mut rx) = channel(65536);

        let server = listener.incoming().for_each(move |socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let connection = io::write_all(socket, "Hello guy\n")
            .then(|res| {
                println!("Greeting sent, success={:?}", res);
                Ok(())
            });
            
            ::tokio::spawn(connection);

            let task = SyncTask { tx: tx.clone() };

            ::tokio::spawn(task);

            //let sync = future::result(tx.send(Event::NOP)).into_future();

            //let s: String = tx.send(Event::NOP);

            //::tokio::spawn(sync);

            Ok(())
        }).map_err(|err| println!("accept error = {:?}", err));

        let handle = thread::spawn(move || {
            loop {
                if let Ok(msg) = rx.try_next() {
                    println!("Got message: {:?}", msg);
                }
            }
        });

        ::tokio::run(server);

        handle.join().unwrap();

        if let Ok(mut state) = self.state.write() {
            let agents: &mut Vec<Agent> = &mut (*state).registered_agents;

            let a = Ipv4Addr::new(127, 0, 0, 1);
            let b = IpAddr::V4(a);
            let c = SocketAddr::new(b, 8080);
            agents.push(Agent::new(c));
        }
    }
}
