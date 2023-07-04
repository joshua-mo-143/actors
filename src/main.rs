use tokio::sync::{oneshot, mpsc};

struct MyActor {
    receiver: mpsc::Receiver<ActorMessage>,
    next_id: i32,
    data: String
}

enum ActorMessage {
    GetUniqueId {
        respond_to: oneshot::Sender<u32>,
    },
    GetData {
        respond_to: oneshot::Sender<String>,
    },
    SetData {
        respond_to: oneshot::Sender<String>,
        message: String
    }
}

impl MyActor {
    fn new(receiver: mpsc::Receiver<ActorMessage>) -> Self {
        MyActor {
            receiver,
            next_id: 0,
            data: "Hello world!".to_string()
        }
    }

    fn handle_message(&mut self, msg: ActorMessage) {
        match msg {
            ActorMessage::GetUniqueId {respond_to} => {
                self.next_id += 1;
                let _ = respond_to.send(self.next_id.try_into().unwrap());
            }
            ActorMessage::GetData {respond_to} => {
                let _ = respond_to.send(self.data.clone());
            }
            ActorMessage::SetData {respond_to, message} => {
                self.data = message;
                let _ = respond_to.send(self.data.clone());
            }
        }
    }
}

async fn run_my_actor(mut actor: MyActor) {
    while let Some(msg) = actor.receiver.recv().await {
        actor.handle_message(msg);
    }
}

#[derive(Clone)]
pub struct MyActorHandle {
    sender: mpsc::Sender<ActorMessage>,
}

impl MyActorHandle {
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel(8);
        let actor = MyActor::new(receiver);
        tokio::spawn(run_my_actor(actor));

        Self { sender }
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
    
    pub async fn get_data(&self) -> String {
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

    pub async fn set_data(&self, message: String) -> String {
        let (send, recv) = oneshot::channel();
        let msg = ActorMessage::SetData {
            respond_to: send,
            message
        };

        let _ = self.sender.send(msg).await;
        recv.await.expect("Actor task has been killed")
    }
}

#[tokio::main]
async fn main() {
    let actor_handle = MyActorHandle::new();

    let meme = actor_handle.get_unique_id().await;

    println!("The unique id of this actor is: {}", meme);

    let data = actor_handle.get_data().await;

    println!("The message of this actor is: {data}");

    let new_data = actor_handle.set_data("Hehe!".to_string()).await;

    println!("The new message of this actor is: {new_data}");
}
