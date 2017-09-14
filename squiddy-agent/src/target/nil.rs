use super::Target;
use super::super::state::State;

pub struct NilTarget;

impl Target for NilTarget {

    fn accept(&mut self, _: &State) -> bool {
        true
    }
}
