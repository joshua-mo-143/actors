use uuid::Uuid;
use crate::actor::ActorHandle;
use tokio::time::{sleep, Duration};

pub struct Supervisor {
    pub id: Uuid,
    pub actors: Vec<ActorHandle>
}

impl Supervisor {
   pub fn new() -> Self {
        Self {
            id: Uuid::new_v4(),
            actors: Vec::new()
        }
    }

   pub fn add_actor(mut self, actor: ActorHandle) -> Self {
        self.actors.push(actor);

        self
    }
}

impl Default for Supervisor {
    fn default() -> Self {
        Self::new()
    }
}