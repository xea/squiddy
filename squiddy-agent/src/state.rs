use super::event::Event;

pub struct State {
    vars: Vec<StateVariable>
}

impl State {
    pub fn accept(&mut self, event: Event) -> () {
        match event {
            Event::IncrementCounter(idx, diff) => (),
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
