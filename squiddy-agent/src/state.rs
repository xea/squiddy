use super::event::Event;

pub struct State {
    pub vars: Vec<StateVariable>
}

impl State {
    pub fn accept(&mut self, event: Event) -> () {
        // Update global counter
        match &mut self.vars[0] {
            &mut StateVariable::U32(value) => self.vars[0] = StateVariable::U32(value + 1)
        };

        // Update event
        match event {
            Event::IncrementCounter(idx) => self.increment_var(idx, 1),
        }
    }

    pub fn register_u32(&mut self) -> usize {
        // All unsigned integers are initialised to 0
        self.vars.push(StateVariable::U32(0));

        self.vars.len() - 1
    }

    fn increment_var(&mut self, idx: usize, diff: usize) -> () {
        match &mut self.vars[idx] {
            &mut StateVariable::U32(value) => self.vars[idx] = StateVariable::U32(value + diff as u32),
        }
    }

    /*
    fn decrement_var(&mut self, idx: usize, diff: usize) -> () {
        match &mut self.vars[idx] {
            &mut StateVariable::U32(value) => self.vars[idx] = StateVariable::U32(value - diff as u32),
        }
    }
    */
}

impl Default for State {

    fn default() -> Self {
        State { vars: vec![
            // Default counter, normally counting the number of inputs
            StateVariable::U32(0)
        ]}
    }
}

#[derive(Debug)]
pub enum StateVariable {
    U32(u32),
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn registering_a_variable_should_return_the_right_id() {
        let mut state = State::default();

        let idx = state.register_u32();
        assert_eq!(idx, 0);

        let idx = state.register_u32();
        assert_eq!(idx, 1);

        let idx = state.register_u32();
        assert_eq!(idx, 2);
    }
}
