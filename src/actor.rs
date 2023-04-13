use std::sync::Arc;

use tokio::sync::mpsc::{unbounded_channel, UnboundedSender};
use tokio::sync::oneshot;
pub use tokio::sync::oneshot::Sender as OneshotSender;

use crate::types::State;

pub trait Actor<M>: Sized + Send
    where
        M: Message,
{
    type Result: MessageResponse<Self, M>;
    type Context: ActorContext<M>;

    fn handle(&self, msg: M, ctx: Self::Context) -> Self::Result;

}


pub trait DownStream<M1, M2>
    where
        Self: Actor<M1, Result=M2>,
        M2: Message,
        M1: Message
{
    fn spawn(&mut self, p2: impl Actor<M2>);
}


pub trait Message: Send + Sync {
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


pub trait ActorContext<M>
    where Self: Actor<M>,
          M: Message
{
    // current processor execution state
    fn state(&self) -> State;

    fn stop(&mut self);

    fn downstream<M1, A: Actor<M1>>(&self) -> Option<A> where M1: Message;
}

pub struct Envelope<M> {
    pub message: M,
    pub sender: Option<ActorRef<M>>,
    // Other metadata can be added here if needed.
}

impl<M> Envelope<M> {
    pub fn new(message: M, sender: Option<ActorRef<M>>) -> Self {
        Envelope { message, sender }
    }
}

pub struct Addr<M>
    where M: Message,
{
    tx: UnboundedSender<M>,
    terminate_tx: oneshot::Sender<()>,
}

impl<M> Addr<M>
    where M: Message,
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
