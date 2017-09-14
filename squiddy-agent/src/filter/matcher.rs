use bytes::Bytes;

pub trait Matcher {
    fn accept(&self, bytes: &Bytes) -> bool;
}

pub struct AlwaysMatcher;

impl Matcher for AlwaysMatcher {
    fn accept(&self, _: &Bytes) -> bool {
        true
    }
}

pub struct LengthMatcher {
    pub threshold: usize
}

impl Matcher for LengthMatcher {
    fn accept(&self, bytes: &Bytes) -> bool {
        bytes.len() >= self.threshold
    }
}

pub struct AnyMatcher {
    pub children: Vec<Box<Matcher>>
}

impl Matcher for AnyMatcher {

    fn accept(&self, bytes: &Bytes) -> bool {
        for child in &self.children {
            if child.accept(bytes) {
                return true;
            }
        }

        false
    }
}
