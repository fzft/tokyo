use crate::actor::{Actor, Message};

pub trait SinkPool<M> where  M: Message {}