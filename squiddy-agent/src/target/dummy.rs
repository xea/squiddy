use super::Target;
use super::super::state::State;
use std::time::{Duration, Instant};

pub struct DummyTarget {
    last_update: Instant
}

impl DummyTarget {
    pub fn new() -> Self {
        DummyTarget {
            last_update: Instant::now()
        }
    }
}

impl Target for DummyTarget {

    fn accept(&mut self, state: &State) -> bool {
        let now = Instant::now();

        if now - self.last_update > Duration::from_secs(1) {
            for var in &state.vars {
                println!("Var {:?}", var);
            }

            self.last_update = now;
            true
        } else {
            println!("Stale");
            false
        }
    }
}
