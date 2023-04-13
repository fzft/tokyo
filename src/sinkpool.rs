use crate::actor::{Actor, Message};

pub trait SinkPool<M> where Self: Actor<M>, M: Message {}