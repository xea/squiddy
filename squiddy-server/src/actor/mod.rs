pub struct ActorRef {

}

pub struct ActorSystem {
    name: String
}

impl ActorSystem {
    pub fn create(name: String) -> Self {
        Self { name }
    }

    pub fn actorOf(name: String) -> ActorRef {
        ActorRef {}
    }
}

pub enum Message {

}

pub trait BaseActor {

    fn receive(message: Message);
}