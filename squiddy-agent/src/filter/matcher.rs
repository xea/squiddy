use bytes::BytesMut;

pub trait Matcher {
    fn accept(&self, bytes: &mut BytesMut) -> bool;
}

pub struct LengthMatcher {
    pub threshold: usize
}

impl Matcher for LengthMatcher {
    fn accept(&self, bytes: &mut BytesMut) -> bool {
        bytes.len() >= self.threshold
    }
}

pub struct AnyMatcher {
    pub children: Vec<Box<Matcher>>
}

impl Matcher for AnyMatcher {

    fn accept(&self, bytes: &mut BytesMut) -> bool {
        for child in self.children.iter() {
            if child.accept(bytes) {
                return true;
            }
        }

        false
    }
}
