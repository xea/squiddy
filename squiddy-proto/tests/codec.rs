extern crate bytes;
extern crate squiddy_proto;
extern crate tokio_io;

use squiddy_proto::message::Message;
use squiddy_proto::codec::*;
use bytes::{BytesMut, BufMut};
use tokio_io::codec::{Encoder, Decoder};

#[test]
fn bytes_test() {
    let mut buffer = BytesMut::with_capacity(32);

    assert_eq!(0, buffer.len());
    assert_eq!(32, buffer.remaining_mut());

    buffer.put_u8(13);

    assert_eq!(1, buffer.len());
    assert_eq!(31, buffer.remaining_mut());

    buffer.put_u8(13);

    assert_eq!(2, buffer.len());
    assert_eq!(30, buffer.remaining_mut());

    let a: BytesMut = buffer.take();

    assert_eq!(2, a.len());
    assert_eq!(0, a.remaining_mut());

    assert_eq!(0, buffer.len());
    assert_eq!(30, buffer.remaining_mut());
}

#[test]
fn server_hello_is_encoded_and_decoded() {
    test_encoding_roundtrip(|| Message::ServerHello((1, 3)));
}

#[test]
fn client_hello_is_encoded_and_decoded() {
    test_encoding_roundtrip(|| Message::ClientHello(String::from("Test client")));
}

#[test]
fn client_hello_too_long_names_are_truncated_to_maximum_allowed_length() {
    // At the moment we're limiting agent names to 32 characters
    test_encoding_roundtrip_bidirectional(
        || Message::ClientHello(String::from("1234567890123456789012345678901234567890")),
        || Message::ClientHello(String::from("12345678901234567890123456789012")));
}

fn test_encoding_roundtrip(input_generator: fn() -> Message) {
    test_encoding_roundtrip_bidirectional(input_generator, input_generator);
}

fn test_encoding_roundtrip_bidirectional(input_generator: fn() -> Message, output_generator: fn() -> Message) {
    let msg = input_generator();

    let mut encoder = SquiddyEncoder;
    let mut decoder = SquiddyDecoder;

    let mut buffer = BytesMut::with_capacity(64);

    let _ = encoder.encode(msg, &mut buffer);

    //assert!(false, format!("{:?}", buffer));
    //assert_eq!(0, buffer.remaining_mut());

    let mut read_buffer = buffer.take();

    let decoded_msg = decoder.decode(&mut read_buffer);

    match decoded_msg {
        Ok(maybe_msg) => match maybe_msg {
            Some(decoded_msg) => assert_eq!(decoded_msg, output_generator()),
            _ => assert!(false, "Decoded message is invalid")
        },
        Err(error) => assert!(false, format!("Message couldn't be decoded: {}", error))
    }
}
