use super::super::event::Event;

/// `Action` is the abstract definition of something that can be performed whenever a condition
/// is triggered.
pub trait Action {

    /// Perform this action
    fn perform(&mut self) -> Option<Vec<Event>>;
}

/// An `Action` that just ignores any kind of input, ie. does nothing.
pub struct IgnoreInput;

impl Action for IgnoreInput {
    fn perform(&mut self) -> Option<Vec<Event>> {
        None
    }
}

pub struct IncrementCounter {
    pub name: String,
    pub i: usize
}

impl Action for IncrementCounter {

    fn perform(&mut self) -> Option<Vec<Event>> {
        Some(vec![Event::IncrementCounter(self.i)])
    }

}
