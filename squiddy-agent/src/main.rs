extern crate bytes;

use bytes::BytesMut;

pub trait Action {
    fn perform(&mut self);
}

pub struct IncrementCounter {
    name: String
}

impl Action for IncrementCounter {

    fn perform(&mut self) {

    }

}

pub trait Matcher {
    fn accept(&self, bytes: &mut BytesMut) -> bool;
}

pub struct LengthMatcher {
    threshold: usize
}

impl Matcher for LengthMatcher {
    fn accept(&self, bytes: &mut BytesMut) -> bool {
        bytes.len() >= self.threshold
    }
}

pub struct AnyMatcher {
    children: Vec<Box<Matcher>>
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

pub struct Condition {
    matcher: Box<Matcher>,
    actions: Vec<Box<Action>>
}

fn main() {
    let input_lines = vec![
        "Hello world",
        "Delimited | Hello world",
        "Hello world,in,CSV,format",
    ];

    let mut conditions = vec![
        Condition { matcher: Box::new(AnyMatcher { children: vec![] }), actions: vec![ Box::new(IncrementCounter { name: String::from("asdf") }) ] }
    ];

    let mut bytes = BytesMut::with_capacity(32);

    for line in input_lines {
        for condition in &mut conditions {
            if condition.matcher.accept(&mut bytes) {
                for action in &mut condition.actions {
                    action.perform();
                }
            }
        }
    }
}
