#[derive(Debug)]
pub enum Event {
    IncrementCounter(usize),
    DecrementCounter(usize),
    SetCounter(usize, u64),
}
