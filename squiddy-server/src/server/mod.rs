use std::net::SocketAddr;
use std::sync::{ Arc, RwLock };
use std::thread::{ JoinHandle, spawn };
use ::config::ServerConfig;
use ::state::State;
use tokio::prelude::{ Async, Future, Stream };
use tokio::net::{ TcpStream, TcpListener };
use tokio::io;

#[derive(Clone)]
pub struct Server {
    config: ServerConfig,
    state: Arc<RwLock<State>>
}

impl Server {
    pub fn new(config: ServerConfig, state: Arc<RwLock<State>>) -> Self {
        Self { config, state }
    }

    pub fn start(&mut self) {
        let addr = SocketAddr::new(self.config.listen_address, self.config.listen_port);

        if let Ok(listener) = TcpListener::bind(&addr) {
            let server = listener.incoming().for_each(move |socket| {
                println!("New connection: {:?}", socket.peer_addr().unwrap());

                let connection = AcceptClient::new(socket).then(|_| {
                    println!("Accepted and stuff");
                    Ok(())
                });

                ::tokio::spawn(connection);

                Ok(())
            }).map_err(|err| println!("Accept error: {:?}", err));

            ::tokio::run(server);
        } else {
            println!("Failed to bind to address: {:?}", addr);
        }
    }

    pub fn start_detached(&mut self) -> JoinHandle<()> {
        let mut self_clone = self.clone();

        spawn(move || {
            self_clone.start();
        })
    }

    pub fn stop(self, handle: JoinHandle<()>) {
        match handle.join() {
            Ok(h) => println!("Server stopped {:?}", h),
            Err(err) => println!("Error while joining {:?}", err)
        }
    }
}

enum ClientState {
    NewConnection(TcpStream),
}

struct AcceptClient {
    state: ClientState,
}

impl AcceptClient {
    pub fn new(stream: TcpStream) -> AcceptClient {
        Self { state: ClientState::NewConnection(stream) }
    }
}

impl Future for AcceptClient {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        /*
        match self.state {
            ClientState::NewConnection(ref mut stream) => {
                let mut a = io::write_all(stream, "HELLO\n")
                .and_then(|(s, b)| {
                    let buf = vec![0; 5];
                    io::read_exact(s, buf)
                })
                .and_then(|(s, b)| { io::write_all(s, "ACK\n") })
                .and_then(|(s, b)| {
                    let buf = vec![0; 5];
                    io::read_exact(s, buf)
                })
                .and_then(|(s, b)| { io::write_all(s, "KTHXBYE\n") })
                .map(|_| ());
                
                a.poll()
            }
        }
        */

        match self.state {
            ClientState::NewConnection(ref mut stream) => {
                let mut a = io::write_all(stream, "HELLO\n")
                .and_then(|(s, b)| {
                    let buf = vec![0; 5];
                    io::read_exact(s, buf)
                })
                .and_then(|(s, b)| { io::write_all(s, "ACK\n") })
                .and_then(|(s, b)| {
                    let buf = vec![0; 5];
                    io::read_exact(s, buf)
                })
                .and_then(|(s, b)| { io::write_all(s, "KTHXBYE\n") })
                .map(|_| ());

                loop {
                    let v = match a.poll() {
                        Ok(Async::Ready(t)) => t,
                        Ok(Async::NotReady) => return Ok(Async::NotReady),
                        Err(err) => return Err(err)
                    };

                    println!("Got result: {:?}", v);
                    break;
                };

                Ok(Async::Ready(()))
            }
        }
    }
}