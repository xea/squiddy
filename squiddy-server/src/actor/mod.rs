use std::sync::{ Arc, RwLock };
use std::thread;
use std::thread::JoinHandle;

pub struct ActorRef {
    inbox: Arc<RwLock<Inbox>>
}

pub struct ActorSystem {
    name: String,
    executor: Executor
}

impl ActorSystem {
    pub fn create(name: String) -> Self {
        let mut executor = Executor::new(8);

        executor.start();

        Self { 
            name, 
            executor
        }
    }

    pub fn actorOf(actor_object: Box<BaseActor>, name: String) -> ActorRef {
        let new_inbox = Inbox::new();
        let shared_inbox = Arc::new(RwLock::new(new_inbox));

        ActorRef {
            inbox: shared_inbox
        }
    }
}

pub enum Message {

}

struct Inbox {
    message_queue: Vec<Message>
}

impl Inbox {
    pub fn new() -> Self {
        Inbox {
            message_queue: vec![]
        }
    }
}

pub trait BaseActor {

    fn receive(&mut self, message: Message);
}

struct Executor {
    worker_threads: Vec<JoinHandle<()>>,
    watched_inboxes: Vec<Arc<RwLock<Inbox>>>,
    max_worker_count: usize
}

impl Executor {
    pub fn new(worker_count: usize) -> Self {
        Self {
            worker_threads: Vec::with_capacity(worker_count),
            max_worker_count: worker_count,
            watched_inboxes: vec![]
        }
    }

    pub fn start(&mut self) {
        for _ in 0..self.max_worker_count {
            let new_thread = thread::spawn(move || {
                //for inbox in self.watched_inboxes {

                //}
            });

            self.worker_threads.push(new_thread);
        }
    }
}