use std::sync::{ Arc, RwLock };
use std::thread::{ JoinHandle, spawn };
use ::config::ServerConfig;
use ::state::State;

pub struct Server<'c> {
    config: &'c ServerConfig,
    state: Arc<RwLock<State>>,
    detached_thread: Option<JoinHandle<()>>
}

impl<'c> Server<'c> {
    pub fn new(config: &'c ServerConfig, state: Arc<RwLock<State>>) -> Self {
        Self { config, state, detached_thread: None }
    }

    pub fn start(&mut self) {
        // For now any start will start the server in detached mode but it's just convenient, not necessary
        self.start_detached();
    }

    pub fn start_detached(&mut self) {
        self.detached_thread = Some(spawn(move || {
            
        }));
    }

    pub fn stop(self) {
        self.detached_thread.map(|thread| thread.join().unwrap());
    }
}