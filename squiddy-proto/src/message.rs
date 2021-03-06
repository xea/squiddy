pub type ServiceId = u16;
pub type ProtocolVersion = (u8, u8);
pub type ClientName = String;

#[derive(Debug, PartialEq)]
pub enum Message {
    /// Initial greeting message sent by the Squiddy server when a client connects to it.
    /// The `ServerHello` message includes a version number that identifies the protocol the
    /// server speaks.
    ServerHello(ProtocolVersion),

    /// Greeting response the client must send to the server, in reply to `ServerHello` messages
    /// if the client. Client name can be any arbitrary string but it's length is limited to 32
    /// bytes.
    ClientHello(ClientName)
}
