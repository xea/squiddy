use std::net::SocketAddr;
use std::sync::{ Arc, RwLock };
use std::thread::{ JoinHandle, spawn };
use futures::future;
use futures::future::Either;
use tokio::prelude::{ Async, Future, Poll, Stream };
use tokio::net::TcpListener;
use tokio::io;
use super::agent::Agent;
use super::config::ServerConfig;
use super::state::State;
use self::codec::ClientCodec;

mod codec;

/// Instances of `Server` are responsible for creating and maintaining data collection services to which agents can connect and send
/// updates. It's primary task is to collect and process the incoming update requests and update it's internal shared state, accordingly.
/// 
/// `Server` is not responsible for generating any sort of output, it merely updates the shared `state` that other services may read.
#[derive(Clone)]
pub struct Server {
    config: ServerConfig,
    state: Arc<RwLock<State>>
}

impl Server {
    /// Create a new `Server` instance that is ready to start.
    pub fn new(config: ServerConfig, state: Arc<RwLock<State>>) -> Self {
        Self { config, state }
    }

    /// Start the data collection service inline (ie. on the same thread as it was launched on and blocking the current thread until
    /// termination, which is usually the end of the program)
    pub fn start(&mut self) {
        let listen_address = SocketAddr::new(self.config.listen_address, self.config.listen_port);

        if let Ok(listener) = TcpListener::bind(&listen_address) {
            let server = listener.incoming().for_each(move |socket| {
                // Extracting the client address early on as socket itself will be moved into ClientCodec
                let client_address = socket.peer_addr().unwrap();

                let codec = ClientCodec::new(socket);

                // Calling into_future() first ensures that only one message is returned at this time so we're able to
                // authenticate/authorise the connecting client before accepting other messages
                let connection = codec.into_future()
                    // TBH I'm not quite sure why we need this map_err here but it looks like into_future's error type 
                    // is a tuple and we're expected to pass down a single Error
                    .map_err(|(e, _)| e)
                    .and_then(move |(greeting, messages)| {
                        let agent_info = match greeting {
                            // We expect a ClientHello as a first message in which the client identifies itself. 
                            Some(ClientMessage::ClientHello { name }) => Agent::new(client_address, name),
                            // If the first event we received from the client wasn't a ClientHello event, then we simply
                            // drop the connection.
                            _ => return Either::A(future::ok(())),
                        };

                        let client = Client::new(messages);

                        Either::B(client)
                    })
                    // And again, the final pipeline is expected to return () for errors, enforcing proper error handling
                    .map_err(|_| ());

                ::tokio::spawn(connection);

                Ok(())
            }).map_err(|err| println!("Accept error: {:?}", err));

            ::tokio::run(server);
        } else {
            println!("Failed to bind to address: {:?}", listen_address);
        }
    }

    /// Start data collecting similarly to `start()` but it is performed on a separate thread and as such, it does not block the 
    /// calling thread. 
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

#[derive(Debug)]
pub enum ClientMessage {
    Noop,
    ClientHello { name: String }
}

struct Client {
    codec: ClientCodec
}

impl Client {
    pub fn new(codec: ClientCodec) -> Self {
        Self { codec }
    }
}

impl Future for Client {
    type Item = ();
    type Error = io::Error;

    fn poll(&mut self) -> Poll<(), io::Error> {
        while let Async::Ready(event) = self.codec.poll()? {
            println!("Got some event: {:?}", event);

            if let Some(msg) = event {
                println!("Processing message: {:?}", msg);
            } else {
                println!("No more messages, client has disconnected");
                return Ok(Async::Ready(()))
            }
        }

        Ok(Async::NotReady)
    }
}

/*
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
*/