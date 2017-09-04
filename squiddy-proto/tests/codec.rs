extern crate bytes;
extern crate squiddy_proto;
extern crate tokio_io;

use squiddy_proto::message::Message;
use squiddy_proto::codec::*;
use bytes::BytesMut;
use tokio_io::codec::{Encoder, Decoder};

#[test]
fn server_hello_is_encoded_and_decoded() {
    let msg = Message::ServerHello((0, 0));

    let mut encoder = SquiddyEncoder;
    let mut buffer = BytesMut::with_capacity(32);

    let _ = encoder.encode(msg, &mut buffer);


    let mut decoder = SquiddyDecoder;
    let decoded_msg = decoder.decode(&mut buffer);

    match decoded_msg {
        Ok(maybe_msg) => match maybe_msg {
            Some(decoded_msg) => assert_eq!(decoded_msg, Message::ServerHello((0, 0))),
            _ => assert!(false)
        },
        _ => assert!(false)
    }
}
