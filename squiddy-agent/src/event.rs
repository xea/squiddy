#[derive(Debug)]
pub enum Event {
    IncrementCounter(usize, u32),
    DecrementCounter(usize, u32),
    SetCounter(usize, u64),
}
