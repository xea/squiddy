use futures::sync::mpsc;
use std::net::{ IpAddr, Ipv4Addr, SocketAddr };
use std::time::Instant;
use super::agent::Agent;
use super::state::*;
use tokio::io;
use tokio::net::TcpListener;
use tokio::prelude::*;

pub struct Server {
    tx: mpsc::Sender<StateUpdate>
}

impl Server {
    pub fn new(tx: mpsc::Sender<StateUpdate>) -> Self {
        Server {
            tx: tx
        }
    }

    pub fn start(&mut self) {
        let addr = "0.0.0.0:7979".parse().unwrap();
        let tcp = TcpListener::bind(&addr).unwrap();

        // Iterate incoming connections
        let server = tcp.incoming().for_each(|tcp| {
            // tcp == socket
            println!("accepted socket; addr={:?}", tcp.peer_addr().unwrap());

                //let mut s = self.state.write().unwrap();
            // Split up the read and write halves
            let (reader, writer) = tcp.split();

            // Copy the data back to the client
            let conn = io::copy(reader, writer)
                // print what happened
                .map(|(n, _, _)| { println!("wrote {} bytes", n) })
                // Handle any errors
                .map_err(|err| { println!("IO error {:?}", err) });

            // Spawn the future as a concurrent task
            ::tokio::spawn(conn);

            Ok(())
        })
        .map_err(|err| {
            println!("server error {:?}", err);
        });

        // Start the runtime and spin up the server
        ::tokio::run(server);
    }
}
