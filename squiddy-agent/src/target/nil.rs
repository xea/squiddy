use super::Target;
use squiddy_proto::message::Message;

pub struct NilTarget;

impl Target for NilTarget {

    fn accept(&mut self, message: Message) -> bool {
        true
    }
}
