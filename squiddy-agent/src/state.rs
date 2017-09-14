use super::event::Event;

pub struct State {
    vars: Vec<StateVariable>
}

impl State {
    pub fn accept(&mut self, event: Event) -> () {
        match event {
            Event::IncrementCounter(idx) => self.increment_var(idx, 1),
            Event::DecrementCounter(idx) => self.decrement_var(idx, 1),
            _ => ()
        }
    }

    pub fn register_u32(&mut self) -> usize {
        self.vars.push(StateVariable::U32(0));

        self.vars.len()
    }

    pub fn register_u64(&mut self) -> usize {
        self.vars.push(StateVariable::U64(0));

        self.vars.len()
    }

    fn increment_var(&mut self, idx: usize, diff: usize) -> () {
        match &mut self.vars[idx] {
            &mut StateVariable::U32(value) => self.vars[idx] = StateVariable::U32(value + diff as u32),
            &mut StateVariable::U64(value) => self.vars[idx] = StateVariable::U64(value + diff as u64)
        }
    }

    fn decrement_var(&mut self, idx: usize, diff: usize) -> () {
        match &mut self.vars[idx] {
            &mut StateVariable::U32(value) => self.vars[idx] = StateVariable::U32(value - diff as u32),
            &mut StateVariable::U64(value) => self.vars[idx] = StateVariable::U64(value - diff as u64)
        }
    }
}

impl Default for State {

    fn default() -> Self {
        State { vars: vec![] }
    }
}

pub enum StateVariable {
    U32(u32),
    U64(u64),
}
