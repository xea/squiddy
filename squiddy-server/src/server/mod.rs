use std::net::SocketAddr;
use std::sync::{ Arc, RwLock };
use std::thread::{ JoinHandle, spawn };
use ::config::ServerConfig;
use ::state::State;
use bytes::BytesMut;
use tokio::prelude::{ Async, AsyncRead, Future, Poll, Stream };
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

                let codec = ClientCodec::new(socket);

                let connection = codec.into_future()
                    .map_err(|_| ())
                    .map(|_| ());
/*
                let connection = TestFuture::new(socket).then(|_| {
                    println!("Accepted and stuff");
                    Ok(())
                }).map_err(|_| ());
                */

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

struct ClientCodec {
    stream: TcpStream,
    read_buffer: BytesMut,
    write_buffer: BytesMut
}

impl ClientCodec {
    pub fn new(stream: TcpStream) -> Self {
        Self { 
            stream,
            read_buffer: BytesMut::new(),
            write_buffer: BytesMut::new()
        }
    }

    fn read_next_packet(&mut self) -> Poll<(), io::Error> {
        loop {
            self.read_buffer.reserve(1024);

            let n = try_ready!(self.stream.read_buf(&mut self.read_buffer));

            println!("Got value from try_ready: {}", n);

            if n == 0 {
                return Ok(Async::Ready(()))
            }
        }
    }
}

impl Stream for ClientCodec {

    type Item = ClientMessage;
    type Error = io::Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let sock_closed = self.read_next_packet()?.is_ready();

        println!("- read next {:?} {:?}", sock_closed, self.read_buffer.len());

        if self.read_buffer.len() >= 10 {
            let msg_code = self.read_buffer.split_to(2);

            match &msg_code[..] {
                b"he" => println!("Hello request"),
                _ => println!("Unknown package")

            }

            return Ok(Async::Ready(Some(ClientMessage::Noop)))
        }

        if sock_closed {
            Ok(Async::Ready(None))
        } else {
            Ok(Async::NotReady)
        }
    }
}

enum ClientMessage {
    Noop
}

/*
struct TestFuture {
    inner: Box<Future<Item=(), Error=()>>
}

impl TestFuture {
    pub fn new(stream: TcpStream) -> Self {
        Self {
            inner: Box::new(io::write_all(stream, "HELLO\n").map(|_| ()).map_err(|_| ()))
        }
    }
}

impl Future for TestFuture {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        let v = try_ready!(self.inner.poll());
        Ok(Async::Ready(()))

        //try_ready!(self.inner.poll())
    }
    
}
*/

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