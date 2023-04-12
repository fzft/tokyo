use std::sync::Arc;

pub use tokio::sync::oneshot::Sender as OneshotSender;

use crate::types::State;

pub trait Processor<M>: Sized
    where
        M: Message,
{
    type Result: MessageResponse<Self, M>;
    type Context: ProcessCtx<M>;

    fn process(&self, msg: M, ctx: Self::Context) -> Self::Result;
}


pub trait DownStream<M1, M2>
    where
        Self: Processor<M1, Result=M2>,
        M2: Message,
        M1: Message
{
    fn spawn(&mut self, p2: impl Processor<M2>);
}


pub trait Message {
    type Result: 'static;
}

impl<M> Message for Arc<M>
    where M: Message
{
    type Result = M::Result;
}


pub trait MessageResponse<P, M>
    where M: Message, P: Processor<M>
{
    fn process(self, ctx: &mut P::Context, tx: Option<OneshotSender<M::Result>>);
}


pub trait ProcessCtx<M>
    where Self: Processor<M>,
          M: Message
{
    // current processor execution state
    fn state(&self) -> State;

    fn stop(&mut self);

    // look up next processor
    fn downstream<M1>(&self) -> Option<Box<dyn Processor<M1, Result=(), Context=()>>> where M1: Message;
}