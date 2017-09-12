use super::Target;
use squiddy_proto::message::Message;

/// `VecTarget` is a `Target` implementation that does not do any actual processing, only accepts
/// messages unconditionally and stores them in its internal `Vec` storage.
///
/// `VecTarget` is primarily intended for testing purposes only.
pub struct MemoryTarget {
    pub output: Vec<Message>
}

impl Default for MemoryTarget {
    fn default() -> Self {
        MemoryTarget { output: vec![] }
    }
}

impl Target for MemoryTarget {

    fn accept(&mut self, message: Message) -> bool {
        self.output.push(message);
        true
    }
}
