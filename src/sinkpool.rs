use crate::process::Processor;

pub trait SinkPool<M> where Self: Processor<M> {}