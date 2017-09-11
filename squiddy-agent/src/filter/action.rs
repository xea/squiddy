/// `Action` is the abstract definition of something that can be performed whenever a condition
/// is triggered.
pub trait Action {

    /// Perform this action
    fn perform(&mut self);
}

/// An `Action` that just ignores any kind of input, ie. does nothing.
pub struct IgnoreInput;

impl Action for IgnoreInput {
    fn perform(&mut self) {}
}

pub struct IncrementCounter {
    pub name: String
}

impl Action for IncrementCounter {

    fn perform(&mut self) {
        println!("Increment counter");
    }

}
