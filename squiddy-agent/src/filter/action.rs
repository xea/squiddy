use super::super::event::Event;

/// `Action` is the abstract definition of something that can be performed whenever a condition
/// is triggered.
pub trait Action {

    /// Perform this action
    fn perform(&mut self) -> Option<Vec<Event>>;
}

pub struct IncrementCounter {
    pub name: String,
    pub idx: usize
}

impl Action for IncrementCounter {

    fn perform(&mut self) -> Option<Vec<Event>> {
        Some(vec![Event::IncrementCounter(self.idx)])
    }

}
