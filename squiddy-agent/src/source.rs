use bytes::{Bytes, BytesMut};
use std::io::Error;

#[derive(Deserialize)]
pub enum SourceType {
    Stdin
}

pub trait Source {

    fn next(&mut self) -> Bytes;

    fn has_more(&self) -> bool;
}

/// `StdinSource` is a `Source` implementation that gets its items from the standard input.
pub struct StdinSource;

impl Source for StdinSource {
    fn next(&mut self) -> Bytes {
        BytesMut::with_capacity(10).freeze()

    }

    fn has_more(&self) -> bool {
        true
    }
}

pub struct SourceBuilder;

impl SourceBuilder {
    pub fn build(source_type: SourceType) -> Result<Box<Source>, Error> {
        match source_type {
            SourceType::Stdin => Ok(Box::new(StdinSource))
        }
    }
}
