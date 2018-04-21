use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use super::agent::Agent;
use super::config::ServerConfig;
use super::state::*;
use std::sync::{Arc, RwLock};

use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;
use futures::channel::mpsc::{ channel, Sender, Receiver };
//use futures::future;
//use futures::{ IntoFuture, SinkExt };
use std::thread;

pub struct Server<'c> {
    state: Arc<RwLock<State>>,
    config: &'c ServerConfig
}

#[derive(Debug)]
pub enum Event {
    NOP
}

pub struct ClientEvent {
}

impl Stream for ClientEvent {
    type Item = Event;
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Option<Self::Item>>, Self::Error> {
        Ok(Async::NotReady)
    }
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
        let addr = SocketAddr::new(self.config.listen_address, self.config.listen_port);
        let listener = TcpListener::bind(&addr).unwrap();

        let (tx, mut rx) = channel(65536);
        let local_state = self.state.clone();

        let server = listener.incoming().for_each(move |socket| {
            println!("accepted socket; addr={:?}", socket.peer_addr().unwrap());

            let event_stream = ClientEvent {};

            let preconn = event_stream.into_future().map(|_| ()).map_err(|(e, _)| e).map_err(|_| ());

            ::tokio::spawn(preconn);


            let lsg = local_state.clone();

            if let Ok(mut wr) = lsg.write() {
                
            }

            let greeting = format!("!SQUIDDY{}{}\n", 0, 1);

            let connection = io::write_all(socket, greeting)
            .and_then(|(a, b)| {
                let buf = vec![0; 5];
                io::read_exact(a, buf)
            }).and_then(|(a, b)| {
                io::write_all(a, "More lines\n")
            }).and_then(|(a, b)| {
                let buf = vec![0; 5];
                io::read_exact(a, buf)
            }).then(|res| {
                println!("Greeting sent, success={:?}", res);
                Ok(())
            });
            
            ::tokio::spawn(connection);

            let task = SyncTask { tx: tx.clone() };

            ::tokio::spawn(task);

            Ok(())
        }).map_err(|err| println!("accept error = {:?}", err));

/*
        let handle = thread::spawn(move || {
            let lsg = sg.clone();

            loop {
                if let Ok(msg) = rx.try_next() {
                    println!("Got message: {:?}", msg);
                }
            }
        });
        
        handle.join().unwrap();
        */

        ::tokio::run(server);


        if let Ok(mut state) = self.state.write() {
            let agents: &mut Vec<Agent> = &mut (*state).registered_agents;

            let a = Ipv4Addr::new(127, 0, 0, 1);
            let b = IpAddr::V4(a);
            let c = SocketAddr::new(b, 8080);
            agents.push(Agent::new(c));
        }
    }
}
