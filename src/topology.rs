use crate::engine::Engine;
use crate::mailbox::Mailbox;
use crate::actor::{Message, Actor};

// stand alone tp is only one processor
pub trait StandAloneTopology
    where
        Self: Engine
{
    fn spawn<M: Message>(&mut self, p: impl Actor<M>, ctx: Self::Context);
}


// chain tp is link list tp
// tp will validate the tp
pub trait ChainTopology where Self: Engine
{
    fn spawn<M: Message>(&mut self, p: impl Actor<M> + Mailbox<M>) -> Result<(), String>;
}


// DAG tp is more complex tp than chain
// tp will validate the tp, use strong connect to check whether cyclic
pub trait DAGTopology where Self: Engine {

    fn spawn<M: Message>(&mut self, p: impl Actor<M> + Mailbox<M>) -> Result<(), String>;
}

