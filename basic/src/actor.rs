use tokio::sync::{oneshot, mpsc};
// The base actor, which our ActorHandle will be able to control. Holds data and an ID.
struct Actor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: i32,
    message: Option<String>,
    alive: bool
}

// An enum which will hold a Sender from a oneshot channel.
// This enum can also hold data (so for example if we want to change some data on the Actor)
pub enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    GetData {
        respond_to: oneshot::Sender<Option<String>>,
    },
    SetData {
        respond_to: oneshot::Sender<Option<String>>,
        message: Option<String>
    },
    Kill
}

impl Actor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        Self {
            receiver,
            next_id: 0,
            message: None,
            alive: true
        }
    }

    // do something based on what the enum is
    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId {respond_to} => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id.try_into().unwrap());
            }
            ActorMessage::GetData {respond_to} => {
                let _ = respond_to.send(self.message.clone());
            }
            ActorMessage::SetData {respond_to, message} => {
                self.message = message;
                let _ = respond_to.send(self.message.clone());
            },
            ActorMessage::Kill => {
                self.alive = false;
            }
        }
    }
}

// run the actor - this is run in a separate thread when creating the ActorHandle
async fn run_my_actor(mut actor: Actor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }

    if !actor.alive {
        drop(actor);
        println!("Actor was killed");
    }
}

#[derive(Clone)]
pub struct ActorHandle {
    sender: mpsc::Sender<ActorMessage>,
    alt_channel: Option<mpsc::Sender<ActorMessage>>
}

impl ActorHandle {
    // this creates the actor while creating the handle
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Actor::new(receiver);
        tokio::spawn(run_my_actor(actor));

        Self { sender, alt_channel: None }
    }
    
    pub fn new_and_receives_from(alt_channel: mpsc::Sender<ActorMessage>) -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = Actor::new(receiver);
        tokio::spawn(run_my_actor(actor));

        Self { sender, alt_channel: Some(alt_channel) }
    }

    pub fn change_receives_from(mut self, alt_channel: mpsc::Sender<ActorMessage>) {
        self.alt_channel = Some(alt_channel);
    }

    pub async fn get_unique_id(&self) -> u32 {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetUniqueId {
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    // get data
    pub async fn get_data(&self) -> Option<String> {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::GetData {
            respond_to: send,
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    // set data
    pub async fn set_data(&self, message: String) -> Option<String> {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::SetData {
            respond_to: send,
            message: Some(message)
        };

        // Ignore send errors. If this send fails, so does the
        // recv.await below. There's no reason to check for the
        // same failure twice.
        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }

    pub async fn kill_actor(&self) {
        let _ = self.sender.send(ActorMessage::Kill).await;
        println!("Actor was killed")
    }
}

impl Default for ActorHandle {
    fn default() -> Self {
        Self::new()
    }
}