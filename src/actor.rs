use std::marker::PhantomData;
use std::sync::Arc;

use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::oneshot;
pub use tokio::sync::oneshot::Sender as OneshotSender;
use uuid::Uuid;

use crate::engine::IEngine;
use crate::mailbox::Mailbox;
use crate::types::{State, StateVal};
use crate::uid::CustomUuid;

pub trait Actor<M>: Sized + Send
    where M: Message
{
    type Context: ActorContext<M>;
    type Id: 'static + Default;


    fn ctx(&self) -> Self::Context {
        Self::Context::new()
    }

    fn id(&self) -> Self::Id {
        Self::Id::default()
    }
}


pub trait Handler<M>
    where Self: Actor<M>,
          M: Message
{
    type Result: MessageResponse<Self, M>;

    fn handle(&mut self, msg: M, ctx: &mut Self::Context) -> Self::Result;

    fn spawn(self, actor: impl Actor<M>, e: impl IEngine) {}
}



pub trait Message: Send + Sync + Clone {
    type Result: 'static;
}

impl<M> Message for Arc<M>
    where M: Message + Send + Sync
{
    type Result = M::Result;
}

impl<M> Message for Box<M> where
    M: Message + Send + Sync
{
    type Result = M::Result;
}


pub trait MessageResponse<A, M>
    where M: Message, A: Actor<M>
{
    fn process(self, ctx: &mut A::Context, tx: Option<OneshotSender<M::Result>>);
}


pub trait ActorContext<M>: Sized
    where M: Message,
          Self: Actor<M>,
{
    // Add this new function
    fn new() -> Self;

    // current processor execution state
    fn state(&self) -> State;

    fn stop(&mut self);

    fn downstream<M1>(&self) -> Option<()> where M1: Message;

    fn id(&self) -> CustomUuid;
}


pub struct Context<M>
    where M: Message
{
    state: State,
    mb: Mailbox<M>,
    marker: PhantomData<M>,
    downstream: Option<Uuid>,
    uid: CustomUuid
}

impl<M> ActorContext<M> for Context<M>
    where M: Message,
          Self: Actor<M>,

{
    fn new() -> Self {
        Self {
            state: State::ActorState(StateVal::Created),
            mb: Mailbox::new(),
            marker: PhantomData,
            downstream: None,
            uid: CustomUuid::default()
        }
    }

    fn state(&self) -> State {
        todo!()
    }

    fn stop(&mut self) {
        todo!()
    }

    fn downstream<M1>(&self) -> Option<()> where M1: Message {
        todo!()
    }

    fn id(&self) -> CustomUuid {
        self.uid
    }
}


// pub struct Envelope<M> {
//     pub message: M,
//     pub sender: Option<ActorRef<M>>,
//     // Other metadata can be added here if needed.
// }
//
// impl<M> Envelope<M> {
//     pub fn new(message: M, sender: Option<ActorRef<M>>) -> Self {
//         Envelope { message, sender }
//     }
// }

pub struct Addr<M>
    where M: Message,
{
    tx: UnboundedSender<M>,
    terminate_tx: oneshot::Sender<()>,
}

impl<M> Addr<M>
    where M: Message
{
    pub fn new<A>(actor: A) -> Self where
        A: Actor<M>,
    {
        let (tx, mut rx) = unbounded_channel::<M>();
        let (terminate_tx, terminate_rx) = oneshot::channel::<()>();


        tokio::spawn(async move {
            loop {
                tokio::select! {
                    msg = rx.recv() => {
                        if let Some(msg) = msg {
                            // Process the message here using the actor instance.
                        } else {
                            // The channel has been closed, terminate the task.
                            break;
                        }
                    }
                    _ = terminate_rx => {
                        // Termination signal received, terminate the task.
                        break;
                    }
                }
            }
        });

        Addr { tx, terminate_tx }
    }

    pub fn send(&self, msg: M) {
        let _ = self.tx.send(msg);
    }

    pub fn terminate(self) {
        let _ = self.terminate_tx.send(());
    }
}

