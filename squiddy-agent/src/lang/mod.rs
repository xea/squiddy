use super::state::State;
use super::filter::condition::Condition;

pub mod ast;
pub mod compiler;
pub mod token;

pub struct SquiddyProgram {
    pub state: State,
    pub conditions: Vec<Condition>

}

impl SquiddyProgram {

}

impl Default for SquiddyProgram {

    fn default() -> SquiddyProgram {
        SquiddyProgram {
            state: State::default(),
            conditions: vec![]
        }
    }
}
